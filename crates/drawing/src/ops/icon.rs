//! The [`DrawIconOps`] capability trait.

use crate::ops::DrawBackend;
use ab_glyph::FontVec;
use smearor_commons_color::Color;
use smearor_commons_geometry::Grid;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// Icon drawing operations: named icons resolved through a caller-provided
/// codepoint resolver, raw codepoints, a placeholder and icon grids.
///
/// The icon font is injected per call — [`DrawIconOps::draw_icon`] takes
/// `Option<&FontVec>` and falls back to
/// [`DrawIconOps::draw_icon_placeholder`] on `None`; the other methods
/// take `&FontVec` and are simply not called when no font is available.
pub trait DrawIconOps: DrawBackend {
    /// Draw an icon by name using the injected `font`, resolved via
    /// `resolve_codepoint`; a `None` font or an unknown name falls back to
    /// [`DrawIconOps::draw_icon_placeholder`]. The `icon_color` override
    /// applies to the placeholder as well.
    fn draw_icon(
        &mut self,
        font: Option<&FontVec>,
        icon_name: &str,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
        icon_color: Option<Color<f64>>,
    ) -> Result<(), Self::Error>;
    /// Draw a circular placeholder when the font or icon is unavailable.
    /// `icon_color` overrides the active-state default color when `Some`.
    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error>;
    /// Draw a glyph by Unicode codepoint at a custom position and size.
    fn draw_codepoint(
        &mut self,
        font: &FontVec,
        codepoint: char,
        center: Vec2<Pixels, f32>,
        icon_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error>;
    /// Draw a 2x2 or 3x3 icon grid within the view area.
    fn draw_icon_grid(
        &mut self,
        font: &FontVec,
        icons: &[&str],
        grid: Grid,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
    ) -> Result<(), Self::Error>;
}

impl<T: DrawIconOps + ?Sized> DrawIconOps for &mut T {
    fn draw_icon(
        &mut self,
        font: Option<&FontVec>,
        icon_name: &str,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
        icon_color: Option<Color<f64>>,
    ) -> Result<(), Self::Error> {
        (**self).draw_icon(font, icon_name, is_active, resolve_codepoint, icon_color)
    }

    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        (**self).draw_icon_placeholder(is_active, icon_color)
    }

    fn draw_codepoint(
        &mut self,
        font: &FontVec,
        codepoint: char,
        center: Vec2<Pixels, f32>,
        icon_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        (**self).draw_codepoint(font, codepoint, center, icon_size, color)
    }

    fn draw_icon_grid(
        &mut self,
        font: &FontVec,
        icons: &[&str],
        grid: Grid,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
    ) -> Result<(), Self::Error> {
        (**self).draw_icon_grid(font, icons, grid, is_active, resolve_codepoint)
    }
}
