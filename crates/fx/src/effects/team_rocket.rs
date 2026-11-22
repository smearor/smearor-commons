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
use smearor_commons_geometry::Vec3;
use smearor_commons_noise::Hash;

/// The window gets sucked toward an off-center point while a rotating
/// white sparkle flares - Team Rocket blasting off again.
///
/// Behavioral spec (source shader: `team-rocket.frag`, re-implemented
/// from semantics): progress splits at `animation_split` - the first
/// half `easeOutQuad`-warp-scales and optionally random-rotates the
/// window uv (`uWindowRotation` -> seed-hashed xyz factors), the
/// second half `easeInOutSine`-drives a star sparkle (`getSpark`:
/// superellipse mask powered by `easeInExpo` brightness, faded by
/// `fadeInOut(t, 2)`) plus edge fades `fadeInOut(uv, 8)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TeamRocket {
    /// Where warp hands off to sparkle - schema `team-rocket-animation_split`, default `0.5`.
    #[serde(default = "TeamRocket::default_animation_split")]
    pub animation_split: Factor,
    /// Sparkle x offset - schema `team-rocket-horizontal_sparkle_position`, default `0.0`.
    #[serde(default)]
    pub horizontal_sparkle_position: Factor,
    /// Sparkle y offset - schema `team-rocket-vertical_sparkle_position`, default `0.5`.
    #[serde(default = "TeamRocket::default_vertical_sparkle_position")]
    pub vertical_sparkle_position: Factor,
    /// Sparkle rotation turns - schema `team-rocket-sparkle_rot`, default `0.33`.
    #[serde(default = "TeamRocket::default_sparkle_rot")]
    pub sparkle_rot: Factor,
    /// Sparkle size - schema `team-rocket-sparkle_size`, default `0.75`.
    #[serde(default = "TeamRocket::default_sparkle_size")]
    pub sparkle_size: Factor,
    /// Random xyz window warp - schema `team-rocket-window_rotation`, default `true`.
    #[serde(default = "TeamRocket::default_window_rotation")]
    pub window_rotation: bool,
}

impl TeamRocket {
    const fn default_animation_split() -> Factor {
        Factor::new(0.5)
    }
    const fn default_vertical_sparkle_position() -> Factor {
        Factor::new(0.5)
    }
    const fn default_sparkle_rot() -> Factor {
        Factor::new(0.33)
    }
    const fn default_sparkle_size() -> Factor {
        Factor::new(0.75)
    }
    const fn default_window_rotation() -> bool {
        true
    }
}

impl Default for TeamRocket {
    fn default() -> Self {
        Self {
            animation_split: Self::default_animation_split(),
            horizontal_sparkle_position: Factor::new(0.0),
            vertical_sparkle_position: Self::default_vertical_sparkle_position(),
            sparkle_rot: Self::default_sparkle_rot(),
            sparkle_size: Self::default_sparkle_size(),
            window_rotation: Self::default_window_rotation(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "animation_split",
        label: "Animation Split",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.5,
        },
    },
    ParamSpec {
        key: "horizontal_sparkle_position",
        label: "Sparkle X",
        kind: ParamKind::Float {
            min: -1.0,
            max: 1.0,
            step: 0.05,
            default: 0.0,
        },
    },
    ParamSpec {
        key: "vertical_sparkle_position",
        label: "Sparkle Y",
        kind: ParamKind::Float {
            min: -1.0,
            max: 1.0,
            step: 0.05,
            default: 0.5,
        },
    },
    ParamSpec {
        key: "sparkle_rot",
        label: "Sparkle Rotation",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.33,
        },
    },
    ParamSpec {
        key: "sparkle_size",
        label: "Sparkle Size",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.75,
        },
    },
    ParamSpec {
        key: "window_rotation",
        label: "Window Rotation",
        kind: ParamKind::Toggle { default: true },
    },
];

const TAU: f64 = std::f64::consts::TAU;

