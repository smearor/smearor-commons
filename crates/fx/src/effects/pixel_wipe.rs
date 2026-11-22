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
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2DFractal;

/// [`Pixelate`](super::Pixelate)'s cell-dropout driven by a radial
/// wipe from the pointer position instead of uniform noise - the
/// dissolve spreads in a circle from where the user touched.
///
/// Behavioral spec (source shader: `pixel-wipe.frag`, re-implemented
/// from semantics): `circle = length(uv - start_pos)` feeds
/// `progress = smoothstep(((1 - easeOutQuad(p)) * (1 + FADE_WIDTH) -
/// 1 + circle) / FADE_WIDTH)`, inverted for `Close`; `start_pos` is
/// the pointer in uv space (`uStartPos`, set from the actor-local
/// pointer position in `PixelWipe.js` - `ctx.pointer`, window
/// center when absent). Pixelation and noise dropout then follow
/// `pixelate` (`simplex2DFractal(texcoord * size / 20) * 1.5 - 0.25`,
/// `* 10` collapse).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PixelWipe {
    /// Maximum pixel size - schema `pixel-wipe-pixel_size`, default `10`.
    #[serde(default = "PixelWipe::default_pixel_size")]
    pub pixel_size: Pixels<u32>,
}

impl PixelWipe {
    const fn default_pixel_size() -> Pixels<u32> {
        Pixels::new(10)
    }
}

impl Default for PixelWipe {
    fn default() -> Self {
        Self {
            pixel_size: Self::default_pixel_size(),
        }
    }
}

/// Width of the radial transition band - `FADE_WIDTH`.
const FADE_WIDTH: f64 = 1.0;

const PARAMS: &[ParamSpec] = &[ParamSpec {
    key: "pixel_size",
    label: "Pixel Size",
    kind: ParamKind::Int { min: 1, max: 50, default: 10 },
}];

impl WindowEffect for PixelWipe {
    fn nick(&self) -> &'static str {
        "pixel-wipe"
    }

    fn label(&self) -> &'static str {
        "Pixel Wipe"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Int(self.pixel_size.0 as i64)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match values {
            [ParamValue::Int(size)] => {
                self.pixel_size = Pixels::new((*size).max(1) as u32);
                Ok(())
            }
            _ => Err(FxError::ParamMismatch(format!("pixel-wipe takes 1 int parameter, got {}", values.len()))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        // The wipe origin - pointer uv or window center.
        let start = ctx.pointer.unwrap_or(Vec2::<()>::new(0.5, 0.5));
        let eased = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let opening = matches!(ctx.direction, crate::Direction::Open);
        // Frame constants - pixel grid uses the progress after the
        // radial transform, so it varies per pixel.
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let circle = ((uv.x - start.x).mul_add(uv.x - start.x, (uv.y - start.y) * (uv.y - start.y))).sqrt();
            let mut progress = crate::smoothstep(0.0, 1.0, ((1.0 - eased) * (1.0 + FADE_WIDTH) - 1.0 + circle) / FADE_WIDTH);
            if !opening {
                progress = 1.0 - progress;
            }
            let pixel_size = (self.pixel_size.0 as f64 * progress + 1.0).ceil().max(1.0);
            let grid = Vec2::<()>::new(pixel_size / size_px.x, pixel_size / size_px.y);
            let texcoord = Vec2::<()>::new(
                uv.x - crate::fract(uv.x / grid.x) * grid.x + grid.x * 0.5,
                uv.y - crate::fract(uv.y / grid.y) * grid.y + grid.y * 0.5,
            );
            let mut c = ctx.window.sample_bilinear(texcoord);
            let random = Simplex2DFractal.at(Vec2::<()>::new(texcoord.x * size_px.x / 20.0, texcoord.y * size_px.y / 20.0)) * 1.5 - 0.25;
            if progress > random {
                c.a *= (1.0 - (progress - random) * 10.0).max(0.0);
            }
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
    use smearor_commons_color::Color;
    use smearor_commons_geometry::Fraction;
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
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
        }
    }

    /// Without a pointer the wipe starts at the window center - early
    /// close dissolves the center before the corners.
    #[test]
    fn wipe_starts_at_the_pointer() {
        let window = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        PixelWipe::default().apply(&ctx_of(&window, 0.18, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let center = view.sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a;
        let corner = view.sample_bilinear(Vec2::<()>::new(0.03, 0.03)).a;
        assert!(center < corner, "center {center} dissolves before corner {corner}");
    }

    /// Full open is untouched, full close is gone.
    #[test]
    fn pixel_wipe_endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = PixelWipe::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }
}
