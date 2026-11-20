//! The [`DrawTextOps`] implementation for [`CairoTarget`] plus
//! cairo-only text extras.
//!
//! The trait methods mirror the raster backend's metric pipeline —
//! `PxScale`, `truncate_text_to_width`, pen advance via `h_advance` —
//! but fill glyph vector paths instead of rasterizing coverage.
//!
//! Fallback asymmetry (deliberate): a `None` font draws nothing on
//! cairo — the bitmap fallback stays raster-exclusive because it
//! renders fixed bitmap glyphs, which contradicts cairo's vector
//! pipeline and has no consumer.
//!
//! The extra inherent methods cover the **toy text API** path the
//! shared vocabulary cannot express: system font names, rotation and
//! outline-halo styling (chess `text_centered` and qdo36 `place_label`
//! lineage). They draw with the caller-configured toy font — face,
//! size and source color are set on [`CairoTarget::context`]
//! beforehand, exactly like the consumer originals did.

use crate::cairo::CairoTarget;
use crate::cairo::glyph_path::fill_glyph;
use crate::ops::DrawTarget;
use crate::ops::DrawTextOps;
use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use ab_glyph::PxScale;
use ab_glyph::ScaleFont;
use cairo::FontSlant;
use cairo::FontWeight;
use smearor_commons_color::Color;
use smearor_commons_font::truncate_text_to_width;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Radians;
use smearor_commons_geometry::Vec2;
use tracing::debug;

impl DrawTextOps for CairoTarget {
    /// Same layout as the raster backend — `size·0.22` font height,
    /// 90%-width truncation, baseline at `size·0.88` — but a `None`
    /// font is a no-op (no bitmap fallback on cairo, see module docs).
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) -> Result<(), Self::Error> {
        let size = self.size();
        let font = match font {
            Some(f) => f,
            None => {
                debug!("no label font provided, skipping text on cairo");
                return Ok(());
            }
        };

        let color: Color<f64> = text_color_override.unwrap_or_else(|| self.default_text_color(is_active).into());
        color.apply(&self.ctx);

        let font_size = Pixels::new((size.y as f32 * 0.22).max(8.0));
        let scale = PxScale::from(font_size.as_raw());
        let scaled_font = font.as_scaled(scale);

        let max_width = Pixels::new(size.x as f32 * 0.9);
        let truncated = truncate_text_to_width(text, &scaled_font, max_width);
        if truncated.is_empty() {
            return Ok(());
        }

        let total_width = Pixels::new(truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum::<f32>());
        let start_x = Pixels::new(((size.x as f32 - total_width.as_raw()) / 2.0).max(0.0));
        let baseline = Pixels::new(size.y as f32 * 0.88);

        let mut pen = Vec2::<Pixels, f32>::new(start_x.as_raw(), baseline.as_raw());
        for ch in truncated.chars() {
            let glyph_id = font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, pen);
            fill_glyph(&self.ctx, font, &glyph)?;
            pen.x += scaled_font.h_advance(glyph_id);
        }
        Ok(())
    }

    /// Centered text at `baseline_y` with the caller's `font_size`,
    /// truncated to 90% target width — same layout as the raster
    /// backend, filled paths instead of coverage raster.
    fn draw_text_centered(
        &mut self,
        font: Option<&FontVec>,
        text: &str,
        baseline_y: Pixels<f32>,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        let size = self.size();
        let font = match font {
            Some(f) => f,
            None => {
                debug!("no text font provided, skipping text on cairo");
                return Ok(());
            }
        };

        let scale = PxScale::from(font_size.as_raw());
        let scaled_font = font.as_scaled(scale);
        let max_width = Pixels::new(size.x as f32 * 0.9);
        let truncated = truncate_text_to_width(text, &scaled_font, max_width);
        if truncated.is_empty() {
            return Ok(());
        }

        let total_width = Pixels::new(truncated.chars().map(|c| scaled_font.h_advance(font.glyph_id(c))).sum::<f32>());
        let start_x = Pixels::new(((size.x as f32 - total_width.as_raw()) / 2.0).max(0.0));
        let color: Color<f64> = color.into().into();
        color.apply(&self.ctx);

        let mut pen = Vec2::<Pixels, f32>::new(start_x.as_raw(), baseline_y.as_raw());
        for ch in truncated.chars() {
            let glyph_id = font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, pen);
            fill_glyph(&self.ctx, font, &glyph)?;
            pen.x += scaled_font.h_advance(glyph_id);
        }
        Ok(())
    }

    /// Free-standing text at an explicit baseline anchor. With an
    /// injected `FontVec` glyphs are laid out from `position` onward;
    /// without one cairo's toy text API draws a system `Sans` — the
    /// asymmetry to the metric methods is deliberate: free text is
    /// exactly where the toy path has a consumer (GTK examples).
    fn draw_text(
        &mut self,
        font: Option<&FontVec>,
        position: Vec2<Pixels, f32>,
        text: &str,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        let color: Color<f64> = color.into().into();
        let font = match font {
            Some(f) => f,
            None => {
                color.apply(&self.ctx);
                self.ctx.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
                self.ctx.set_font_size(f64::from(font_size.as_raw()));
                self.ctx.move_to(f64::from(position.x), f64::from(position.y));
                self.ctx.show_text(text)?;
                return Ok(());
            }
        };

        let scale = PxScale::from(font_size.as_raw());
        let scaled_font = font.as_scaled(scale);
        color.apply(&self.ctx);
        let mut pen = position;
        for ch in text.chars() {
            let glyph_id = font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(scale, pen);
            fill_glyph(&self.ctx, font, &glyph)?;
            pen.x += scaled_font.h_advance(glyph_id);
        }
        Ok(())
    }
}