/// `fadeInOut` - `1 - ((t-0.5)/0.5)^power` clamped, a plateau easing.
fn fade_in_out(t: f64, power: f64) -> f64 {
    (1.0 - ((t - 0.5) / 0.5).powf(power)).clamp(0.0, 1.0)
}

/// `getSpark` - rotating superellipse star mask.
fn spark(uv: Vec2<()>, center: Vec2<()>, brightness: f64, size: f64, rotation: f64) -> f64 {
    let brightness = brightness.clamp(0.001, 1.0);
    let size = size.clamp(0.001, 1.0);
    let bn = crate::mix(0.0, 0.07, brightness);
    let mut p = Vec2::<()>::new(uv.x + 0.5 - center.x, uv.y + 0.5 - center.y);
    // scaleUV(uv, vec2(1.0 - size), vec2(0.0)): uv*2-1, / (1-size), *0.5+0.5.
    // scaleUV divisor = mix(1, 0, 1-size) = size.
    p = Vec2::<()>::new(((p.x * 2.0 - 1.0) / size) * 0.5 + 0.5, ((p.y * 2.0 - 1.0) / size) * 0.5 + 0.5);
    // rotate around (0.5, 0.5).
    let (sin, cos) = rotation.sin_cos();
    p = Vec2::<()>::new(cos * (p.x - 0.5) + sin * (p.y - 0.5) + 0.5, cos * (p.y - 0.5) - sin * (p.x - 0.5) + 0.5);
    let power = crate::mix(-1.0, 1000.0, EasingFunction::EaseInExpo.apply(Fraction::new(bn)).as_f64());
    let m = (((p.x - 0.5).abs() * 2.0).powf(power) + ((p.y - 0.5).abs() * 2.0).powf(power)).clamp(0.0, 1.0);
    (EasingFunction::EaseInSine.apply(Fraction::new(1.0 - (m - bn))).as_f64() - 0.004).clamp(0.0, 1.0)
}

