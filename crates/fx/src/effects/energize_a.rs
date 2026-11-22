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
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex3D;

/// An expanding ring of glowing "atom" particles swallows the window
/// - the content dissolves into the effect color while simplex
///   molecules condense around it, then a bright heart fades last.
///
/// Behavioral spec (source shader: `energize-a.frag`, re-implemented
/// from semantics): `progress = easeOutQuad(uProgress)` (only the
/// window mask reads the direction). `atomMask` expands from the
/// center over `FADE_IN_TIME`, dies over `FADE_OUT_TIME`, bounded by
/// a 50-px absolute edge fade; `heartMask` is the
/// `relativeEdgeMask(0.5)⁵ * 3` center flash over the last
/// `HEART_FADE_TIME`. The window fades by `(1 - fadeOut)²` (inverted
/// for `Open`) and tints toward `color`. Particles: one static
/// `simplex3D` octave cubed, plus three octaves of
/// `1/(1-atoms)-1`-sharped atoms scrolling up at `0.1 * progress *
/// duration`, all scaled by `scale` and masked by `atomMask`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct EnergizeA {
    /// Particle/tint color - schema `energize-a-color`, default `rgb(255, 200, 50)`.
    #[serde(default = "EnergizeA::default_color")]
    pub color: Color<f64>,
    /// Noise scale - schema `energize-a-scale`, default `1.0`.
    #[serde(default = "EnergizeA::default_scale")]
    pub scale: ScaleFactor,
}

impl EnergizeA {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(1.0, 200.0 / 255.0, 50.0 / 255.0, 1.0)
    }
    const fn default_scale() -> ScaleFactor {
        ScaleFactor::new(1.0)
    }
}

impl Default for EnergizeA {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            scale: Self::default_scale(),
        }
    }
}

/// Particle fade-in - `FADE_IN_TIME`.
const FADE_IN_TIME: f64 = 0.3;
/// Particle fade-out - `FADE_OUT_TIME`.
const FADE_OUT_TIME: f64 = 0.6;
/// Center-flash duration - `HEART_FADE_TIME`.
const HEART_FADE_TIME: f64 = 0.3;
/// Edge fade in px - `EDGE_FADE_WIDTH`.
const EDGE_FADE_WIDTH: f64 = 50.0;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Effect Color",
        kind: ParamKind::Color {
            default: EnergizeA::default_color(),
        },
    },
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 5.0,
            step: 0.1,
            default: 1.0,
        },
    },
];

impl WindowEffect for EnergizeA {
    fn nick(&self) -> &'static str {
        "energize-a"
    }

    fn label(&self) -> &'static str {
        "Energize A"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Color(self.color), ParamValue::Float(self.scale.0)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match values {
            [ParamValue::Color(color), ParamValue::Float(scale)] => {
                self.color = *color;
                self.scale = ScaleFactor::new(*scale);
                Ok(())
            }
            _ => Err(FxError::ParamMismatch(format!("energize-a takes color + float, got {values:?}"))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let progress = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        // Frame constants (getMasks' time schedule).
        let fade_in = (progress / FADE_IN_TIME).clamp(0.0, 1.0);
        let fade_out = ((progress - FADE_IN_TIME) / FADE_OUT_TIME).clamp(0.0, 1.0);
        let heart = ((progress - (1.0 - HEART_FADE_TIME)) / HEART_FADE_TIME).clamp(0.0, 1.0);
        let atom_grow = fade_in * 2.0 + 1.0;
        let atom_fade = fade_in * crate::smoothstep(1.0, 0.0, fade_out);
        let heart_glow = 3.0 * fade_out * (1.0 - heart);
        let window_base = (1.0 - fade_out).powi(2);
        let window_mask = match ctx.direction {
            crate::Direction::Open => 1.0 - window_base,
            crate::Direction::Close => window_base,
        };
        let tint_t = 0.2 * window_mask + 0.8;
        let time = ctx.duration.as_secs_f64();
        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv = Vec2::<()>::new(px.x / size_px.x, px.y / size_px.y);
            // Masks.
            let dist = ((uv.x - 0.5).mul_add(uv.x - 0.5, (uv.y - 0.5) * (uv.y - 0.5)).sqrt()) * 4.0;
            let mut atom_mask = crate::smoothstep(0.0, 1.0, atom_grow - dist) * atom_fade;
            atom_mask *= crate::absolute_edge_mask(px, size_px, ctx.padding, Pixels::new(EDGE_FADE_WIDTH), 0.5, ctx.fullscreen);
            let heart_mask = heart_glow * crate::relative_edge_mask(uv, 0.5).powi(5);
            atom_mask = (heart_mask + atom_mask).clamp(0.0, 1.0);
            // Window dissolves toward the tint.
            let mut c = ctx.window.sample_bilinear(uv);
            c = Color::<f64>::new_rgba(
                crate::mix(self.color.r, c.r, tint_t),
                crate::mix(self.color.g, c.g, tint_t),
                crate::mix(self.color.b, c.b, tint_t),
                c.a * window_mask,
            );
            // Atom particles.
            let mut scaled = Vec2::<()>::new((uv.x - 0.5) * (1.0 + 0.1 * progress), (uv.y - 0.5) * (1.0 + 0.1 * progress));
            scaled.x /= self.scale.0;
            scaled.y /= self.scale.0;
            let first = Vec2::<()>::new((scaled.x) * 0.010598 * 0.5 * size_px.x, (scaled.y + 0.1 * progress * time) * 0.010598 * size_px.y);
            let mut particles = 0.2 * Simplex3D.at(Vec3::<()>::new(first.x, first.y, 0.0)).powi(3);
            for i in 1..=3u32 {
                let f = i as f64;
                let auv = Vec2::<()>::new(scaled.x * 0.12154 / 1.5f64.powf(f) * size_px.x, scaled.y * 0.12154 / 1.5f64.powf(f) * size_px.y);
                let atoms = Simplex3D.at(Vec3::<()>::new(auv.x, auv.y, 2.0 * progress * time / f));
                particles += 0.5 * (0.2 * (1.0 / (1.0 - atoms).max(f64::EPSILON) - 1.0)).powi(2);
            }
            c.r += self.color.r * particles * atom_mask;
            c.g += self.color.g * particles * atom_mask;
            c.b += self.color.b * particles * atom_mask;
            c.a += particles * atom_mask;
            c
        });
    }
}
