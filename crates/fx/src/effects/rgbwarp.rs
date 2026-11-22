use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// The window peels upward with chromatic separation - a wave
/// travels top to bottom and the three color channels lag behind at
/// different speeds, splitting the image into RGB streaks while a
/// brightness pulse crests mid-run.
///
/// Behavioral spec (source shader: `rgbwarp.frag`, re-implemented
/// from semantics): `progress` is raw (`Close`) or inverted (`Open`).
/// `minSpeed = min(speedR, speedG, speedB)` sets `waveTime =
/// mix(0.1, 0.9, minSpeed)` and `waveProgress = progress /
/// waveTime`. Per row `offset = max(waveProgress - uv.y, 0)`,
/// normalized by `/((1/waveTime) - 1)` then `* (1 - waveTime)`; each
/// channel is sampled `offset * (speed - minSpeed + 1)` further down.
/// Output rgb comes from the three displaced samples, alpha is their
/// mean; rgb is scaled by `mix(1, brightness, fadeInOut(progress,
/// 4))` where `fadeInOut = clamp(1 - |2p-1|^4, 0, 1)`. A 30-px
/// absolute edge mask (`offset = 1`) fades the frame.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct RgbWarp {
    /// Mid-run brightness pulse - schema `rgbwarp-brightness`, default `1.0`.
    #[serde(default = "RgbWarp::default_brightness")]
    pub brightness: Factor,
    /// Red channel wave speed - schema `rgbwarp-speed_r`, default `1.0`.
    #[serde(default = "RgbWarp::default_speed_r")]
    pub speed_r: Factor,
    /// Green channel wave speed - schema `rgbwarp-speed_g`, default `0.90`.
    #[serde(default = "RgbWarp::default_speed_g")]
    pub speed_g: Factor,
    /// Blue channel wave speed - schema `rgbwarp-speed_b`, default `0.80`.
    #[serde(default = "RgbWarp::default_speed_b")]
    pub speed_b: Factor,
}

impl RgbWarp {
    const fn default_brightness() -> Factor {
        Factor::new(1.0)
    }
    const fn default_speed_r() -> Factor {
        Factor::new(1.0)
    }
    const fn default_speed_g() -> Factor {
        Factor::new(0.90)
    }
    const fn default_speed_b() -> Factor {
        Factor::new(0.80)
    }
}

impl Default for RgbWarp {
    fn default() -> Self {
        Self {
            brightness: Self::default_brightness(),
            speed_r: Self::default_speed_r(),
            speed_g: Self::default_speed_g(),
            speed_b: Self::default_speed_b(),
        }
    }
}

/// Symmetric in/out envelope - GLSL `FadeInOut(t, power) =
/// clamp(1 - ((t-0.5)/0.5)^power, 0, 1)`; `|2t-1|^power` keeps the
/// semantics defined for negative bases (GLSL `pow` is undefined
/// there but BMW's drivers abs() it).
fn fade_in_out(t: f64, power: f64) -> f64 {
    (1.0 - (2.0 * t - 1.0).abs().powf(power)).clamp(0.0, 1.0)
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "brightness",
        label: "Brightness",
        kind: ParamKind::Float {
            min: 0.0,
            max: 5.0,
            step: 0.1,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "speed_r",
        label: "Red Speed",
        kind: ParamKind::Float {
            min: 0.0,
            max: 2.0,
            step: 0.05,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "speed_g",
        label: "Green Speed",
        kind: ParamKind::Float {
            min: 0.0,
            max: 2.0,
            step: 0.05,
            default: 0.90,
        },
    },
    ParamSpec {
        key: "speed_b",
        label: "Blue Speed",
        kind: ParamKind::Float {
            min: 0.0,
            max: 2.0,
            step: 0.05,
            default: 0.80,
        },
    },
];

impl WindowEffect for RgbWarp {
    fn nick(&self) -> &'static str {
        "rgbwarp"
    }

    fn label(&self) -> &'static str {
        "RGB Warp"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.brightness.0),
            ParamValue::Float(self.speed_r.0),
            ParamValue::Float(self.speed_g.0),
            ParamValue::Float(self.speed_b.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("rgbwarp param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("rgbwarp takes 4 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        self.brightness = Factor::new(float(0)?);
        self.speed_r = Factor::new(float(1)?);
        self.speed_g = Factor::new(float(2)?);
        self.speed_b = Factor::new(float(3)?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let progress = match ctx.direction {
            crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
            crate::Direction::Close => ctx.progress.as_f64(),
        };
        // Frame constants.
        let min_speed = self.speed_r.0.min(self.speed_g.0.min(self.speed_b.0));
        // Clamped below 1.0 - at waveTime = 1 the offset normalization divides by zero.
        let wave_time = crate::mix(0.1, 0.9, min_speed).min(0.95);
        let wave_progress = progress / wave_time;
        let offset_scale = (1.0 - wave_time) / (1.0 / wave_time - 1.0);
        let brighten = crate::mix(1.0, self.brightness.0, fade_in_out(progress, 4.0));
        let (dr, dg, db) = (self.speed_r.0 - min_speed + 1.0, self.speed_g.0 - min_speed + 1.0, self.speed_b.0 - min_speed + 1.0);
        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv = Vec2::<()>::new(px.x / size_px.x, px.y / size_px.y);
            let offset = (wave_progress - uv.y).max(0.0) * offset_scale;
            let sample = |dy: f64| ctx.window.sample_bilinear(Vec2::<()>::new(uv.x, uv.y + offset * dy));
            let (cr, cg, cb) = (sample(dr), sample(dg), sample(db));
            Color::<f64>::new_rgba(
                cr.r * brighten,
                cg.g * brighten,
                cb.b * brighten,
                (cr.a + cg.a + cb.a) / 3.0 * crate::absolute_edge_mask(px, size_px, ctx.padding, Pixels::new(30.0), 1.0, ctx.fullscreen),
            )
        });
    }
}
