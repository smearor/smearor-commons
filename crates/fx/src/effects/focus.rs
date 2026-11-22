//! Focus - a heavy radial blur that resolves into focus while the
//! window fades in (Burn-My-Windows' `focus` effect).
//!
//! The blur follows BMW's `getBlurredInputColor` from `common.glsl`:
//! `BLUR_DIRECTIONS` rays around the pixel, `blur_quality` taps along
//! each ray at distances `{1/n, 2/n, ..., n/n} * blur_amount` -
//! [`BufferView::cone_offsets`] + [`BufferView::sample_cone`]
//! implement exactly this table (the GLSL `1 - s` ramp enumerates the
//! same distances, so the mean is identical). The offsets are
//! computed once per frame and shared by all pixels.
//!
//! Timing (`focus.frag`):
//! * `progl = uProgress` when opening, `1 - uProgress` when closing
//! * `blurAmount = mix(uBlurAmount, 0, easeInOutSine(progl))` px
//! * `alpha = easeInOutCubic(progl)`, multiplied into the blurred
//!   straight-alpha color afterwards
//!
//! At `progl = 1` the blur amount collapses to zero and the cone
//! degenerates to a plain bilinear tap - exact rest behavior.
//!
//! Params (`focus.ui` ranges): `blur_amount` 0..=100 px (default 50),
//! `blur_quality` 1..=10 taps per ray (default 3).

use crate::BlurQuality;
use crate::Direction;
use crate::FxContext;
use crate::Layers;
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
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// Rays of the BMW blur cone (`directions = 15.0` in
/// `getBlurredInputColor`).
const BLUR_DIRECTIONS: u32 = 15;

/// Focus - blur + fade, the simplest BMW transition.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Focus {
    /// `uBlurAmount` - the blur radius in pixels at animation start
    /// (`focus.ui`: 0..=100, default 50). Serializes as the bare
    /// pixel count (`"blur_amount": 50`).
    pub blur_amount: Pixels<u32>,
    /// `uBlurQuality` - taps per ray of the blur cone (`focus.ui`:
    /// 1..=10, default 3). Higher values smooth the blur at linear
    /// sampling cost.
    pub blur_quality: BlurQuality,
}

impl Default for Focus {
    fn default() -> Self {
        Self {
            blur_amount: Self::default_blur_amount(),
            blur_quality: Self::default_blur_quality(),
        }
    }
}

impl Focus {
    const fn default_blur_amount() -> Pixels<u32> {
        Pixels::new(50)
    }
    const fn default_blur_quality() -> BlurQuality {
        BlurQuality::new(3)
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "blur_amount",
        label: "Blur Amount",
        kind: ParamKind::Int { min: 0, max: 100, default: 50 },
    },
    ParamSpec {
        key: "blur_quality",
        label: "Blur Quality",
        kind: ParamKind::Int { min: 1, max: 10, default: 3 },
    },
];

impl WindowEffect for Focus {
    fn nick(&self) -> &'static str {
        "focus"
    }

    fn label(&self) -> &'static str {
        "Focus"
    }

    fn layers(&self) -> Layers {
        Layers::WindowOnly
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Int(i64::from(self.blur_amount.0)),
            ParamValue::Int(i64::from(self.blur_quality.as_u32())),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), crate::FxError> {
        let mismatch = |index: usize| crate::FxError::ParamMismatch(format!("focus param {index}"));
        if values.len() != 2 {
            return Err(crate::FxError::ParamMismatch(format!("focus takes 2 parameters, got {}", values.len())));
        }
        self.blur_amount = Pixels::new(u32::try_from(values[0].as_i64().ok_or(mismatch(0))?.clamp(0, 100)).map_err(|_| mismatch(0))?);
        self.blur_quality = BlurQuality::new(u32::try_from(values[1].as_i64().ok_or(mismatch(1))?.clamp(1, 10)).map_err(|_| mismatch(1))?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = out.size();
        let w = f64::from(size.x).max(1.0);
        let h = f64::from(size.y).max(1.0);
        let opening = ctx.direction == Direction::Open;
        let progl = if opening { ctx.progress.as_f64() } else { 1.0 - ctx.progress.as_f64() };

        let blur_px = crate::mix(f64::from(self.blur_amount.0), 0.0, EasingFunction::EaseInOutSine.apply(Fraction::new(progl)).as_f64());
        let alpha = EasingFunction::EaseInOutCubic.apply(Fraction::new(progl)).as_f64();

        // The cone table once for the whole frame; `sample_cone`
        // degenerates to a plain bilinear tap when the radius is zero.
        let radius = Vec2::<()>::new(blur_px / w, blur_px / h);
        let offsets = BufferView::<'_, Rgba>::cone_offsets(radius, BLUR_DIRECTIONS, self.blur_quality.as_u32().max(1));

        let inv = Vec2::<()>::new(1.0 / w, 1.0 / h);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((f64::from(x) + 0.5) * inv.x, (f64::from(y) + 0.5) * inv.y);
            let mut c = ctx.window.sample_cone(uv, &offsets);
            c.a *= alpha;
            c
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Layer;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_color::Color;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn solid(size: u32) -> PixelBuffer<Rgba> {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
        let mut view = buffer.view_mut();
        for y in 0..size {
            for x in 0..size {
                view.put_color(Pixels::new(x), Pixels::new(y), Color::<f64>::new_rgba(0.5, 0.5, 0.5, 1.0));
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
            window_rect: smearor_commons_geometry::Rect::from_xywh(4.0, 4.0, f64::from(size.x) - 8.0, f64::from(size.y) - 8.0),
        }
    }

    /// Open end = the window at rest, fully opaque and sharp.
    #[test]
    fn open_end_is_the_window() {
        let window = solid(48);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(48, 48));
        Focus::default().apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        let c = out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5));
        assert_eq!(c.a, 1.0);
        // 8-bit quantization: 0.5 -> 128/255
        assert!((c.r - 0.50196).abs() < 0.01);
    }

    /// Close end = fully transparent.
    #[test]
    fn close_end_is_empty() {
        let window = solid(48);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(48, 48));
        Focus::default().apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Mid animation the cone actually blurs: a hard edge's straight
    /// alpha is smeared into the transparent side.
    #[test]
    fn mid_animation_blurs_edges() {
        let mut window = PixelBuffer::<Rgba>::new(Size::new(96, 96));
        {
            let mut view = window.view_mut();
            for y in 0..96 {
                for x in 0..96 {
                    let c = if x < 48 {
                        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)
                    } else {
                        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 0.0)
                    };
                    view.put_color(Pixels::new(x), Pixels::new(y), c);
                }
            }
        }
        let mut out = PixelBuffer::<Rgba>::new(Size::new(96, 96));
        Focus::default().apply(&ctx_of(&window, 0.5, Direction::Open), &mut out.view_mut());
        let view = out.view();
        // the fade halves every alpha at progress 0.5
        // (easeInOutCubic(0.5) = 0.5)
        let deep = view.sample_bilinear(Vec2::<()>::new(0.0625, 0.5)).a;
        assert!(deep > 0.45 && deep < 0.55, "deep alpha {deep}");
        // blur radius 50px * (1-sine_eased(0.5)=0.5) = 25px smears
        // alpha into the transparent half (14px in): less than `deep`
        // but not fully dissolved
        let smeared = view.sample_bilinear(Vec2::<()>::new(0.65, 0.5)).a;
        assert!(smeared > 0.0 && smeared < deep, "smeared alpha {smeared} vs deep {deep}");
    }
}
