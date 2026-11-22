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
use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2D;

/// Shared glitch-row machinery - [`Glitch`] and
/// [`super::TvGlitch`] differ only in how they warp `coords` up front
/// and how they dissolve at the end, so the displacement,
/// interference, grain, line pattern and channel shifts live here.
///
/// `noise` comes back to the caller for its own alpha dissolve.
pub(super) struct GlitchRow {
    /// The composited row color before any dissolve.
    pub color: Color<f64>,
    /// The row noise value (`0..1`) - Glitch uses it for the dissolve.
    pub noise: f64,
}

/// Shades one row of the glitch effect: the column position `x_pos`
/// gets displaced left by `noise² * displace`, the window is sampled
/// there, then interference lines, grain, a 4-px line pattern and
/// the green/blue channel offsets are composited on top.
#[allow(clippy::too_many_arguments)]
pub(super) fn glitch_row(
    window: BufferView<'_, Rgba>,
    coords: Vec2<()>,
    size_px: Vec2<()>,
    scale: f64,
    strength: f64,
    displace: f64,
    time: f64,
    seed: f64,
    tint: Color<f64>,
) -> GlitchRow {
    let y_pos = scale * size_px.y * (coords.y + seed * 10.0);
    // Large noise waves plus smaller detail waves.
    let mut noise = (Simplex2D.at(Vec2::<()>::new(time, y_pos * 0.002)) - 0.5).clamp(0.0, 1.0);
    noise += (Simplex2D.at(Vec2::<()>::new(time * 10.0, y_pos * 0.05)) - 0.5) * 0.15;
    // Row x displacement by squared noise.
    let x_pos = (coords.x - displace * noise * noise).clamp(0.0, 1.0);
    let mut color = window.sample_bilinear(Vec2::<()>::new(x_pos, coords.y));
    // Random interference lines.
    let interference_strength = noise * strength.min(1.0);
    let mix_rgb = |a: f64, b: f64, t: f64| crate::mix(a, b, t);
    color = Color::<f64>::new_rgba(
        mix_rgb(color.r, tint.r * Hash::<f64>::hash(&Vec2::<()>::new(y_pos * time, 0.0)), tint.a * interference_strength),
        mix_rgb(color.g, tint.g * Hash::<f64>::hash(&Vec2::<()>::new(y_pos * time, 0.5)), tint.a * interference_strength),
        mix_rgb(color.b, tint.b * Hash::<f64>::hash(&Vec2::<()>::new(y_pos * time, 1.0)), tint.a * interference_strength),
        color.a,
    );
    // Grainy noise overlay.
    let grain_noise = Simplex2D.at(Vec2::<()>::new(size_px.x * coords.x + time * 100.0, size_px.y * coords.y + time * 100.0));
    let grain_strength = 0.2 * strength.min(1.0);
    color = Color::<f64>::new_rgba(
        mix_rgb(color.r, tint.r * grain_noise, tint.a * grain_strength),
        mix_rgb(color.g, tint.g * grain_noise, tint.a * grain_strength),
        mix_rgb(color.b, tint.b * grain_noise, tint.a * grain_strength),
        color.a,
    );
    // Subtle darkening every 4th line.
    if (y_pos * 0.25 % 2.0).floor() == 0.0 {
        color = Color::<f64>::new_rgba(
            mix_rgb(color.r, tint.r, tint.a * 0.15 * noise),
            mix_rgb(color.g, tint.g, tint.a * 0.15 * noise),
            mix_rgb(color.b, tint.b, tint.a * 0.15 * noise),
            color.a,
        );
    }
    // Green/blue channel shifts.
    let offset = 0.1 * noise * displace;
    color.g = crate::mix(color.g, window.sample_bilinear(Vec2::<()>::new(x_pos + offset, coords.y)).g, 0.25);
    color.b = crate::mix(color.b, window.sample_bilinear(Vec2::<()>::new(x_pos - offset, coords.y)).b, 0.25);
    GlitchRow { color, noise }
}

/// Rows of the window shear sideways on simplex waves while
/// interference lines, grain and RGB splits degrade the signal -
/// then the window dissolves like a bad broadcast.
///
/// Behavioral spec (source shader: `glitch.frag`, re-implemented
/// from semantics): `progress` is `easeInQuad`-eased (inverted for
/// `Open`); `time = progress * duration * speed`, `strength =
/// strength * progress`, `displace = 1000 * strength / width`. Each
/// row's `yPos = scale * height * (uv.y + seed * 10)` feeds two
/// octaves of `Simplex2D`; `xPos` clamps leftward by `displace *
/// noise²`. Interference (`hash12` lines), grain (another octave),
/// a mod-4 line pattern and 0.25-weight green/blue channel offsets
/// composite over the displaced sample. Alpha dissolves via
/// `noise + 1 + fadeDelay - (3 + fadeDelay) * progress` with
/// `fadeDelay = 1.5`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Glitch {
    /// Interference/grain tint - schema `glitch-color`, default `rgb(100, 160, 255)`.
    #[serde(default = "Glitch::default_color")]
    pub color: Color<f64>,
    /// Row-frequency scale - schema `glitch-scale`, default `1.0`.
    #[serde(default = "Glitch::default_scale")]
    pub scale: ScaleFactor,
    /// Displacement strength - schema `glitch-strength`, default `2.0`.
    #[serde(default = "Glitch::default_strength")]
    pub strength: Factor,
    /// Noise wave speed - schema `glitch-speed`, default `2.0`.
    #[serde(default = "Glitch::default_speed")]
    pub speed: CycleSpeed,
}

impl Glitch {
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

impl Default for Glitch {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            scale: Self::default_scale(),
            strength: Self::default_strength(),
            speed: Self::default_speed(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Effect Color",
        kind: ParamKind::Color {
            default: Glitch::default_color(),
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

impl WindowEffect for Glitch {
    fn nick(&self) -> &'static str {
        "glitch"
    }

    fn label(&self) -> &'static str {
        "Glitch"
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
        let mismatch = |index: usize| FxError::ParamMismatch(format!("glitch param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("glitch takes 4 parameters, got {}", values.len())));
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
        let progress = EasingFunction::EaseIn
            .apply(Fraction::new(match ctx.direction {
                crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
                crate::Direction::Close => ctx.progress.as_f64(),
            }))
            .as_f64();
        // Frame constants.
        let time = progress * ctx.duration.as_secs_f64() * self.speed.0;
        let strength = self.strength.0 * progress;
        let displace = 1000.0 * strength / size_px.x;
        let seed = ctx.seed.as_f64();
        crate::render::fill(out, |x, y| {
            let coords = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let row = glitch_row(ctx.window, coords, size_px, self.scale.0, strength, displace, time, seed, self.color);
            // The dissolve rides the row noise.
            let alpha = (row.noise + 2.5 - 4.5 * progress).clamp(0.0, 1.0);
            let mut c = row.color;
            c.a *= alpha;
            c
        });
    }
}
