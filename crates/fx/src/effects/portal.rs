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
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

/// A swirling portal opens, the window shrinks and travels through
/// it while it wobbles elastically - five composited layers: a
/// radial base gradient, two noise-band layers at different whirl
/// speeds, a glowing rim and sparkles.
///
/// Behavioral spec (source shader: `portal.frag`, re-implemented
/// from semantics): `getPortalScale` `easeOutBack`-grows the portal
/// over the first `PORTAL_OPEN_TIME / duration` and shrinks over
/// the last `PORTAL_CLOSE_TIME / duration`; `getPortalWobble`
/// `easeInBack`-pulses around the window-travel midpoint
/// (`WINDOW_OPEN_TIME`). Layer coords center at `(uv-0.5)*2*1.5`,
/// divided by `scale*0.5+0.5`. Layers: radial `darken` gradient
/// (alpha disc at `dist>1`), two whirled `simplex2D` bands at
/// `0.6`/`1.3` detail frequencies (`alphaOver`), a rim of
/// `mix(1,5,rand)*GLOW_EDGE_WIDTH*detailScale - 150*|dist-1|`, and
/// additive `lighten` sparkles at `noise*rand+0.9`^50. `rand =
/// dot(displace,displace)` from `getRandomDisplace(seed*3.7)`. The
/// window travels on `easeInBack(progress, 1.2)`: scales to
/// `WINDOW_SCALE`, squishes `WINDOW_SQUISH` vertically, tilts
/// `WINDOW_TILT`, fading late (`(1-p)*3`). Output is `alphaOver
/// (portal, window)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Portal {
    /// Portal tint - schema `portal-color`, default `rgb(121, 233, 24)`.
    #[serde(default = "Portal::default_color")]
    pub color: Color<f64>,
    /// Whirl rotation speed - schema `portal-rotation_speed`, default `0.3`.
    #[serde(default = "Portal::default_rotation_speed")]
    pub rotation_speed: Factor,
    /// Whirl warp amount - schema `portal-whirling`, default `1.0`.
    #[serde(default = "Portal::default_whirling")]
    pub whirling: Factor,
    /// Noise detail - schema `portal-details`, default `15.0`.
    #[serde(default = "Portal::default_details")]
    pub details: Factor,
}

impl Portal {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(121.0 / 255.0, 233.0 / 255.0, 24.0 / 255.0, 1.0)
    }
    const fn default_rotation_speed() -> Factor {
        Factor::new(0.3)
    }
    const fn default_whirling() -> Factor {
        Factor::new(1.0)
    }
    const fn default_details() -> Factor {
        Factor::new(15.0)
    }
}

impl Default for Portal {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            rotation_speed: Self::default_rotation_speed(),
            whirling: Self::default_whirling(),
            details: Self::default_details(),
        }
    }
}

/// Wobble envelope - `PORTAL_WOBBLE_TIME`/`STRENGTH`.
const PORTAL_WOBBLE_TIME: f64 = 0.8;
const PORTAL_WOBBLE_STRENGTH: f64 = 1.2;
/// Rim width - `GLOW_EDGE_WIDTH`.
const GLOW_EDGE_WIDTH: f64 = 5.0;
/// Window travel target scale - `WINDOW_SCALE`.
const WINDOW_SCALE: f64 = 0.3;
/// Window vertical squish - `WINDOW_SQUISH`.
const WINDOW_SQUISH: f64 = 1.0;
/// Window x-tilt - `WINDOW_TILT`.
const WINDOW_TILT: f64 = -1.0;
/// Portal open/close fractions of duration.
const PORTAL_OPEN_TIME: f64 = 0.4;
const PORTAL_CLOSE_TIME: f64 = 0.4;
/// Window travel fraction of duration.
const WINDOW_OPEN_TIME: f64 = 0.35;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Portal Color",
        kind: ParamKind::Color {
            default: Portal::default_color(),
        },
    },
    ParamSpec {
        key: "rotation_speed",
        label: "Rotation Speed",
        kind: ParamKind::Float {
            min: -2.0,
            max: 2.0,
            step: 0.05,
            default: 0.3,
        },
    },
    ParamSpec {
        key: "whirling",
        label: "Whirling",
        kind: ParamKind::Float {
            min: 0.0,
            max: 2.0,
            step: 0.05,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "details",
        label: "Details",
        kind: ParamKind::Float {
            min: 1.0,
            max: 20.0,
            step: 0.5,
            default: 15.0,
        },
    },
];

