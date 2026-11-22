//! The `Matrix` effect - `Matrix` plus its machinery: the
//! [`GlyphSource`] abstraction with the bundled
//! [`ProceduralGlyphAtlas`] substitute, and the [`rain`] column-RNG
//! domain.

mod atlas;
mod effect;
mod font;
mod glyph;
mod rain;

pub use atlas::ProceduralGlyphAtlas;
pub use effect::Matrix;
pub use glyph::GlyphSource;
