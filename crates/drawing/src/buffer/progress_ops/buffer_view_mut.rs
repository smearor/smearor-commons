//! The [`DrawProgressOps`] implementations for the buffer targets.

use crate::buffer::raster::blend_pixel;
use crate::ops::DrawProgressOps;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;

impl DrawProgressOps for BufferViewMut<'_, Rgba> {
    /// Draw a horizontal progress bar at the bottom of the image.
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        let size = self.size();
        let stride = self.stride().as_usize();
        let pixels = self.data_mut();
        let bar_height = 4u32;
        let color: Color<u8> = color.into();
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let fill_width = ((size.x as f64 * value.clamped().as_f64()) as u32).min(size.x);
        let bytes: [u8; 4] = color.into();

        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = y as usize * stride + x as usize * Rgba::BYTES_PER_PIXEL;
                if x < fill_width {
                    pixels[idx..idx + Rgba::BYTES_PER_PIXEL].copy_from_slice(&bytes);
                } else {
                    pixels[idx..idx + Rgba::BYTES_PER_PIXEL].copy_from_slice(&[bytes[0] / 3, bytes[1] / 3, bytes[2] / 3, 255]);
                }
            }
        }
        Ok(())
    }

    /// Raster gauge: per-pixel `Annulus` membership plus `Arc` direction
    /// test. `RotationRad::new` measures from +x clockwise in screen
    /// coordinates (y-down), so `-PI/2` starts the sweep at 12 o'clock.
    fn draw_gauge(
        &mut self,
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    ) -> Result<(), Self::Error> {
        let (center, radius, half_width) = (center.to_f64(), f64::from(radius.as_raw()), f64::from(line_width.as_raw()) / 2.0);
        if half_width <= 0.0 || radius <= half_width {
            return Ok(());
        }
        let annulus = Annulus::<Pixels>::new(center, radius - half_width, radius + half_width);
        let arc = Arc::new(
            RotationRad::new(-std::f64::consts::FRAC_PI_2),
            RotationRad::new(value.clamped().as_f64() * std::f64::consts::TAU),
        );
        let (color, track): (Color<u8>, Color<u8>) = (color.into().into(), track.into().into());
        let size = self.size();
        let stride = self.stride().as_usize();
        let pixels = self.data_mut();
        let bounds = annulus.outer_circle().bounding_box().to_i32_ceil();
        let (x0, y0) = (bounds.min.x.max(0), bounds.min.y.max(0));
        let (x1, y1) = (bounds.max().x.min(size.x as i32), bounds.max().y.min(size.y as i32));

        for y in y0..y1 {
            for x in x0..x1 {
                let point = Vec2::<Pixels>::new(x as f64 + 0.5, y as f64 + 0.5);
                if annulus.contains(&point) {
                    let idx = y as usize * stride + x as usize * Rgba::BYTES_PER_PIXEL;
                    let fill = if arc.contains((point - center).direction()) { color } else { track };
                    blend_pixel(pixels, idx, fill, Alpha::default());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Size;

    #[test]
    fn draw_progress_bar_full_fill() {
        let size = Size::new(8, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 0, 0);
        buf.view_mut().draw_progress_bar(Fraction::new(1.0), color).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let bytes: [u8; 4] = color.into();
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                assert_eq!(&pixels[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "pixel at ({},{}) should be fully filled", x, y);
            }
        }
    }

    #[test]
    fn draw_progress_bar_half_fill() {
        let size = Size::new(8, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 0, 0);
        buf.view_mut().draw_progress_bar(Fraction::new(0.5), color).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let bytes: [u8; 4] = color.into();
        let fill_width = (size.x as f32 * 0.5) as u32;
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                if x < fill_width {
                    assert_eq!(&pixels[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "pixel at ({},{}) should be filled", x, y);
                } else {
                    assert_eq!(pixels[idx], bytes[0] / 3, "pixel at ({},{}) should be dimmed", x, y);
                    assert_eq!(pixels[idx + 3], 255, "pixel at ({},{}) should be opaque", x, y);
                }
            }
        }
    }

    #[test]
    fn draw_progress_bar_zero_fill() {
        let size = Size::new(4, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(100, 200, 50);
        buf.view_mut().draw_progress_bar(Fraction::new(0.0), color).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let bytes: [u8; 4] = color.into();
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                assert_eq!(pixels[idx], bytes[0] / 3, "pixel at ({},{}) should be dimmed", x, y);
                assert_eq!(pixels[idx + 3], 255, "pixel at ({},{}) should be opaque", x, y);
            }
        }
    }

    #[test]
    fn draw_progress_bar_clamps_value_above_one() {
        let size = Size::new(4, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        buf.view_mut().draw_progress_bar(Fraction::new(2.0), color).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let bytes: [u8; 4] = color.into();
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                assert_eq!(&pixels[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "pixel at ({},{}) should be fully filled (clamped)", x, y);
            }
        }
    }

    #[test]
    fn draw_progress_bar_clamps_negative_value() {
        let size = Size::new(4, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        buf.view_mut().draw_progress_bar(Fraction::new(-1.0), color).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                assert_eq!(pixels[idx + 3], 255, "pixel at ({},{}) should be opaque", x, y);
            }
        }
    }

    #[test]
    fn draw_progress_bar_accepts_color_f64() {
        let size = Size::new(4, 8);
        let mut buf = PixelBuffer::new(size);
        let color_f64 = Color::<f64>::new(1.0, 0.0, 0.0);
        buf.view_mut().draw_progress_bar(Fraction::new(1.0), color_f64).unwrap();

        let bar_height = 4u32;
        let bar_y = size.y.saturating_sub(bar_height + 2);
        let expected: [u8; 4] = Color::<u8>::from(color_f64).into();
        let pixels = buf.pixels();
        for y in bar_y..bar_y + bar_height {
            for x in 0..size.x {
                let idx = (y * size.x + x) as usize * Rgba::BYTES_PER_PIXEL;
                assert_eq!(
                    &pixels[idx..idx + Rgba::BYTES_PER_PIXEL],
                    &expected,
                    "pixel at ({},{}) should match f64-converted color",
                    x,
                    y
                );
            }
        }
    }
}

