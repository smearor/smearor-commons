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
use smearor_commons_geometry::Vec2;

/// The window gets squashed horizontally onto a fading center line
/// like an old CRT switching off - top/bottom collapse first, then
/// left/right, then a final fade.
///
/// Behavioral spec (source shader: `tv.frag`, re-implemented from
/// semantics): `progress` is `easeOutQuad`-eased (`Close` collapses,
/// `Open` un-collapses). The window uv.y is scaled around the center
/// by `1/mix(1, SCALING, p)`. Three staged masks gate the output:
/// `tb` (top-center-bottom gradient, `TB_TIME`), `lr` (left-center-
/// right, delayed by `LR_DELAY`, `LR_TIME`) and `ff` (final fade,
/// last `FF_TIME`); each transitions over `BLUR_WIDTH` of its
/// gradient. While collapsing, rgb mixes toward `color * alpha`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Tv {
    /// Squeeze line tint - schema `tv-effect-color`, default `rgb(255, 255, 255)`.
    #[serde(default = "Tv::default_color")]
    pub color: Color<f64>,
}

impl Tv {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)
    }
}

impl Default for Tv {
    fn default() -> Self {
        Self { color: Self::default_color() }
    }
}

/// Width of the mask gradients - `BLUR_WIDTH`.
const BLUR_WIDTH: f64 = 0.01;
/// Relative time for the top/bottom collapse - `TB_TIME`.
const TB_TIME: f64 = 0.7;
/// Relative time for the left/right collapse - `LR_TIME`.
const LR_TIME: f64 = 0.4;
/// Delay before the left/right collapse starts - `LR_DELAY`.
const LR_DELAY: f64 = 0.6;
/// Final fade duration - `FF_TIME`.
const FF_TIME: f64 = 0.1;
/// Vertical squash target - `SCALING`.
const SCALING: f64 = 0.5;

const PARAMS: &[ParamSpec] = &[ParamSpec {
    key: "color",
    label: "Effect Color",
    kind: ParamKind::Color { default: Tv::default_color() },
}];

impl WindowEffect for Tv {
    fn nick(&self) -> &'static str {
        "tv"
    }

    fn label(&self) -> &'static str {
        "TV"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Color(self.color)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match values {
            [ParamValue::Color(color)] => {
                self.color = *color;
                Ok(())
            }
            _ => Err(FxError::ParamMismatch(format!("tv takes 1 color parameter, got {}", values.len()))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        let eased = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let prog = match ctx.direction {
            crate::Direction::Open => 1.0 - eased,
            crate::Direction::Close => eased,
        };
        // Frame constants.
        let scale = 1.0 / crate::mix(1.0, SCALING, prog) - 1.0;
        let tb_prog = crate::smoothstep(0.0, 1.0, (prog / TB_TIME).clamp(0.0, 1.0));
        let lr_prog = crate::smoothstep(0.0, 1.0, ((prog - LR_DELAY) / LR_TIME).clamp(0.0, 1.0));
        let ff_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((prog - 1.0 + FF_TIME) / FF_TIME).clamp(0.0, 1.0));
        let tint = self.color.a * crate::smoothstep(0.0, 1.0, prog);
        crate::render::fill(out, |x, y| {
            let mut coords = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            coords.y = coords.y * (scale + 1.0) - scale * 0.5;
            // Center-out gradients in [0..1..0].
            let tb = (coords.y * 2.0).min(2.0 - coords.y * 2.0);
            let lr = (coords.x * 2.0).min(2.0 - coords.x * 2.0);
            let tb_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((tb_prog - tb) / BLUR_WIDTH).clamp(0.0, 1.0));
            let lr_mask = 1.0 - crate::smoothstep(0.0, 1.0, ((lr_prog - lr) / BLUR_WIDTH).clamp(0.0, 1.0));
            let mask = tb_mask * lr_mask * ff_mask;
            let mut c = ctx.window.sample_bilinear(coords);

            c = Color::<f64>::new_rgba(
                crate::mix(c.r, self.color.r * c.a, tint),
                crate::mix(c.g, self.color.g * c.a, tint),
                crate::mix(c.b, self.color.b * c.a, tint),
                c.a * mask,
            );
            c
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

    fn solid(r: f64, g: f64, b: f64, a: f64, size: u32) -> PixelBuffer<Rgba> {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
        let mut view = buffer.view_mut();
        let c = Color::<f64>::new_rgba(r, g, b, a);
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
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
        }
    }

    /// At full open the solid window is untouched, at full close it
    /// has fully collapsed to transparency.
    #[test]
    fn tv_endpoints() {
        let window = solid(0.2, 0.4, 0.8, 1.0, 32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let tv = Tv::default();
        tv.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0, "fully open keeps alpha");
        tv.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0, "fully close fades out");
    }

    /// Mid-collapse the center line outlives the edges.
    #[test]
    fn tv_center_outlives_edges() {
        let window = solid(0.5, 0.5, 0.5, 1.0, 64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        Tv::default().apply(&ctx_of(&window, 0.4, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let center = view.sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a;
        let corner = view.sample_bilinear(Vec2::<()>::new(0.03, 0.03)).a;

        assert!(center > corner, "center {center} should outlive corner {corner}");
    }
}