impl WindowEffect for TeamRocket {
    fn nick(&self) -> &'static str {
        "team-rocket"
    }

    fn label(&self) -> &'static str {
        "Team Rocket"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.animation_split.0),
            ParamValue::Float(self.horizontal_sparkle_position.0),
            ParamValue::Float(self.vertical_sparkle_position.0),
            ParamValue::Float(self.sparkle_rot.0),
            ParamValue::Float(self.sparkle_size.0),
            ParamValue::Bool(self.window_rotation),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("team-rocket param {index}"));
        if values.len() != 6 {
            return Err(FxError::ParamMismatch(format!("team-rocket takes 6 parameters, got {}", values.len())));
        }
        self.animation_split = Factor::new(values[0].as_f64().ok_or(mismatch(0))?);
        self.horizontal_sparkle_position = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.vertical_sparkle_position = Factor::new(values[2].as_f64().ok_or(mismatch(2))?);
        self.sparkle_rot = Factor::new(values[3].as_f64().ok_or(mismatch(3))?);
        self.sparkle_size = Factor::new(values[4].as_f64().ok_or(mismatch(4))?);
        self.window_rotation = values[5].as_bool().ok_or(mismatch(5))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let aspect = size_px.x / size_px.y;
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let progress = ctx.progress.as_f64();
        let split = self.animation_split.0;
        // Random xyz warp factors (zero when rotation is off).
        // hash32(uSeed) scaled to [-0.1, 0.1] xyz warp factors.
        let rot3 = Hash::<Vec3<()>>::hash(&ctx.seed.as_f64());
        let rot = if self.window_rotation {
            Vec3::<()>::new((rot3.x * 2.0 - 1.0) * 0.1, (rot3.y * 2.0 - 1.0) * 0.1, (rot3.z * 2.0 - 1.0) * 0.1)
        } else {
            Vec3::<()>::new(0.0, 0.0, 0.0)
        };
        let sparkle_xy = Vec2::<()>::new(self.horizontal_sparkle_position.0, self.vertical_sparkle_position.0);
        let sparkle_rot = self.sparkle_rot.0;
        let sparkle_size = self.sparkle_size.0;
        let window = ctx.window;
        crate::render::fill(out, |x, y| {
            let uv0 = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let progress = if opening { 1.0 - progress } else { progress };
            let scalep = EasingFunction::EaseOut
                .apply(Fraction::new(crate::remap(progress, 0.0, split + 0.1, 0.0, 1.0)))
                .as_f64();
            let sparkp = EasingFunction::EaseInOutSine
                .apply(Fraction::new(crate::remap(progress, split - 0.1, 1.0, 0.0, 1.0)))
                .as_f64();
            // Window warp: scale toward the sparkle point, rotate.
            let scale_offset = Vec2::<()>::new(sparkle_xy.x * 0.5 * scalep, -sparkle_xy.y * 0.5 * scalep);
            let mut uv2 = Vec2::<()>::new(uv0.x * 2.0 - 1.0 - scale_offset.x / aspect * scalep, uv0.y * 2.0 - 1.0 - scale_offset.y * scalep);
            uv2 = Vec2::<()>::new(uv2.x / (1.0 - scalep), uv2.y / (1.0 - scalep));
            uv2 = Vec2::<()>::new(uv2.x / crate::mix(1.0, rot.x * TAU * uv2.y, scalep), uv2.y / crate::mix(1.0, rot.y * TAU * uv2.x, scalep));
            let (sin, cos) = (rot.z * TAU * scalep).sin_cos();
            uv2 = Vec2::<()>::new(uv2.x * cos - uv2.y * sin, uv2.x * sin + uv2.y * cos);
            uv2 = Vec2::<()>::new(uv2.x * 0.5 + 0.5, uv2.y * 0.5 + 0.5);
            // The sparkle.
            let uv = Vec2::<()>::new(uv0.x * aspect, uv0.y);
            let offset = Vec2::<()>::new(sparkle_xy.x * 0.2, sparkle_xy.y * -0.2);
            let fade = fade_in_out(sparkp, 2.0);
            let mut sparkle = spark(
                uv,
                Vec2::<()>::new(offset.x + 0.5 * aspect, offset.y + 0.5),
                fade * sparkle_size,
                fade * crate::mix(25.0, 50.0, sparkle_size),
                sparkp * TAU * sparkle_rot,
            );
            sparkle *= fade_in_out(uv0.y, 8.0) * fade_in_out(uv0.x, 8.0);
            let c = window.sample_bilinear(uv2);
            crate::alpha_over(c, Color::<f64>::new_rgba(1.0, 1.0, 1.0, sparkle))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn solid(size: u32) -> PixelBuffer<Rgba> {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
        let mut view = buffer.view_mut();
        let c = Color::<f64>::new_rgba(0.5, 0.5, 0.5, 1.0);
        for y in 0..size {
            for x in 0..size {
                view.put_color(Pixels::new(x), Pixels::new(y), c);
            }
        }
        buffer
    }

    fn ctx_of<'a>(window: &'a PixelBuffer<Rgba>, progress: f64, direction: Direction) -> FxContext<'a> {
        let size = window.size();
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_secs_f64(2.0),
            seed: Seed(0xABCD_EF01_2345_6789),
            direction,
            layer: Layer::Window,
            window: window.view(),
            surface: None,
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
        }
    }

    /// Open end untouched, close end warped away + sparkle faded.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = TeamRocket::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a < 0.5);
    }

    /// Late in the close the sparkle flares - some bright pixels appear.
    #[test]
    fn sparkle_flares_late() {
        let window = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        TeamRocket::default().apply(&ctx_of(&window, 0.6, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let any = (0..64).any(|y| (0..64).any(|x| view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 64.0, (y as f64 + 0.5) / 64.0)).a > 0.1));
        assert!(any, "sparkle should paint something");
    }
}
