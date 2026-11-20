//! The [`DrawIconOps`] implementations for the buffer targets.

use crate::buffer::raster::RasterOps;
use crate::ops::Codepoint;
use crate::ops::DrawIconOps;
use crate::ops::DrawTarget;
use crate::ops::Icon;
use crate::ops::IconGrid;
use crate::tech::glyph_fit_scale;
use ab_glyph::Font;
use ab_glyph::PxScale;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_font::centered_glyph;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use tracing::debug;

impl DrawIconOps for BufferViewMut<'_, Rgba> {
    /// Draw an icon centered on the image, occupying the upper portion.
    ///
    /// The icon name is resolved to a codepoint via the caller-provided
    /// resolver function. If no font is injected or the icon is unknown, a
    /// circular placeholder is drawn instead.
    fn draw_icon(&mut self, icon: Icon<'_>) -> Result<(), Self::Error> {
        let Icon {
            font,
            name: icon_name,
            is_active,
            resolve: resolve_codepoint,
            color: icon_color,
        } = icon;
        let size = self.size();
        let font = match font {
            Some(f) => f,
            None => {
                debug!("no icon font provided, drawing placeholder");
                return self.draw_icon_placeholder(is_active, icon_color);
            }
        };

        let codepoint = match resolve_codepoint(icon_name) {
            Some(c) => c,
            None => {
                debug!("unknown icon name '{}', drawing placeholder", icon_name);
                return self.draw_icon_placeholder(is_active, icon_color);
            }
        };

        let color: Color<u8> = icon_color.map(|c| c.into()).unwrap_or_else(|| self.default_text_color(is_active));

        let icon_size = Pixels::new(size.x.min(size.y) as f32 * 0.6);
        let glyph_id = font.glyph_id(codepoint);
        let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
        let scale = PxScale::from(em_scale.as_raw());

        // Center the glyph bounds in the upper portion of the buffer
        let size_f = size.to_f32();
        let target = Vec2::<Pixels, f32>::new(size_f.half().x, size_f.y * 0.4);
        let glyph = centered_glyph(font, glyph_id, scale, target);
        self.draw_glyph(font, &glyph, color);
        Ok(())
    }

    /// Draw a simple circular placeholder when the font or icon is unavailable.
    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        let size = self.size();
        let color: Color<u8> = icon_color.map(|c| c.into()).unwrap_or_else(|| self.default_text_color(is_active));
        let stride = self.stride().as_usize();
        let pixels = self.data_mut();
        let size_f = size.to_f64();
        let cx = size_f.half().x;
        let center = Vec2::<Pixels>::new(cx, cx - size_f.y / 6.0);
        let circle = Circle::<Pixels>::new(center, (size.x.min(size.y) / 3).max(8) as f64);

        for y in 0..size.y as i32 {
            for x in 0..size.x as i32 {
                if circle.contains(&Vec2::<Pixels>::new(x as f64, y as f64)) {
                    let idx = y as usize * stride + x as usize * Rgba::BYTES_PER_PIXEL;
                    Rgba::write_pixel(&mut pixels[idx..idx + Rgba::BYTES_PER_PIXEL], color.r, color.g, color.b, color.a);
                }
            }
        }
        Ok(())
    }

    /// Draw a glyph by its Unicode codepoint at a custom position and size.
    fn draw_codepoint(&mut self, glyph: Codepoint<'_>) -> Result<(), Self::Error> {
        let Codepoint {
            font,
            codepoint,
            center,
            size: icon_size,
            color,
        } = glyph;
        let glyph_id = font.glyph_id(codepoint);
        let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
        let scale = PxScale::from(em_scale.as_raw());
        let glyph = centered_glyph(font, glyph_id, scale, center);
        self.draw_glyph(font, &glyph, color);
        Ok(())
    }

    /// Draw a 2x2 or 3x3 icon grid within the image area.
    fn draw_icon_grid(&mut self, spec: IconGrid<'_>) -> Result<(), Self::Error> {
        let IconGrid {
            font,
            icons,
            grid,
            is_active,
            resolve: resolve_codepoint,
        } = spec;
        let size = self.size();
        let cell_size = grid.cell_size(size);
        let icon_size = Pixels::new(cell_size.x.min(cell_size.y) as f32 * 0.6);
        let color = self.default_text_color(is_active);

        for (i, icon_name) in icons.iter().enumerate() {
            let codepoint = match resolve_codepoint(icon_name) {
                Some(c) => c,
                None => continue,
            };

            let Some(cell_pos) = grid.cell(i as u32) else {
                continue;
            };
            let cell = grid.rect(cell_pos, size);

            let glyph_id = font.glyph_id(codepoint);
            let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
            let scale = PxScale::from(em_scale.as_raw());

            // Center the glyph bounds in its cell
            let glyph = centered_glyph(font, glyph_id, scale, cell.center().to_f32().with_unit());
            self.draw_glyph(font, &glyph, color);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nerd_fonts_rs::NerdFonts;
    use nerd_fonts_rs::fonts::nerd_font;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Grid;
    use smearor_commons_geometry::Size;

    fn resolve_codepoint(name: &str) -> Option<char> {
        NerdFonts::codepoint(name).map(|c| c.as_char())
    }

    #[test]
    fn draw_icon_placeholder_draws_circle() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut().draw_icon_placeholder(false, None).unwrap();

        let color = buf.default_text_color(false);
        let bytes: [u8; 4] = color.into();
        let center_x = size.x / 2;
        let center_y = size.x / 2;
        let cx = center_x as i32;
        let cy = (center_y as i32) - (size.y as i32 / 6);
        let idx = (cy as u32 * size.x + cx as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(&buf.pixels()[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "center pixel should be filled by placeholder circle");

        let dx = (size.x.min(size.y) / 3).max(8) as i32 + 2;
        let outside_idx = (cy as u32 * size.x + (cx + dx) as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(
            &buf.pixels()[outside_idx..outside_idx + Rgba::BYTES_PER_PIXEL],
            &[0, 0, 0, 0],
            "pixel outside circle should be untouched"
        );
    }

    #[test]
    fn draw_icon_placeholder_active_uses_active_color() {
        let size = Size::new(16, 16);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut().draw_icon_placeholder(true, None).unwrap();

        let active_color = buf.default_text_color(true);
        let bytes: [u8; 4] = active_color.into();
        let center_x = size.x / 2;
        let center_y = size.x / 2;
        let cx = center_x as i32;
        let cy = (center_y as i32) - (size.y as i32 / 6);
        let idx = (cy as u32 * size.x + cx as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(&buf.pixels()[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "center pixel should use active color");
    }

    #[test]
    fn draw_codepoint_renders_pixels() {
        let size = Size::new(64, 64);
        let mut buf = PixelBuffer::new(size);
        let color = Color::<u8>::new(255, 255, 255);
        let font = nerd_font().expect("nerd font should be loaded");
        buf.view_mut().draw_codepoint(Codepoint::new(
            font,
            '\u{F11B}',
            Vec2::<Pixels, f32>::new(size.x as f32 / 2.0, size.y as f32 / 2.0),
            Pixels::new(48.0),
            color,
        ));

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered for a valid codepoint");
    }

    #[test]
    fn draw_codepoint_accepts_color_f64() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        let color_f64 = Color::<f64>::new(1.0, 0.0, 0.0);
        let font = nerd_font().expect("nerd font should be loaded");
        buf.view_mut()
            .draw_codepoint(Codepoint::new(
                font,
                '\u{F11B}',
                Vec2::<Pixels, f32>::new(size.x as f32 / 2.0, size.y as f32 / 2.0),
                Pixels::new(24.0),
                color_f64,
            ))
            .unwrap();

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered with f64 color");
    }

    #[test]
    fn draw_icon_with_valid_name_renders() {
        let size = Size::new(64, 64);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut()
            .draw_icon(Icon::new(nerd_font(), "nf-fa-gamepad", true, &resolve_codepoint, Some(Color::<f64>::new(1.0, 1.0, 1.0))));

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered for a valid icon name");
    }

    #[test]
    fn draw_icon_with_unknown_name_draws_placeholder() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut()
            .draw_icon(Icon::new(nerd_font(), "nf-xx-nonexistent", false, &resolve_codepoint, None))
            .unwrap();

        let placeholder_color = buf.default_text_color(false);
        let bytes: [u8; 4] = placeholder_color.into();
        let center_x = size.x / 2;
        let center_y = size.x / 2;
        let cx = center_x as i32;
        let cy = (center_y as i32) - (size.y as i32 / 6);
        let idx = (cy as u32 * size.x + cx as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(&buf.pixels()[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "placeholder should be drawn for unknown icon name");
    }

    #[test]
    fn draw_icon_without_font_draws_placeholder() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut()
            .draw_icon(Icon::new(None, "nf-fa-gamepad", false, &resolve_codepoint, None))
            .unwrap();

        let placeholder_color = buf.default_text_color(false);
        let bytes: [u8; 4] = placeholder_color.into();
        let center_x = size.x / 2;
        let center_y = size.x / 2;
        let cx = center_x as i32;
        let cy = (center_y as i32) - (size.y as i32 / 6);
        let idx = (cy as u32 * size.x + cx as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(
            &buf.pixels()[idx..idx + Rgba::BYTES_PER_PIXEL],
            &bytes,
            "placeholder should be drawn when no font is provided"
        );
    }

    #[test]
    fn draw_icon_placeholder_uses_color_override() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        let red = Color::<f64>::new(1.0, 0.0, 0.0);
        buf.view_mut()
            .draw_icon(Icon::new(nerd_font(), "nf-xx-nonexistent", false, &resolve_codepoint, Some(red)));

        let bytes: [u8; 4] = Color::<u8>::from(red).into();
        let cx = (size.x / 2) as i32;
        let cy = (size.x / 2) as i32 - (size.y as i32 / 6);
        let idx = (cy as u32 * size.x + cx as u32) as usize * Rgba::BYTES_PER_PIXEL;
        assert_eq!(&buf.pixels()[idx..idx + Rgba::BYTES_PER_PIXEL], &bytes, "placeholder should honor the icon color override");
    }

    #[test]
    fn draw_icon_with_none_color_uses_default() {
        let size = Size::new(64, 64);
        let mut buf = PixelBuffer::new(size);
        buf.view_mut()
            .draw_icon(Icon::new(nerd_font(), "nf-fa-gamepad", true, &resolve_codepoint, None))
            .unwrap();

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered with default active color");
    }

    #[test]
    fn draw_icon_grid_renders_multiple_icons() {
        let size = Size::new(96, 96);
        let mut buf = PixelBuffer::new(size);
        let icons = ["nf-fa-gamepad", "nf-fa-star", "nf-fa-heart", "nf-fa-bug"];
        let font = nerd_font().expect("nerd font should be loaded");
        buf.view_mut()
            .draw_icon_grid(IconGrid::new(font, &icons, Grid::for_items(icons.len() as u32, 2), true, &resolve_codepoint));

        let non_zero = buf.pixels().as_chunks::<4>().0.iter().filter(|px| px.iter().any(|&b| b != 0)).count();
        assert!(non_zero > 0, "at least some pixels should be rendered for icon grid");
    }

    #[test]
    fn draw_icon_grid_with_empty_icons_does_not_panic() {
        let size = Size::new(32, 32);
        let mut buf = PixelBuffer::new(size);
        let icons: [&str; 0] = [];
        let font = nerd_font().expect("nerd font should be loaded");
        buf.view_mut()
            .draw_icon_grid(IconGrid::new(font, &icons, Grid::for_items(icons.len() as u32, 2), false, &resolve_codepoint));

        assert!(buf.pixels().iter().all(|&p| p == 0), "no pixels should be rendered for empty icon list");
    }

    #[test]
    fn draw_icon_grid_with_unknown_icons_skips_them() {
        let size = Size::new(64, 64);
        let mut buf = PixelBuffer::new(size);
        let icons = ["nf-xx-nonexistent", "nf-yy-also-nonexistent"];
        let font = nerd_font().expect("nerd font should be loaded");
        buf.view_mut()
            .draw_icon_grid(IconGrid::new(font, &icons, Grid::for_items(icons.len() as u32, 2), false, &resolve_codepoint));

        assert!(buf.pixels().iter().all(|&p| p == 0), "no pixels should be rendered when all icons are unknown");
    }
}
