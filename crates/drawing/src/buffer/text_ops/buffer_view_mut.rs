//! The [`DrawTextOps`] implementations for the buffer targets.

use crate::buffer::raster::RasterOps;
use crate::ops::DrawTarget;
use crate::ops::DrawTextOps;
use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use ab_glyph::PxScale;
use ab_glyph::ScaleFont;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_font::truncate_text_to_width;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use tracing::debug;

impl DrawTextOps for BufferViewMut<'_, Rgba> {
    /// Draw a text label at the bottom of the image using the injected font.
    ///
    /// Falls back to the built-in bitmap renderer when `font` is `None`.
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) {
        let size = self.size();
        let font = match font {
            Some(f) => f,
            None => {
                debug!("no label font provided, using bitmap fallback");
                self.draw_text_bitmap(text, is_active);
                return;
            }
        };

        let color: Color<u8> = text_color_override.map(|c| c.into()).unwrap_or_else(|| self.default_text_color(is_active));

        let font_size = Pixels::new((size.y as f32 * 0.22).max(8.0));
        let scale = PxScale::from(font_size.as_raw());
        let scaled_font = font.as_scaled(scale);

        let max_width = Pixels::new(size.x as f32 * 0.9);
        let truncated = truncate_text_to_width(text, &scaled_font, max_width);
        if truncated.is_empty() {
            return;
        }

        let total_width = Pixels::new(truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum::<f32>());
        let start_x = Pixels::new(((size.x as f32 - total_width.as_raw()) / 2.0).max(0.0));
        let baseline = Pixels::new(size.y as f32 * 0.88);

        let mut pen = Vec2::<Pixels, f32>::new(start_x.as_raw(), baseline.as_raw());
        for ch in truncated.chars() {
            let glyph_id = font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, pen);
            self.draw_glyph(font, &glyph, color);
            pen.x += scaled_font.h_advance(glyph_id);
        }
    }

    /// Draw text centered horizontally at a given y-baseline with a configurable font size.
    ///
    /// Falls back to the built-in bitmap renderer when `font` is `None`.
    fn draw_text_centered(&mut self, font: Option<&FontVec>, text: &str, baseline_y: Pixels<f32>, font_size: Pixels<f32>, color: impl Into<Color<u8>>) {
        let size = self.size();
        let font = match font {
            Some(f) => f,
            None => {
                debug!("no text font provided, using bitmap fallback");
                self.draw_text_bitmap(text, false);
                return;
            }
        };

        let scale = PxScale::from(font_size.as_raw());
        let scaled_font = font.as_scaled(scale);
        let max_width = Pixels::new(size.x as f32 * 0.9);
        let truncated = truncate_text_to_width(text, &scaled_font, max_width);
        if truncated.is_empty() {
            return;
        }

        let total_width = Pixels::new(truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum::<f32>());
        let start_x = Pixels::new(((size.x as f32 - total_width.as_raw()) / 2.0).max(0.0));

        let color: Color<u8> = color.into();

        let mut pen = Vec2::<Pixels, f32>::new(start_x.as_raw(), baseline_y.as_raw());
        for ch in truncated.chars() {
            let glyph_id = font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, pen);
            self.draw_glyph(font, &glyph, color);
            pen.x += scaled_font.h_advance(glyph_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_fonts_rs::fonts::label_font;
    use nerd_fonts_rs::fonts::nerd_font;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Size;

    #[test]
    fn draw_label_renders_pixels() {
        let size = Size::new(96, 96);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut().draw_label(label_font().or_else(nerd_font), "hello", true, None);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered for label text");
    }

    #[test]
    fn draw_label_with_color_override() {
        let size = Size::new(96, 96);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<f64>::new(1.0, 0.0, 0.0);
        buf.view_mut().draw_label(label_font().or_else(nerd_font), "test", false, Some(color));

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered with color override");
    }

    #[test]
    fn draw_label_empty_string_does_not_render() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut().draw_label(label_font().or_else(nerd_font), "", false, None);

        assert!(buf.pixels().iter().all(|&p| p == 0), "no pixels should be rendered for empty string");
    }

    #[test]
    fn draw_label_without_font_uses_bitmap_fallback() {
        let size = Size::new(64, 16);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut().draw_label(None, "AB", false, None);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "bitmap fallback should render some pixels when no font is provided");
    }

    #[test]
    fn draw_text_centered_renders_pixels() {
        let size = Size::new(96, 96);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        buf.view_mut()
            .draw_text_centered(label_font().or_else(nerd_font), "center", Pixels::new(size.y as f32 / 2.0), Pixels::new(16.0), color);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered for centered text");
    }

    #[test]
    fn draw_text_centered_accepts_color_f64() {
        let size = Size::new(64, 64);
        let mut buf = PixelBuffer::new(size);
        let color_f64 = Color::<f64>::new(0.0, 1.0, 0.0);
        buf.view_mut()
            .draw_text_centered(label_font().or_else(nerd_font), "abc", Pixels::new(32.0), Pixels::new(12.0), color_f64);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered with f64 color");
    }

    #[test]
    fn draw_text_centered_empty_string_does_not_render() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        buf.view_mut()
            .draw_text_centered(label_font().or_else(nerd_font), "", Pixels::new(16.0), Pixels::new(12.0), color);

        assert!(buf.pixels().iter().all(|&p| p == 0), "no pixels should be rendered for empty string");
    }

    #[test]
    fn draw_text_centered_truncates_long_text() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        let long_text = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        buf.view_mut()
            .draw_text_centered(label_font().or_else(nerd_font), long_text, Pixels::new(16.0), Pixels::new(12.0), color);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "truncated text should still render some pixels");
    }

    #[test]
    fn draw_text_centered_without_font_uses_bitmap_fallback() {
        let size = Size::new(64, 16);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        buf.view_mut().draw_text_centered(None, "AB", Pixels::new(8.0), Pixels::new(12.0), color);

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "bitmap fallback should render some pixels when no font is provided");
    }
}
