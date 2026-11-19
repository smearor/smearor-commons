//! Internal raster primitives for the buffer backend.
//!
//! `draw_glyph` rasterizes one `ab_glyph` glyph with alpha blending;
//! `draw_text_bitmap` is the fontless bitmap fallback. Kept private so
//! they stay out of the public API surface — other backends implement
//! the capability traits with their own machinery instead.

mod blend;
mod buffer_view_mut;
mod pixel_buffer;

pub(super) use blend::blend_pixel;

use ab_glyph::FontVec;
use ab_glyph::Glyph;
use smearor_commons_color::Color;

/// Internal raster primitives shared by the buffer implementations of
/// the capability traits.
pub(super) trait RasterOps {
    /// Draw a single glyph onto the view with alpha blending.
    fn draw_glyph(&mut self, font: &FontVec, glyph: &Glyph, color: Color<u8>);
    /// Simple bitmap text renderer used when no font is available.
    fn draw_text_bitmap(&mut self, text: &str, is_active: bool);
}
