//! The `RasterOps` delegation impl for `PixelBuffer<Rgba>` — an owned
//! packed buffer delegates to its packed view.

use crate::buffer::raster::RasterOps;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;

impl RasterOps for PixelBuffer<Rgba> {
    fn draw_glyph(&mut self, font: &FontVec, glyph: &Glyph, color: Color<u8>) {
        self.view_mut().draw_glyph(font, glyph, color);
    }

    fn draw_text_bitmap(&mut self, text: &str, is_active: bool) {
        self.view_mut().draw_text_bitmap(text, is_active);
    }
}
