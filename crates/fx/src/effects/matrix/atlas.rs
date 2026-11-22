use super::GlyphSource;
use super::font;
use smearor_commons_geometry::Vec2;

/// Bitmap-font glyph tiles - deterministic, no asset needed.
///
/// The `16 x 16` atlas maps each tile onto one of the embedded GLCD
/// 5x7 letterforms in [`font`]: real digit and uppercase glyph
/// silhouettes, stable per tile. This matches the atlas' role (a
/// fixed set of repeatable glyph shapes) while staying asset-free -
/// a consumer bitmap font plugs in through the same [`GlyphSource`]
/// trait via `apply_with_glyphs`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProceduralGlyphAtlas;

impl GlyphSource for ProceduralGlyphAtlas {
    fn glyph(&self, tile: Vec2<()>, uv: Vec2<()>) -> f64 {
        const TILES: f64 = 16.0;
        let index = ((tile.x + tile.y * TILES) as usize) % font::GLYPH_COUNT;
        font::coverage(index, uv)
    }
}
