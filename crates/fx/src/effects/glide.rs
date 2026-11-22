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
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;

/// The window glides away scaled, squished, tilted and vertically
/// shifted while dissolving - a pure UV-transform resample.
///
/// Behavioral spec (source shader: `glide.frag`, re-implemented from
/// semantics): progress is `easeOutQuad`-eased *inside* the effect (it
/// does not arrive pre-eased); for `Open` the eased progress is
/// inverted so the window glides *into* place. The sample coordinate
/// is centered, divided by progress-blended `scale`/`squish`/`tilt`
/// factors, `shift`-ed and recentered. The dissolve is `progress`
/// when opening, `(1 - progress)²` when closing - a faster alpha
/// drop-off than the UV motion.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Glide {
    /// Image scale factor - schema `glide-scale`, default `0.95`.
    #[serde(default = "Glide::default_scale")]
    pub scale: ScaleFactor,
    /// Vertical squish amount - schema `glide-squish`, default `0.15`.
    #[serde(default = "Glide::default_squish")]
    pub squish: Factor,
    /// X-axis tilt proportional to y - schema `glide-tilt`, default `-0.3`.
    #[serde(default = "Glide::default_tilt")]
    pub tilt: Factor,
    /// Vertical shift - schema `glide-shift`, default `-0.05`.
    #[serde(default = "Glide::default_shift")]
    pub shift: Factor,
}

impl Glide {
    const fn default_scale() -> ScaleFactor {
        ScaleFactor::new(0.95)
    }
    const fn default_squish() -> Factor {
        Factor::new(0.15)
    }
    const fn default_tilt() -> Factor {
        Factor::new(-0.3)
    }
    const fn default_shift() -> Factor {
        Factor::new(-0.05)
    }
}

impl Default for Glide {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            squish: Self::default_squish(),
            tilt: Self::default_tilt(),
            shift: Self::default_shift(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.5,
            max: 1.5,
            step: 0.05,
            default: 0.95,
        },
    },
    ParamSpec {
        key: "squish",
        label: "Squish",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.15,
        },
    },
    ParamSpec {
        key: "tilt",
        label: "Tilt",
        kind: ParamKind::Float {
            min: -2.0,
            max: 2.0,
            step: 0.1,
            default: -0.3,
        },
    },
    ParamSpec {
        key: "shift",
        label: "Shift",
        kind: ParamKind::Float {
            min: -1.0,
            max: 1.0,
            step: 0.05,
            default: -0.05,
        },
    },
];

impl WindowEffect for Glide {
    fn nick(&self) -> &'static str {
        "glide"
    }

    fn label(&self) -> &'static str {
        "Glide"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.squish.0),
            ParamValue::Float(self.tilt.0),
            ParamValue::Float(self.shift.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("glide param {index}: expected Float"));
        self.scale = ScaleFactor(values.first().and_then(ParamValue::as_f64).ok_or(mismatch(0))?);
        self.squish = Factor(values.get(1).and_then(ParamValue::as_f64).ok_or(mismatch(1))?);
        self.tilt = Factor(values.get(2).and_then(ParamValue::as_f64).ok_or(mismatch(2))?);
        self.shift = Factor(values.get(3).and_then(ParamValue::as_f64).ok_or(mismatch(3))?);
        if values.len() > 4 {
            return Err(FxError::ParamMismatch(format!("glide takes 4 parameters, got {}", values.len())));
        }
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        // easeOutQuad inside the shader; inverted for opening.
        let eased = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let progress = match ctx.direction {
            crate::Direction::Open => 1.0 - eased,
            crate::Direction::Close => eased,
        };
        let alpha = match ctx.direction {
            crate::Direction::Open => ctx.progress.as_f64(),
            crate::Direction::Close => (1.0 - ctx.progress.as_f64()).powi(2),
        };
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        // Frame constants - the scale/squish denominators and the
        // shift step depend on progress alone.
        let scale = crate::mix(1.0, self.scale.0, progress);
        let squish = crate::mix(1.0, 1.0 - 0.2 * self.squish.0, progress);
        let tilt = 0.1 * self.tilt.0 * progress;
        let shift = self.shift.0 * progress;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let mut coords = uv * 2.0 - Vec2::<()>::new(1.0, 1.0);
            coords = coords / scale;
            coords.y /= squish;
            // mix(1, 1 - tilt_k*coords.y, progress) is linear:
            // 1 - tilt*progress*coords.y.
            coords.x /= 1.0 - tilt * coords.y;
            coords.y += shift;
            coords = coords * 0.5 + Vec2::<()>::new(0.5, 0.5);
            let mut c = ctx.window.sample_bilinear(coords);
            c.a *= alpha;
            c
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::BufferView;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn ctx_for<'a>(window: BufferView<'a, Rgba>, direction: Direction, progress: f64) -> FxContext<'a> {
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_millis(300),
            seed: Seed(1),
            direction,
            layer: Layer::Window,
            window,
            surface: None,
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 4.0, 4.0),
        }
    }

    fn uniform_window() -> Vec<u8> {
        [40, 80, 120, 255].repeat(16)
    }

    /// progress = 0 (close) or 1 (open): identity sample of the window;
    /// the other end is fully dissolved.
    #[test]
    fn glide_endpoints() {
        let data = uniform_window();
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(4, 4)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(4, 4));

        Glide::default().apply(&ctx_for(window, Direction::Close, 0.0), &mut out.view_mut());
        assert_eq!(out.view().data(), &data[..], "close@0: identity");

        Glide::default().apply(&ctx_for(window, Direction::Close, 1.0), &mut out.view_mut());
        assert!(out.view().data().iter().skip(3).step_by(4).all(|a| *a == 0), "close@1: dissolved");
    }

    /// The transform pivots around the window center: with `shift = 0`
    /// the exact center texel is stable under scale + squish + tilt
    /// (a 5x5 buffer has a true center texel).
    #[test]
    fn glide_center_stable() {
        let mut data = [40, 80, 120, 255].repeat(25);
        data[2 * 5 * 4 + 2 * 4..2 * 5 * 4 + 3 * 4].copy_from_slice(&[255, 0, 0, 255]); // (2,2) red
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(5, 5)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(5, 5));
        let ctx = FxContext {
            window_rect: Rect::from_xywh(0.0, 0.0, 5.0, 5.0),
            ..ctx_for(window, Direction::Close, 0.5)
        };
        let glide = Glide {
            shift: Factor::new(0.0),
            ..Default::default()
        };
        glide.apply(&ctx, &mut out.view_mut());
        let offset = 2 * 5 * 4 + 2 * 4;
        assert_eq!(out.view().data()[offset], 255, "center stays red at mid-progress");
    }
}
