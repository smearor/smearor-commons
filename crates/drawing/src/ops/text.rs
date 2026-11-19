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