impl CairoTarget {
    /// Draws `text` centered at `at`, rotated by `angle`, with the toy
    /// font face/size/source the caller set on
    /// [`context()`](Self::context) beforehand (chess `text_centered`
    /// lineage — rotated piece labels).
    pub fn draw_rotated_text(&mut self, text: &str, at: Vec2<Pixels>, angle: Radians) -> Result<(), cairo::Error> {
        let extents = self.ctx.text_extents(text)?;
        self.ctx.save()?;
        self.ctx.translate(at.x, at.y);
        self.ctx.rotate(angle.as_f64());
        self.ctx
            .move_to(-(extents.x_bearing() + extents.width() / 2.0), -(extents.y_bearing() + extents.height() / 2.0));
        let result = self.ctx.show_text(text);
        self.ctx.restore()?;
        result
    }

    /// Draws `text` centered at `at`, rotated by `angle`, tracing the
    /// glyph outlines in `halo` before filling them in `ink` — the
    /// wide round-joined stroke lifts the text off its background
    /// (qdo36 `place_label` lineage — district captions over map
    /// detail). Face and size come from the caller's toy font
    /// configuration; the halo width derives from the configured font
    /// height like the original's `font_px·0.28` policy.
    pub fn draw_halo_text(&mut self, text: &str, at: Vec2<Pixels>, angle: Radians, halo: Color<f64>, ink: Color<f64>) -> Result<(), cairo::Error> {
        let extents = self.ctx.text_extents(text)?;
        let font_height = self.ctx.font_extents()?.height();
        self.ctx.save()?;
        self.ctx.translate(at.x, at.y);
        self.ctx.rotate(angle.as_f64());
        self.ctx.move_to(-extents.width() / 2.0, extents.height() / 3.0);
        self.ctx.text_path(text);
        self.ctx.set_line_join(cairo::LineJoin::Round);
        self.ctx.set_line_width((font_height * 0.28).max(0.8));
        halo.apply(&self.ctx);
        let stroke_result = self.ctx.stroke_preserve();
        ink.apply(&self.ctx);
        let fill_result = self.ctx.fill();
        self.ctx.restore()?;
        stroke_result.and(fill_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::Context;
    use cairo::FontSlant;
    use cairo::FontWeight;
    use cairo::Format;
    use cairo::ImageSurface;
    use nerd_fonts_rs::fonts::label_font;
    use nerd_fonts_rs::fonts::nerd_font;
    use smearor_commons_geometry::Size;

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
    fn draw_label_renders_glyph_paths() {
        let size = Size::new(96, 96);
        assert!(renders_pixels(size, |target| {
            target.draw_label(label_font().or_else(nerd_font), "hello", true, None).unwrap();
        }));
    }

    #[test]
    fn draw_label_without_font_is_noop_on_cairo() {
        // Documented asymmetry: no bitmap fallback on the cairo backend.
        let size = Size::new(64, 16);
        assert!(!renders_pixels(size, |target| {
            target.draw_label(None, "AB", false, None).unwrap();
        }));
    }

    #[test]
    fn draw_text_centered_renders() {
        let size = Size::new(96, 96);
        assert!(renders_pixels(size, |target| {
            target
                .draw_text_centered(
                    label_font().or_else(nerd_font),
                    "center",
                    Pixels::new(48.0),
                    Pixels::new(16.0),
                    Color::<u8>::new(255, 255, 255),
                )
                .unwrap();
        }));
    }

    #[test]
    fn draw_rotated_text_renders_with_toy_font() {
        let size = Size::new(96, 96);
        assert!(renders_pixels(size, |target| {
            target.context().select_font_face("sans", FontSlant::Normal, FontWeight::Normal);
            target.context().set_font_size(20.0);
            target.context().set_source_rgba(1.0, 1.0, 1.0, 1.0);
            target.draw_rotated_text("A5", Vec2::new(48.0, 48.0), Radians::QUARTER_TURN).unwrap();
        }));
    }

    #[test]
    fn draw_halo_text_renders_outline_and_ink() {
        let size = Size::new(128, 64);
        assert!(renders_pixels(size, |target| {
            target.context().select_font_face("serif", FontSlant::Italic, FontWeight::Normal);
            target.context().set_font_size(28.0);
            target
                .draw_halo_text(
                    "Marais",
                    Vec2::new(64.0, 32.0),
                    Radians::ZERO,
                    Color::<f64>::new(1.0, 1.0, 0.9),
                    Color::<f64>::new(0.2, 0.1, 0.3),
                )
                .unwrap();
        }));
    }
}
