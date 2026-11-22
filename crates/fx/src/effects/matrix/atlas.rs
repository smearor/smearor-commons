use super::GlyphSource;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Hash;

/// Hash-driven pseudo-glyph tiles - deterministic, no asset needed.
///
/// Each of the 256 atlas tiles is subdivided into a `4 x 6` stroke
/// grid; a `hash22(tile + stroke)` threshold decides whether the
/// stroke is filled. The result reads as a blocky alien glyph that is
/// stable per tile - matching the atlas' role (a fixed set of
/// repeatable glyph shapes), not BMW's bitmap shapes themselves.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProceduralGlyphAtlas;

impl GlyphSource for ProceduralGlyphAtlas {
    fn glyph(&self, tile: Vec2<()>, uv: Vec2<()>) -> f64 {
        const GRID: Vec2<()> = Vec2::<()>::new(4.0, 6.0);
        let cell = Vec2::<()>::new((uv.x * GRID.x).floor(), (uv.y * GRID.y).floor());
        let cell_uv = Vec2::<()>::new(crate::fract(uv.x * GRID.x), crate::fract(uv.y * GRID.y));
        let filled: f64 = Hash::<f64>::hash(&(tile + cell * 0.37));
        if filled < 0.45 {
            return 0.0;
        }
        // Soft stroke edges: fade near cell borders.
        crate::smoothstep(0.0, 0.15, cell_uv.x.min(1.0 - cell_uv.x)) * crate::smoothstep(0.0, 0.15, cell_uv.y.min(1.0 - cell_uv.y))
    }
}