impl WindowEffect for Portal {
    fn nick(&self) -> &'static str {
        "portal"
    }

    fn label(&self) -> &'static str {
        "Portal"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Color(self.color),
            ParamValue::Float(self.rotation_speed.0),
            ParamValue::Float(self.whirling.0),
            ParamValue::Float(self.details.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("portal param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("portal takes 4 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        self.color = values[0].as_color().ok_or(mismatch(0))?;
        self.rotation_speed = Factor::new(float(1)?);
        self.whirling = Factor::new(float(2)?);
        self.details = Factor::new(float(3)?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let duration = ctx.duration.as_secs_f64();
        let u_progress = ctx.progress.as_f64();
        let seed = ctx.seed.offset_2d();
        // Frame constants.
        let close_time = PORTAL_CLOSE_TIME / duration.max(f64::EPSILON);
        let open_time = PORTAL_OPEN_TIME / duration.max(f64::EPSILON);
        let scale = if u_progress < open_time {
            EasingFunction::BackOut { overshoot_amount: 1.5 }
                .apply(Fraction::new((u_progress / open_time).clamp(0.0, 1.0)))
                .as_f64()
        } else if u_progress > 1.0 - close_time {
            EasingFunction::BackOut { overshoot_amount: 1.5 }
                .apply(Fraction::new((1.0 - (u_progress - 1.0 + close_time) / close_time).clamp(0.0, 1.0)))
                .as_f64()
        } else {
            1.0
        };
        let travel = match ctx.direction {
            crate::Direction::Open => (1.0 - u_progress) / WINDOW_OPEN_TIME * duration,
            crate::Direction::Close => u_progress / WINDOW_OPEN_TIME * duration,
        };
        let wobble_phase = (1.0 - ((travel - 1.0) / PORTAL_WOBBLE_TIME).abs()).clamp(0.0, 1.0);
        let wobble_amount = EasingFunction::BackIn { overshoot_amount: 1.7 }.apply(Fraction::new(wobble_phase)).as_f64() * PORTAL_WOBBLE_STRENGTH;
        let detail_scale = 10000.0 / (size_px.x + size_px.y) / self.details.0;
        let window_prog = EasingFunction::BackIn { overshoot_amount: 1.2 }
            .apply(Fraction::new(travel.clamp(0.0, 1.0)))
            .as_f64();
        let rotation_base = self.rotation_speed.0 * u_progress * duration;
        let warp_base = self.whirling.0 * (6.0 + 1.5 * u_progress);
        let portal_fade = scale.clamp(0.0, 1.0).powi(2);
        let window = ctx.window;
        let (w, h) = (size_px.x, size_px.y);
        let whirled = |coords: Vec2<()>, speed: f64, warp: f64| crate::whirl(coords, warp_base * warp, rotation_base * speed);
        let displace = |s: Vec2<()>, scale_f: f64| {
            Vec2::<()>::new(
                Simplex2D.at(Vec2::<()>::new(s.x * scale_f + seed.x, s.y * scale_f + seed.y)) - 0.5,
                Simplex2D.at(Vec2::<()>::new(s.x * scale_f + seed.x + 7.89, s.y * scale_f + seed.y + 123.0)) - 0.5,
            )
        };
        let portal_color = |uv: Vec2<()>| -> Color<f64> {
            let mut coords = Vec2::<()>::new((uv.x - 0.5) * 3.0, (uv.y - 0.5) * 3.0);
            let inv = 1.0 / (scale * 0.5 + 0.5).max(0.01);
            coords.x *= inv;
            coords.y *= inv;
            let dist = coords.x.mul_add(coords.x, coords.y * coords.y).sqrt();
            let wobble_scale = (1.0 - dist) * (-dist).exp() * wobble_amount;
            let wobble = Vec2::<()>::new(coords.x * wobble_scale, coords.y * wobble_scale);
            let alpha = if dist > 1.0 { 0.0 } else { 1.0 };
            // Layer 1: distorted radial gradient.
            let mut layer_coords = whirled(Vec2::<()>::new(coords.x - wobble.x, coords.y - wobble.y), 1.0, 1.0);
            let mut disp = displace(Vec2::<()>::new(layer_coords.x * 3.7, layer_coords.y * 3.7), 3.7);
            layer_coords = Vec2::<()>::new(layer_coords.x + disp.x * 0.1, layer_coords.y + disp.y * 0.1);
            let grad_t = layer_coords.x.mul_add(layer_coords.x, layer_coords.y * layer_coords.y).sqrt().powi(5);
            let rand = disp.x * disp.x + disp.y * disp.y;
            let mut color = Color::<f64>::new_rgba(
                crate::mix(crate::darken(self.color, 0.8).r, crate::darken(self.color, 0.2).r, grad_t),
                crate::mix(crate::darken(self.color, 0.8).g, crate::darken(self.color, 0.2).g, grad_t),
                crate::mix(crate::darken(self.color, 0.8).b, crate::darken(self.color, 0.2).b, grad_t),
                alpha,
            );
            // Layer 2: first whirled band.
            let noise = Simplex2D.at(Vec2::<()>::new(layer_coords.x / detail_scale + 12.3 + seed.x, layer_coords.y / detail_scale + 56.4 + seed.y));
            let darkened = crate::darken(self.color, 0.3);
            color = crate::alpha_over(color, Color::<f64>::new_rgba(darkened.r, darkened.g, darkened.b, if noise > 0.6 { alpha } else { 0.0 }));
            // Layer 3: faster, brighter band.
            let mut layer_coords = whirled(Vec2::<()>::new(coords.x - wobble.x * 1.5, coords.y - wobble.y * 1.5), 0.75, 0.5);
            disp = displace(layer_coords, 12.2);
            layer_coords = Vec2::<()>::new(layer_coords.x + disp.x * 0.1, layer_coords.y + disp.y * 0.1);
            let noise = Simplex2D.at(Vec2::<()>::new(layer_coords.x / detail_scale * 1.3 + seed.x, layer_coords.y / detail_scale * 1.3 + seed.y));
            color = crate::alpha_over(color, Color::<f64>::new_rgba(self.color.r, self.color.g, self.color.b, if noise > 0.6 { alpha } else { 0.0 }));
            color = clamped(color);
            // Layer 4: glowing rim.
            let edge = crate::mix(1.0, 5.0, rand) * GLOW_EDGE_WIDTH * detail_scale - 150.0 * (dist - 1.0).abs();
            color = crate::alpha_over(color, Color::<f64>::new_rgba(self.color.r, self.color.g, self.color.b, edge.clamp(0.0, 1.0)));
            // Layer 5: additive sparkles.
            let layer_coords = whirled(Vec2::<()>::new(coords.x - wobble.x * 1.8, coords.y - wobble.y * 1.8), 1.25, 0.0);
            let noise = Simplex2D.at(Vec2::<()>::new(layer_coords.x / detail_scale * 3.0 + seed.x, layer_coords.y / detail_scale * 3.0 + seed.y));
            let sparkle = (noise * rand + 0.9).powi(50).clamp(0.0, 1.0);
            let light = crate::lighten(self.color, 0.8);
            color.r += light.r * sparkle;
            color.g += light.g * sparkle;
            color.b += light.b * sparkle;
            color.a *= portal_fade;
            clamped(color)
        };
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            // Window traveling through the portal.
            let mut wc = Vec2::<()>::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
            let div = crate::mix(1.0, WINDOW_SCALE, window_prog).max(f64::EPSILON);
            wc.x /= div;
            wc.y /= div * crate::mix(1.0, 1.0 - 0.2 * WINDOW_SQUISH, window_prog).max(f64::EPSILON);
            wc.x /= crate::mix(1.0, 1.0 - 0.1 * WINDOW_TILT * wc.y, window_prog).max(f64::EPSILON);
            wc = Vec2::<()>::new(wc.x * 0.5 + 0.5, wc.y * 0.5 + 0.5);
            let mut window_color = window.sample_bilinear(wc);
            if !(0.0..=1.0).contains(&wc.x) || !(0.0..=1.0).contains(&wc.y) {
                window_color.a = 0.0;
            }
            window_color.a *= ((1.0 - window_prog) * 3.0).clamp(0.0, 1.0);
            crate::alpha_over(portal_color(uv), window_color)
        });
    }
}

/// GLSL `clamp(color, 0.0, 1.0)` per channel.
fn clamped(c: Color<f64>) -> Color<f64> {
    Color::<f64>::new_rgba(c.r.clamp(0.0, 1.0), c.g.clamp(0.0, 1.0), c.b.clamp(0.0, 1.0), c.a.clamp(0.0, 1.0))
}
