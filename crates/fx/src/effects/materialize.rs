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
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// A flat tinted panel grows into the window - like a slow computer
/// in a sci-fi movie materializing the UI in a few quantized steps,
/// then revealing the actual content underneath.
///
/// Behavioral spec (source shader: `materialize.frag`, re-implemented
/// from semantics): `prog` is the materialized amount (`uProgress`
/// for `Open`, `1 - uProgress` for `Close`). `steps = 0` gives smooth
/// growth (with an early `FADE_TIME` fade-in); `steps > 1` quantizes
/// `prog` to `floor(prog*steps)/(steps-1)`; `0 < steps <= 1` snaps.
/// `growProg = easeOutCubic(prog / GROW_TIME)` scales the window
/// around its center from `start_scale` to `1`; samples outside
/// `[0,1]` are transparent. While growing, rgb mixes toward `color`
/// by `color.a * tint` (`tint` fading from `GROW_TIME` to `1`) and
/// alpha ramps from `PANEL_OPACITY` to `1`. The schema's
/// `use-accent-color` toggle is compositor integration and has no
/// CPU counterpart - `color` is always used.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Materialize {
    /// Panel tint - schema `materialize-color`, default `rgba(130, 190, 60, 0.85)`.
    #[serde(default = "Materialize::default_color")]
    pub color: Color<f64>,
    /// Initial panel scale - schema `materialize-start_scale`, default `0.6`.
    #[serde(default = "Materialize::default_start_scale")]
    pub start_scale: Fraction,
    /// Quantized growth steps (`0` = smooth) - schema `materialize-steps`, default `4`.
    #[serde(default = "Materialize::default_steps")]
    pub steps: u32,
}

impl Materialize {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(130.0 / 255.0, 190.0 / 255.0, 60.0 / 255.0, 0.85)
    }
    const fn default_start_scale() -> Fraction {
        Fraction::new(0.6)
    }
    const fn default_steps() -> u32 {
        4
    }
}

impl Default for Materialize {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            start_scale: Self::default_start_scale(),
            steps: Self::default_steps(),
        }
    }
}

/// Relative time for scaling to full size - `GROW_TIME`.
const GROW_TIME: f64 = 0.5;
/// Smooth-mode initial fade - `FADE_TIME`.
const FADE_TIME: f64 = 0.15;
/// Initial panel opacity - `PANEL_OPACITY`.
const PANEL_OPACITY: f64 = 0.6;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Panel Color",
        kind: ParamKind::Color {
            default: Materialize::default_color(),
        },
    },
    ParamSpec {
        key: "start_scale",
        label: "Start Scale",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.6,
        },
    },
    ParamSpec {
        key: "steps",
        label: "Steps",
        kind: ParamKind::Int { min: 0, max: 10, default: 4 },
    },
];

impl WindowEffect for Materialize {
    fn nick(&self) -> &'static str {
        "materialize"
    }

    fn label(&self) -> &'static str {
        "Materialize"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Color(self.color),
            ParamValue::Float(self.start_scale.as_f64()),
            ParamValue::Int(self.steps as i64),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("materialize param {index}"));
        if values.len() != 3 {
            return Err(FxError::ParamMismatch(format!("materialize takes 3 parameters, got {}", values.len())));
        }
        self.color = values[0].as_color().ok_or(mismatch(0))?;
        self.start_scale = Fraction::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.steps = values[2].as_i64().ok_or(mismatch(2))?.max(0) as u32;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        let mut prog = match ctx.direction {
            crate::Direction::Open => ctx.progress.as_f64(),
            crate::Direction::Close => 1.0 - ctx.progress.as_f64(),
        };
        // Frame constants.
        let steps = self.steps as f64;
        let fade = if self.steps > 0 { 1.0 } else { crate::smoothstep(0.0, FADE_TIME, prog) };
        if steps > 1.0 {
            prog = (prog * steps).floor().min(steps - 1.0) / (steps - 1.0);
        } else if steps > 0.0 {
            prog = 1.0;
        }
        let grow_prog = EasingFunction::EaseOutCubic.apply(Fraction::new((prog / GROW_TIME).clamp(0.0, 1.0))).as_f64();
        let inv_scale = 1.0 / crate::mix(self.start_scale.as_f64(), 1.0, grow_prog).max(f64::EPSILON);
        let tint = self.color.a * (1.0 - crate::smoothstep(GROW_TIME, 1.0, prog));
        let alpha = fade * crate::mix(PANEL_OPACITY, 1.0, grow_prog);
        crate::render::fill(out, |x, y| {
            let coords = Vec2::<()>::new(((x as f64 + 0.5) / w - 0.5) * inv_scale + 0.5, ((y as f64 + 0.5) / h - 0.5) * inv_scale + 0.5);
            let mut c = ctx.window.sample_bilinear(coords);
            // Samples scaled in from outside the texture stay hidden.
            if !(0.0..=1.0).contains(&coords.x) || !(0.0..=1.0).contains(&coords.y) {
                c.a = 0.0;
            }
            Color::<f64>::new_rgba(
                crate::mix(c.r, self.color.r, tint),
                crate::mix(c.g, self.color.g, tint),
                crate::mix(c.b, self.color.b, tint),
                c.a * alpha,
            )
        });
    }
}