#[cfg(test)]
mod gauge_tests {
    use super::*;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Size;

    fn px(buf: &PixelBuffer<Rgba>, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * buf.size().x + x) as usize * 4;
        buf.pixels()[idx..idx + 4].try_into().unwrap()
    }

    #[test]
    fn draw_gauge_renders_ring() {
        let mut buf = PixelBuffer::new(Size::new(40, 40));
        buf.draw_gauge(
            Vec2::<Pixels, f32>::new(20.0, 20.0),
            Pixels::new(14.0f32),
            Pixels::new(6.0f32),
            Fraction::ONE,
            Color::<f64>::new(0.0, 1.0, 0.0),
            Color::<f64>::new(0.2, 0.2, 0.2),
        )
        .unwrap();

        // Full turn: every ring pixel is the value color - top, right, bottom, left.
        for (x, y) in [(20, 6), (34, 20), (20, 34), (6, 20)] {
            let [r, g, b, _] = px(&buf, x, y);
            assert!(g > 200 && r < 60 && b < 60, "ring pixel at ({x},{y}) should be green, got {r} {g} {b}");
        }
        assert_eq!(px(&buf, 20, 20), [0, 0, 0, 0], "center stays empty");
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0], "corner stays empty");
    }

    #[test]
    fn draw_gauge_half_turn_splits_arc_and_track() {
        let mut buf = PixelBuffer::new(Size::new(40, 40));
        buf.draw_gauge(
            Vec2::<Pixels, f32>::new(20.0, 20.0),
            Pixels::new(14.0f32),
            Pixels::new(6.0f32),
            Fraction::HALF,
            Color::<f64>::new(0.0, 1.0, 0.0),
            Color::<f64>::new(0.2, 0.2, 0.2),
        )
        .unwrap();

        // 12 o'clock and 3 o'clock are inside the first half-turn; 9 o'clock is track.
        let [r, g, _, _] = px(&buf, 20, 6);
        assert!(g > 200 && r < 60, "top should be value color");
        let [r, g, _, _] = px(&buf, 34, 20);
        assert!(g > 200 && r < 60, "right should be value color");
        let [r, g, b, _] = px(&buf, 6, 20);
        assert!(r < 80 && g < 80 && b < 80, "left should be dim track, got {r} {g} {b}");
        assert!(!matches!(px(&buf, 6, 20), [0, 0, 0, 0]), "left ring should be filled, not empty");
    }

    #[test]
    fn draw_gauge_clamps_out_of_range_values() {
        let mut buf = PixelBuffer::new(Size::new(40, 40));
        buf.draw_gauge(
            Vec2::<Pixels, f32>::new(20.0, 20.0),
            Pixels::new(14.0f32),
            Pixels::new(6.0f32),
            Fraction::new(2.5),
            Color::<f64>::new(0.0, 1.0, 0.0),
            Color::<f64>::new(0.2, 0.2, 0.2),
        )
        .unwrap();

        // value > 1 clamps to a full turn.
        let [r, g, _, _] = px(&buf, 6, 20);
        assert!(g > 200 && r < 60, "over-1 value should clamp to full turn");
    }
}
