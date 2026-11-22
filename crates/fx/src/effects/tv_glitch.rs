use super::glitch::glitch_row;
use crate::CycleSpeed;
use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_animation::EasingFunction;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;

/// [`Tv`](super::Tv)'s CRT collapse overlaid with
/// [`Glitch`](super::Glitch)'s row displacement - the window glitches
/// while it squeezes onto the center line.
///
/// Behavioral spec (source shader: `tv-glitch.frag`, re-implemented
/// from semantics): the TV squeeze runs at *half* speed inside the
/// run - `tvProgress = clamp(p*2 - tOffset)` with `tOffset = 0` for
/// `Open` (TV first half) and `1` for `Close` (TV second half), then
/// `easeOutQuad`-eased and inverted for `Open`. Everything else -
/// vertical squash, tb/lr/ff masks, tint - follows `Tv`; the glitch
/// displacement, interference, grain, line pattern and channel
/// shifts follow `Glitch` and composite before the TV masks gate
/// alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TvGlitch {
    /// Interference/squeeze tint - schema `tv-glitch-color`, default `rgb(100, 160, 255)`.
    #[serde(default = "TvGlitch::default_color")]
    pub color: Color<f64>,
    /// Row-frequency scale - schema `tv-glitch-scale`, default `1.0`.
    #[serde(default = "TvGlitch::default_scale")]
    pub scale: ScaleFactor,
    /// Displacement strength - schema `tv-glitch-strength`, default `2.0`.
    #[serde(default = "TvGlitch::default_strength")]
    pub strength: Factor,
    /// Noise wave speed - schema `tv-glitch-speed`, default `2.0`.
    #[serde(default = "TvGlitch::default_speed")]
    pub speed: CycleSpeed,
}

impl TvGlitch {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(100.0 / 255.0, 160.0 / 255.0, 1.0, 1.0)
    }
    const fn default_scale() -> ScaleFactor {
        ScaleFactor::new(1.0)
    }
    const fn default_strength() -> Factor {
        Factor::new(2.0)
    }
    const fn default_speed() -> CycleSpeed {
        CycleSpeed::new(2.0)
    }
}

impl Default for TvGlitch {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            scale: Self::default_scale(),
            strength: Self::default_strength(),
            speed: Self::default_speed(),
        }
    }
}

/// Same staged masks as `tv.frag`.
const BLUR_WIDTH: f64 = 0.01;
const TB_TIME: f64 = 0.7;
const LR_TIME: f64 = 0.4;
const LR_DELAY: f64 = 0.6;
const FF_TIME: f64 = 0.1;
const SCALING: f64 = 0.5;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Effect Color",
        kind: ParamKind::Color {
            default: TvGlitch::default_color(),
        },
    },
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 10.0,
            step: 0.1,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "strength",
        label: "Strength",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 2.0,
        },
    },
    ParamSpec {
        key: "speed",
        label: "Speed",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 2.0,
        },
    },
];

impl WindowEffect for TvGlitch {
    fn nick(&self) -> &'static str {
        "tv-glitch"
    }

    fn label(&self) -> &'static str {
        "TV Glitch"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Color(self.color),
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.strength.0),
            ParamValue::Float(self.speed.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("tv-glitch param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("tv-glitch takes 4 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        self.color = values[0].as_color().ok_or(mismatch(0))?;
        self.scale = ScaleFactor::new(float(1)?);
        self.strength = Factor::new(float(2)?);
        self.speed = CycleSpeed::new(float(3)?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        // The TV squeeze occupies half the run: first half for Open,
        // second for Close.
        let offset = match ctx.direction {
            crate::Direction::Open => 0.0,
            crate::Direction::Close => 1.0,
        };
        let tv_linear = (ctx.progress.as_f64() * 2.0 - offset).clamp(0.0, 1.0);
        let tv_eased = EasingFunction::EaseOut.apply(Fraction::new(tv_linear)).as_f64();
        let tv_prog = match ctx.direction {
            crate::Direction::Open => 1.0 - tv_eased,
            crate::Direction::Close => tv_eased,
        };
        // Glitch timing runs on the full progress.
        let progress = EasingFunction::EaseIn
            .apply(Fraction::new(match ctx.direction {
                crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
                crate::Direction::Close => ctx.progress.as_f64(),
            }))
            .as_f64();
        // Frame constants.
        let scale = 1.0 / crate::mix(1.0, SCALING, tv_prog) - 1.0;
        let time = progress * ctx.duration.as_secs_f64() * self.speed.0;
        let strength = self.strength.0 * progress;
        let displace = 1000.0 * strength / size_px.x;
        let seed = ctx.seed.as_f64();
        let tb_prog = crate::smoothstep(0.0, 1.0, (tv_prog / TB_TIME).clamp(0.0, 1.0));
        let lr_prog = crate::smoothstep(0.0, 1.0, ((tv_prog - LR_DELAY) / LR_TIME).clamp(0.0, 1.0));
        let ff_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((tv_prog - 1.0 + FF_TIME) / FF_TIME).clamp(0.0, 1.0));
        let tint = self.color.a * crate::smoothstep(0.0, 1.0, tv_prog);
        crate::render::fill(out, |x, y| {
            let mut coords = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            coords.y = coords.y * (scale + 1.0) - scale * 0.5;
            let row = glitch_row(ctx.window, coords, size_px, self.scale.0, strength, displace, time, seed, self.color);
            // TV masks gate the glitched color.
            let tb = (coords.y * 2.0).min(2.0 - coords.y * 2.0);
            let lr = (coords.x * 2.0).min(2.0 - coords.x * 2.0);
            let tb_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((tb_prog - tb) / BLUR_WIDTH).clamp(0.0, 1.0));
            let lr_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((lr_prog - lr) / BLUR_WIDTH).clamp(0.0, 1.0));
            let mut c = Color::<f64>::new_rgba(
                crate::mix(row.color.r, self.color.r * row.color.a, tint),
                crate::mix(row.color.g, self.color.g * row.color.a, tint),
                crate::mix(row.color.b, self.color.b * row.color.a, tint),
                row.color.a,
            );
            c.a *= tb_mask * lr_mask * ff_mask;
            c
        });
    }
}
