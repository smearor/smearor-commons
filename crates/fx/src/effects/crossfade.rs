use crate::FxContext;
use crate::FxError;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Vec2;

/// Cross-fade between `window` and transparent - the canonical
/// minimal `WindowEffect` validating the whole skeleton end-to-end
/// (and genuinely useful as the plain fade-in/out).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Crossfade;

impl WindowEffect for Crossfade {
    fn nick(&self) -> &'static str {
        "crossfade"
    }

    fn label(&self) -> &'static str {
        "Crossfade"
    }

    fn params(&self) -> &'static [ParamSpec] {
        &[]
    }

    fn values(&self) -> Vec<ParamValue> {
        Vec::new()
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match values.is_empty() {
            true => Ok(()),
            false => Err(FxError::ParamMismatch(format!("crossfade takes no parameters, got {}", values.len()))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let weight = ctx.direction.weight(ctx.progress.0);
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let c = ctx.window.sample_bilinear(uv);
            c.with_alpha(c.a * weight)
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
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 2.0, 2.0),
        }
    }

    /// `progress = 0` writes the input on `Close` and zero on `Open`;
    /// `progress = 1` the opposite - the full apply path.
    #[test]
    fn crossfade_endpoints() {
        let data: &[u8] = &[10, 20, 30, 200, 10, 20, 30, 200, 10, 20, 30, 200, 10, 20, 30, 200];
        let window = BufferView::<Rgba>::from_packed(data, Size::new(2, 2)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(2, 2));

        Crossfade.apply(&ctx_for(window, Direction::Close, 0.0), &mut out.view_mut());
        assert_eq!(out.view().data(), data, "close@0: fully visible");

        Crossfade.apply(&ctx_for(window, Direction::Open, 0.0), &mut out.view_mut());
        assert_eq!(
            out.view().data(),
            &[10, 20, 30, 0, 10, 20, 30, 0, 10, 20, 30, 0, 10, 20, 30, 0],
            "open@0: straight-alpha fade keeps rgb, alpha 0"
        );

        Crossfade.apply(&ctx_for(window, Direction::Open, 1.0), &mut out.view_mut());
        assert_eq!(out.view().data(), data, "open@1: fully visible");
    }
}
