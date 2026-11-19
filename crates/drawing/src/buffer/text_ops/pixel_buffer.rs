//! The [`DrawTextOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer delegates to its packed view.

use crate::ops::DrawTextOps;
use ab_glyph::FontVec;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;

impl DrawTextOps for PixelBuffer<Rgba> {
    fn draw_label(&mut self, font: Option<&FontVec>, text: &str, is_active: bool, text_color_override: Option<Color<f64>>) -> Result<(), Self::Error> {
        self.view_mut().draw_label(font, text, is_active, text_color_override)
    }

    fn draw_text_centered(
        &mut self,
        font: Option<&FontVec>,
        text: &str,
        baseline_y: Pixels<f32>,
        font_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        self.view_mut().draw_text_centered(font, text, baseline_y, font_size, color)
    }
}
