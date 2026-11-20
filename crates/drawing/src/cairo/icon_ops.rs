//! The [`DrawIconOps`] implementation for [`CairoTarget`].
//!
//! Mirrors the raster backend's metric pipeline — `glyph_fit_scale`,
//! `centered_glyph`, `resolve_codepoint` fallback — but fills glyph
//! vector paths instead of rasterizing coverage, so icons scale
//! cleanly at any surface size.

use crate::cairo::CairoTarget;
use crate::cairo::glyph_path::fill_glyph;
use crate::ops::DrawIconOps;
use crate::ops::DrawTarget;
use crate::tech::glyph_fit_scale;
use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::PxScale;
use smearor_commons_color::Color;
use smearor_commons_font::centered_glyph;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use tracing::debug;

impl DrawIconOps for CairoTarget {
    /// Same layout as the raster backend: icon occupying the upper 60%
    /// of the target, circular placeholder on `None` font or unknown
    /// name — but the glyph renders as a filled cairo path.
    fn draw_icon(
        &mut self,
        font: Option<&FontVec>,
        icon_name: &str,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
        icon_color: Option<Color<f64>>,
    ) -> Result<(), Self::Error> {
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
                debug!("unknown icon name '{icon_name}', drawing placeholder");
                return self.draw_icon_placeholder(is_active, icon_color);
            }
        };

        let color: Color<f64> = icon_color.unwrap_or_else(|| self.default_text_color(is_active).into());
        color.apply(&self.ctx);

        let icon_size = Pixels::new(size.x.min(size.y) as f32 * 0.6);
        let glyph_id = font.glyph_id(codepoint);
        let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
        let scale = PxScale::from(em_scale.as_raw());

        let size_f = size.to_f32();
        let target = Vec2::<Pixels, f32>::new(size_f.half().x, size_f.y * 0.4);
        let glyph = centered_glyph(font, glyph_id, scale, target);
        fill_glyph(&self.ctx, font, &glyph)
    }

    /// The circular placeholder as a cairo `arc` + `fill` — identical
    /// position and radius to the raster version.
    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        let size = self.size();
        let color: Color<f64> = icon_color.unwrap_or_else(|| self.default_text_color(is_active).into());
        let size_f = size.to_f64();
        let cx = size_f.half().x;
        let radius = (size.x.min(size.y) / 3).max(8) as f64;

        color.apply(&self.ctx);
        self.ctx.arc(cx, cx - size_f.y / 6.0, radius, 0.0, std::f64::consts::TAU);
        self.ctx.fill()
    }

    /// A glyph at a custom position and size, filled as a cairo path.
    fn draw_codepoint(
        &mut self,
        font: &FontVec,
        codepoint: char,
        center: Vec2<Pixels, f32>,
        icon_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        let color: Color<f64> = color.into().into();
        color.apply(&self.ctx);

        let glyph_id = font.glyph_id(codepoint);
        let em_scale = glyph_fit_scale(font, glyph_id, icon_size);
        let scale = PxScale::from(em_scale.as_raw());
        let glyph = centered_glyph(font, glyph_id, scale, center);
        fill_glyph(&self.ctx, font, &glyph)
    }

    /// The same grid layout as the raster backend; unknown icon names
    /// are skipped rather than drawing per-cell placeholders.
    fn draw_icon_grid(
        &mut self,
        font: &FontVec,
        icons: &[&str],
        grid: smearor_commons_geometry::Grid,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
    ) -> Result<(), Self::Error> {
        let size = self.size();
        let cell_size = grid.cell_size(size);
        let icon_size = Pixels::new(cell_size.x.min(cell_size.y) as f32 * 0.6);
        let color: Color<f64> = self.default_text_color(is_active).into();
        color.apply(&self.ctx);

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

            let glyph = centered_glyph(font, glyph_id, scale, cell.center().to_f32().with_unit());
            fill_glyph(&self.ctx, font, &glyph)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::Context;
    use cairo::Format;
    use cairo::ImageSurface;
    use nerd_fonts_rs::NerdFonts;
    use nerd_fonts_rs::fonts::nerd_font;
    use smearor_commons_geometry::Grid;
    use smearor_commons_geometry::Size;

    fn resolve_codepoint(name: &str) -> Option<char> {
        NerdFonts::codepoint(name).map(|c| c.as_char())
    }

    /// Runs `draw` on a fresh ARgb32 surface and returns whether any
    /// pixel gained coverage.
    fn renders_pixels(size: Size<u32>, draw: impl Fn(&mut CairoTarget)) -> bool {
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        let mut target = CairoTarget::new(&Context::new(&surface).unwrap(), size);
        draw(&mut target);
        target.status().unwrap();
        drop(target);
        let mut surface = surface;
        surface.flush();
        surface.data().unwrap().iter().any(|&byte| byte != 0)
    }

    #[test]
    fn draw_codepoint_renders_via_glyph_path() {
        let size = Size::new(64, 64);
        let font = nerd_font().expect("nerd font should be loaded");
        assert!(renders_pixels(size, |target| {
            target
                .draw_codepoint(font, '\u{F11B}', Vec2::<Pixels, f32>::new(32.0, 32.0), Pixels::new(48.0), Color::<u8>::new(255, 255, 255))
                .unwrap();
        }));
    }

    #[test]
    fn draw_icon_with_valid_name_renders() {
        let size = Size::new(64, 64);
        assert!(renders_pixels(size, |target| {
            target.draw_icon(nerd_font(), "nf-fa-gamepad", true, resolve_codepoint, None).unwrap();
        }));
    }

    #[test]
    fn draw_icon_placeholder_draws_circle() {
        let size = Size::new(32, 32);
        let surface = ImageSurface::create(Format::ARgb32, 32, 32).unwrap();
        let mut target = CairoTarget::new(&Context::new(&surface).unwrap(), size);
        target.draw_icon_placeholder(false, None).unwrap();
        drop(target);

        let mut surface = surface;
        surface.flush();
        let stride = surface.stride() as usize;
        let data = surface.data().unwrap();
        // Placeholder center: (cx, cx - size.y/6) with cx = 16 → (16, 10.67)
        let center = 10 * stride + 16 * 4;
        assert_eq!(data[center + 3], 255, "placeholder center should be opaque");
        // A corner stays untouched.
        assert_eq!(data[3], 0, "corner should be transparent");
    }

    #[test]
    fn draw_icon_grid_renders() {
        let size = Size::new(96, 96);
        let icons = ["nf-fa-gamepad", "nf-fa-star", "nf-fa-heart", "nf-fa-bug"];
        let font = nerd_font().expect("nerd font should be loaded");
        assert!(renders_pixels(size, |target| {
            target.draw_icon_grid(font, &icons, Grid::for_items(4, 2), true, resolve_codepoint).unwrap();
        }));
    }

    #[test]
    fn draw_icon_without_font_draws_placeholder() {
        let size = Size::new(32, 32);
        assert!(renders_pixels(size, |target| {
            target.draw_icon(None, "nf-fa-gamepad", false, resolve_codepoint, None).unwrap();
        }));
    }
}
