//! The [`DrawIconOps`] capability trait and its draw specs.

mod codepoint;
mod icon_grid;
mod named_icon;

pub use codepoint::Codepoint;
pub use icon_grid::IconGrid;
pub use named_icon::Icon;

use crate::ops::DrawBackend;
use smearor_commons_color::Color;

/// Icon drawing operations: named icons resolved through a caller-provided
/// codepoint resolver, raw codepoints, a placeholder and icon grids.
///
/// The icon font is injected per call — [`DrawIconOps::draw_icon`] takes
/// `Option<&FontVec>` and falls back to
/// [`DrawIconOps::draw_icon_placeholder`] on `None`; the other methods
/// take `&FontVec` and are simply not called when no font is available.
pub trait DrawIconOps: DrawBackend {
    /// Draw `icon` by name; a `None` font or an unknown name falls back
    /// to [`DrawIconOps::draw_icon_placeholder`]. The `icon.color`
    /// override applies to the placeholder as well.
    fn draw_icon(&mut self, icon: Icon<'_>) -> Result<(), Self::Error>;
    /// Draw a circular placeholder when the font or icon is unavailable.
    /// `icon_color` overrides the active-state default color when `Some`.
    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error>;
    /// Draw `glyph`: a glyph by Unicode codepoint at a custom position
    /// and size.
    fn draw_codepoint(&mut self, glyph: Codepoint<'_>) -> Result<(), Self::Error>;
    /// Draw `spec`'s icons as a 2x2 or 3x3 grid within the view area.
    fn draw_icon_grid(&mut self, spec: IconGrid<'_>) -> Result<(), Self::Error>;
}

impl<T: DrawIconOps + ?Sized> DrawIconOps for &mut T {
    fn draw_icon(&mut self, icon: Icon<'_>) -> Result<(), Self::Error> {
        (**self).draw_icon(icon)
    }

    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        (**self).draw_icon_placeholder(is_active, icon_color)
    }

    fn draw_codepoint(&mut self, glyph: Codepoint<'_>) -> Result<(), Self::Error> {
        (**self).draw_codepoint(glyph)
    }

    fn draw_icon_grid(&mut self, spec: IconGrid<'_>) -> Result<(), Self::Error> {
        (**self).draw_icon_grid(spec)
    }
}
