//! The [`DrawTextOps`] capability trait.

use crate::ops::DrawBackend;
use ab_glyph::FontVec;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// Text drawing operations: bottom labels and centered text.
///
/// The font is injected per call as `Option<&FontVec>`; a `None` font
/// falls back to the built-in bitmap renderer.
pub trait DrawTextOps: DrawBackend {
    /// Draw a text label at the bottom of the view using the injected
    /// `font`; a `None` font falls back to the built-in bitmap renderer.
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) -> Result<(), Self::Error>;
    /// Draw text centered horizontally at a given y-baseline; a `None`
    /// font falls back to the built-in bitmap renderer.
    fn draw_text_centered(
        &mut self,
        font: Option<&FontVec>,
        text: &str,
        baseline_y: Pixels<f32>,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error>;

    /// Draw a text run with its baseline starting at `position`, using
    /// the injected `font`. A `None` font falls back to cairo's toy
    /// text API on the cairo backend (system `Sans`) and to the
    /// built-in bitmap renderer on the raster backend (which keeps
    /// its own layout, ignoring `position`).
    fn draw_text(
        &mut self,
        font: Option<&FontVec>,
        position: Vec2<Pixels, f32>,
        text: &str,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error>;
}

impl<T: DrawTextOps + ?Sized> DrawTextOps for &mut T {
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) -> Result<(), Self::Error> {
        (**self).draw_label(font, text, is_active, text_color_override)
    }

    fn draw_text_centered(
        &mut self,
        font: Option<&FontVec>,
        text: &str,
        baseline_y: Pixels<f32>,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        (**self).draw_text_centered(font, text, baseline_y, font_size, color)
    }

    fn draw_text(
        &mut self,
        font: Option<&FontVec>,
        position: Vec2<Pixels, f32>,
        text: &str,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        (**self).draw_text(font, position, text, font_size, color)
    }
}
