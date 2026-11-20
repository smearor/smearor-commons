//! The [`IconGrid`] draw spec.

use ab_glyph::FontVec;
use smearor_commons_geometry::Grid;

/// An icon grid's draw specification: which icons, through which font
/// and codepoint resolver, in which grid and state.
#[derive(Clone, Copy)]
pub struct IconGrid<'a> {
    /// Icon font.
    pub font: &'a FontVec,
    /// Icon names to resolve and draw.
    pub icons: &'a [&'a str],
    /// Grid layout (2x2 or 3x3).
    pub grid: Grid,
    /// Active-state flag for the default color policy.
    pub is_active: bool,
    /// Consumer-side name → codepoint resolver.
    pub resolve: &'a dyn Fn(&str) -> Option<char>,
}

impl<'a> IconGrid<'a> {
    /// Creates an icon grid spec.
    pub fn new(font: &'a FontVec, icons: &'a [&'a str], grid: Grid, is_active: bool, resolve: &'a dyn Fn(&str) -> Option<char>) -> Self {
        Self {
            font,
            icons,
            grid,
            is_active,
            resolve,
        }
    }
}
