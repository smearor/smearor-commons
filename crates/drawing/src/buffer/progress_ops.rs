//! The [`DrawProgressOps`] implementations for the buffer targets.

use crate::ops::DrawProgressOps;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;

impl DrawProgressOps for BufferViewMut<'_, Rgba> {
    /// Draw a horizontal progress bar at the bottom of the image.
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) {
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
    }
}

/// An owned packed buffer draws indicators through its packed view.
impl DrawProgressOps for PixelBuffer<Rgba> {
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) {
        self.view_mut().draw_progress_bar(value, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_geometry::Size;

    #[test]
    fn draw_progress_bar_full_fill() {
        let size = Size::new(8, 8);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 0, 0);
        buf.view_mut().draw_progress_bar(Fraction::new(1.0), color);

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
        buf.view_mut().draw_progress_bar(Fraction::new(0.5), color);

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
        buf.view_mut().draw_progress_bar(Fraction::new(0.0), color);

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
        buf.view_mut().draw_progress_bar(Fraction::new(2.0), color);

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
        buf.view_mut().draw_progress_bar(Fraction::new(-1.0), color);

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
        buf.view_mut().draw_progress_bar(Fraction::new(1.0), color_f64);

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
