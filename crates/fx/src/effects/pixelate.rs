use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2DFractal;

/// The window pixelates harder and harder while noise-selected
/// cells drop out - the classic "dissolve into pixels".
///
/// Behavioral spec (source shader: `pixelate.frag`, re-implemented
/// from semantics): `progress` inverts for `Open`. `pixelSize =
/// ceil(pixel_size * progress + 1)` snaps uv to a pixel grid
/// (sampled at cell centers). `random = simplex2DFractal(texcoord *
/// noise * size / 1000) * 1.5 - 0.25` masks cells; where
/// `progress > random` alpha collapses by `(progress - random) *
/// 20`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Pixelate {
    /// Cell-dropout noise frequency - schema `pixelate-noise`, default `40.0`.
    #[serde(default = "Pixelate::default_noise")]
    pub noise: Factor,
    /// Maximum pixel size - schema `pixelate-pixel_size`, default `10`.
    #[serde(default = "Pixelate::default_pixel_size")]
    pub pixel_size: Pixels<u32>,
}

impl Pixelate {
    const fn default_noise() -> Factor {
        Factor::new(40.0)
    }
    const fn default_pixel_size() -> Pixels<u32> {
        Pixels::new(10)
    }
}

impl Default for Pixelate {
    fn default() -> Self {
        Self {
            noise: Self::default_noise(),
            pixel_size: Self::default_pixel_size(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "noise",
        label: "Noise",
        kind: ParamKind::Float {
            min: 1.0,
            max: 100.0,
            step: 1.0,
            default: 40.0,
        },
    },
    ParamSpec {
        key: "pixel_size",
        label: "Pixel Size",
        kind: ParamKind::Int { min: 1, max: 50, default: 10 },
    },
];

impl WindowEffect for Pixelate {
    fn nick(&self) -> &'static str {
        "pixelate"
    }

    fn label(&self) -> &'static str {
        "Pixelate"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Float(self.noise.0), ParamValue::Int(self.pixel_size.0 as i64)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("pixelate param {index}"));
        if values.len() != 2 {
            return Err(FxError::ParamMismatch(format!("pixelate takes 2 parameters, got {}", values.len())));
        }
        self.noise = Factor::new(values[0].as_f64().ok_or(mismatch(0))?);
        self.pixel_size = Pixels::new(values[1].as_i64().ok_or(mismatch(1))?.max(1) as u32);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let progress = match ctx.direction {
            crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
            crate::Direction::Close => ctx.progress.as_f64(),
        };
        // Frame constants.
        let pixel_size = (self.pixel_size.0 as f64 * progress + 1.0).ceil().max(1.0);
        let grid = Vec2::<()>::new(pixel_size / size_px.x, pixel_size / size_px.y);
        let noise_scale = Vec2::<()>::new(self.noise.0 * size_px.x / 1000.0, self.noise.0 * size_px.y / 1000.0);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            // Snap to the pixel grid, sample at cell centers.
            let texcoord = Vec2::<()>::new(
                uv.x - crate::fract(uv.x / grid.x) * grid.x + grid.x * 0.5,
                uv.y - crate::fract(uv.y / grid.y) * grid.y + grid.y * 0.5,
            );
            let mut c = ctx.window.sample_bilinear(texcoord);
            let random = Simplex2DFractal.at(Vec2::<()>::new(texcoord.x * noise_scale.x, texcoord.y * noise_scale.y)) * 1.5 - 0.25;
            if progress > random {
                c.a *= (1.0 - (progress - random) * 20.0).max(0.0);
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

    /// Full open is untouched, full close is fully dissolved.
    #[test]
    fn pixelate_endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = Pixelate::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Early progress pixelates but keeps alpha - every grid cell of a
    /// solid window samples the same color.
    #[test]
    fn pixelate_snaps_to_the_grid() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        Pixelate::default().apply(&ctx_of(&window, 0.1, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let a = view.sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a;
        assert!(a > 0.5, "early close still shows {a}");
    }
}
