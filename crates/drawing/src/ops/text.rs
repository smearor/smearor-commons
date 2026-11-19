//! The [`DrawTextOps`] capability trait.

use ab_glyph::FontVec;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;

/// Text drawing operations: bottom labels and centered text.
///
/// The font is injected per call as `Option<&FontVec>`; a `None` font
/// falls back to the built-in bitmap renderer.
pub trait DrawTextOps {
    /// Draw a text label at the bottom of the view using the injected
    /// `font`; a `None` font falls back to the built-in bitmap renderer.
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>);
    /// Draw text centered horizontally at a given y-baseline; a `None`
    /// font falls back to the built-in bitmap renderer.
    fn draw_text_centered(&mut self, font: Option<&FontVec>, text: &str, baseline_y: Pixels<f32>, font_size: Pixels<f32>, color: impl Into<Color<u8>>);
}

impl<T: DrawTextOps + ?Sized> DrawTextOps for &mut T {
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) {
        (**self).draw_label(font, text, is_active, text_color_override);
    }

    fn draw_text_centered(&mut self, font: Option<&FontVec>, text: &str, baseline_y: Pixels<f32>, font_size: Pixels<f32>, color: impl Into<Color<u8>>) {
        (**self).draw_text_centered(font, text, baseline_y, font_size, color);
    }
}
