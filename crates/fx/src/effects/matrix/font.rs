//! A tiny embedded 5x7 bitmap font for [`ProceduralGlyphAtlas`].
//!
//! The classic public-domain GLCD letterforms (digits `0`-`9`, then
//! uppercase `A`-`Z`) in column-major order: each `u8` is one
//! column, bit `row` (0 = top) marks a lit pixel. This gives the
//! matrix rain real glyph silhouettes without shipping a font asset
//! or depending on a font loader - the [`GlyphSource`] trait stays
//! the plug for a consumer-supplied atlas.

use crate::smoothstep;
use smearor_commons_geometry::Vec2;

/// Lit columns per glyph.
const COLUMNS: usize = 5;
/// Rows per glyph.
const ROWS: usize = 7;
/// The 5x7 grid sits inside a `7 x 7` cell grid - one empty column
/// of padding on each side so neighboring glyphs never touch.
const GRID: f64 = 7.0;
/// Fraction of a cell over which the silhouette edge fades in.
const EDGE: f64 = 0.25;

/// The public-domain GLCD 5x7 letterforms, column-major - digits
/// `0`-`9` first, then uppercase `A`-`Z`.
pub(super) const GLYPHS: &[[u8; COLUMNS]] = &[
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
    [0x7E, 0x11, 0x11, 0x11, 0x7E], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36], // B
    [0x3E, 0x41, 0x41, 0x41, 0x22], // C
    [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01], // F
    [0x3E, 0x41, 0x49, 0x49, 0x7A], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40], // L
    [0x7F, 0x02, 0x0C, 0x02, 0x7F], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46], // R
    [0x46, 0x49, 0x49, 0x49, 0x31], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
    [0x1F, 0x20, 0x40, 0x20, 0x1F], // V
    [0x3F, 0x40, 0x38, 0x40, 0x3F], // W
    [0x63, 0x14, 0x08, 0x14, 0x63], // X
    [0x07, 0x08, 0x70, 0x08, 0x07], // Y
    [0x61, 0x51, 0x49, 0x45, 0x43], // Z
];

/// The number of letterforms in [`GLYPHS`].
pub(super) const GLYPH_COUNT: usize = GLYPHS.len();

/// Whether the pixel at glyph grid position (`column`, `row`) is lit.
fn lit(index: usize, column: i64, row: i64) -> bool {
    if column < 0 || column >= COLUMNS as i64 || row < 0 || row >= ROWS as i64 {
        return false;
    }
    GLYPHS[index][column as usize] & (1 << row) != 0
}

/// Coverage of glyph `index` at `uv` - the silhouette is sharp, with
/// a short [`smoothstep`] fade only on cell sides that face an
/// unlit neighbor, so stems stay solid while outlines stay soft.
pub(super) fn coverage(index: usize, uv: Vec2<()>) -> f64 {
    let cell_x = (uv.x * GRID).floor();
    let cell_y = (uv.y * GRID).floor();
    let (column, row) = (cell_x as i64 - 1, cell_y as i64);
    if !lit(index, column, row) {
        return 0.0;
    }
    let (cell_u, cell_v) = (uv.x * GRID - cell_x, uv.y * GRID - cell_y);
    let mut edge = 1.0_f64;
    if !lit(index, column - 1, row) {
        edge = edge.min(smoothstep(0.0, EDGE, cell_u));
    }
    if !lit(index, column + 1, row) {
        edge = edge.min(smoothstep(0.0, EDGE, 1.0 - cell_u));
    }
    if !lit(index, column, row - 1) {
        edge = edge.min(smoothstep(0.0, EDGE, cell_v));
    }
    if !lit(index, column, row + 1) {
        edge = edge.min(smoothstep(0.0, EDGE, 1.0 - cell_v));
    }
    edge
}
