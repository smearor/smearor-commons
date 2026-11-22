use smearor_commons_geometry::Vec2;

/// Where `Matrix` gets its glyph coverage - swap in a real bitmap
/// font atlas from the consumer when wanted (BMW's `uFontTexture`).
pub trait GlyphSource {
    /// Coverage of glyph `tile` at `uv` within one glyph cell, `0..1`.
    ///
    /// `uv` is the in-cell coordinate (`0..1` each axis, y down);
    /// `tile` is the atlas tile index `0..256` (BMW's 16x16 grid).
    fn glyph(&self, tile: Vec2<()>, uv: Vec2<()>) -> f64;
}
