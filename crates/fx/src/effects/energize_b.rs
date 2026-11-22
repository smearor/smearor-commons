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
use smearor_commons_noise::Simplex2D;
use smearor_commons_noise::Simplex2DFractal;
use smearor_commons_noise::Simplex3D;

/// A shower of particles sweeps top-to-bottom, trailing streak
/// lines, while glimmering atoms condense where the window
/// dissolved - Energize A's sibling with directional sweep instead
/// of a radial burst.
///
/// Behavioral spec (source shader: `energize-b.frag`, re-implemented
/// from semantics): `progress = easeOutQuad(uProgress)`. `getMasks`
/// returns four components - `shower` (a `SHOWER_WIDTH` band
/// sweeping down over `SHOWER_TIME`), `streak` (everything the
/// shower passed, over `STREAK_TIME`), `atom` (the relative-edge
/// mask minus the shower, streak-gated, fading as
/// `sqrt(1-fade²)`), `window` (`(1-fade)²`, inverted for `Open`).
/// Masks clip at a 50-px absolute edge fade and a top-down
/// `streakProgress` gradient. Particles: shower =
/// `simplex2D(uv·size/scale)^10` drifting up at `0.7p/SHOWER_TIME`,
/// streaks = `simplex2DFractal(uv·(0.05,0.001)·size/scale)*0.5`
/// drifting at `p/SHOWER_TIME`, atoms = `simplex3D(uv·0.2·size/
/// scale, p·duration)^5` drifting at `0.025p/SHOWER_TIME`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct EnergizeB {
    /// Particle/tint color - schema `energize-b-color`, default `rgb(160, 180, 255)`.
    #[serde(default = "EnergizeB::default_color")]
    pub color: Color<f64>,
    /// Noise scale - schema `energize-b-scale`, default `1.0`.
    #[serde(default = "EnergizeB::default_scale")]
    pub scale: ScaleFactor,
}

impl EnergizeB {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(160.0 / 255.0, 180.0 / 255.0, 1.0, 1.0)
    }
    const fn default_scale() -> ScaleFactor {
        ScaleFactor::new(1.0)
    }
}

impl Default for EnergizeB {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            scale: Self::default_scale(),
        }
    }
}

/// Shower sweep duration - `SHOWER_TIME`.
const SHOWER_TIME: f64 = 0.3;
/// Shower band width - `SHOWER_WIDTH`.
const SHOWER_WIDTH: f64 = 0.3;
/// Streak persistence - `STREAK_TIME`.
const STREAK_TIME: f64 = 0.6;
/// Edge fade in px - `EDGE_FADE`.
const EDGE_FADE: f64 = 50.0;

/// The four `getMasks` components for one pixel.
struct Masks {
    /// Leading shower band.
    shower: f64,
    /// Trailing streak region.
    streak: f64,
    /// Glimmering atom field.
    atom: f64,
    /// Remaining window opacity.
    window: f64,
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Effect Color",
        kind: ParamKind::Color {
            default: EnergizeB::default_color(),
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

impl WindowEffect for EnergizeB {
    fn nick(&self) -> &'static str {
        "energize-b"
    }

    fn label(&self) -> &'static str {
        "Energize B"
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
            _ => Err(FxError::ParamMismatch(format!("energize-b takes color + float, got {values:?}"))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let progress = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        // Frame constants (getMasks' time schedule).
        let shower_progress = progress / SHOWER_TIME;
        let streak_progress = ((progress - SHOWER_TIME) / STREAK_TIME).clamp(0.0, 1.0);
        let fade_progress = ((progress - SHOWER_TIME) / (1.0 - SHOWER_TIME)).clamp(0.0, 1.0);
        let atom_fade = (1.0 - fade_progress * fade_progress).max(0.0).sqrt();
        let window_base = (1.0 - fade_progress).powi(2);
        let window_mask = match ctx.direction {
            crate::Direction::Open => 1.0 - window_base,
            crate::Direction::Close => window_base,
        };
        let tint_t = 0.5 * window_mask + 0.5;
        let time = ctx.duration.as_secs_f64();
        let masks = |uv: Vec2<()>, px: Vec2<()>| -> Masks {
            let edge = crate::absolute_edge_mask(px, size_px, ctx.padding, Pixels::new(EDGE_FADE), 0.5, ctx.fullscreen);
            let mut shower = crate::smoothstep(1.0, 0.0, ((shower_progress - uv.y - SHOWER_WIDTH) / SHOWER_WIDTH).abs());
            let mut streak = if shower_progress - uv.y - SHOWER_WIDTH > 0.0 { 1.0 } else { 0.0 };
            let atom = (crate::relative_edge_mask(uv, 0.2) - shower).max(0.0) * streak * atom_fade;
            shower += 0.05 * streak;
            streak = streak.max(shower);
            streak *= edge;
            shower *= edge;
            let fade = crate::smoothstep(0.0, 1.0, 1.0 + uv.y - 2.0 * streak_progress);
            streak *= fade;
            shower *= fade;
            // The atom mask is already relative-edge bounded.
            Masks {
                shower,
                streak,
                atom,
                window: window_mask,
            }
        };
        crate::render::fill(out, |x, y| {
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let uv = Vec2::<()>::new(px.x / size_px.x, px.y / size_px.y);
            let m = masks(uv, px);
            let mut c = ctx.window.sample_bilinear(uv);
            c = Color::<f64>::new_rgba(
                crate::mix(self.color.r, c.r, tint_t),
                crate::mix(self.color.g, c.g, tint_t),
                crate::mix(self.color.b, c.b, tint_t),
                c.a * m.window,
            );
            // Shower band particles.
            let shower_uv = Vec2::<()>::new(
                uv.x * 0.02 * size_px.x / self.scale.0,
                (uv.y - 0.7 * progress / SHOWER_TIME) * 0.02 * size_px.y / self.scale.0,
            );
            let shower = Simplex2D.at(shower_uv).max(0.0).powi(10);
            // Trailing streak lines.
            let streak_uv = Vec2::<()>::new(uv.x * 0.05 * size_px.x / self.scale.0, (uv.y - progress / SHOWER_TIME) * 0.001 * size_px.y / self.scale.0);
            let streaks = Simplex2DFractal.at(streak_uv) * 0.5;
            // Glimmering atoms.
            let atom_uv = Vec2::<()>::new(
                uv.x * 0.2 * size_px.x / self.scale.0,
                (uv.y - 0.025 * progress / SHOWER_TIME) * 0.2 * size_px.y / self.scale.0,
            );
            let atoms = Simplex3D.at(Vec3::<()>::new(atom_uv.x, atom_uv.y, progress * time)).powi(5);
            let (pr, pg, pb) = (self.color.r, self.color.g, self.color.b);
            c.r += pr * shower * m.shower + pr * streaks * m.streak + pr * atoms * m.atom;
            c.g += pg * shower * m.shower + pg * streaks * m.streak + pg * atoms * m.atom;
            c.b += pb * shower * m.shower + pb * streaks * m.streak + pb * atoms * m.atom;
            c.a += shower * m.shower + streaks * m.streak + atoms * m.atom;
            c
        });
    }
}
