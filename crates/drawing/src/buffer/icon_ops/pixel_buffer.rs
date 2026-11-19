//! The [`DrawIconOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer delegates to its packed view.

use crate::ops::DrawIconOps;
use ab_glyph::FontVec;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Grid;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

impl DrawIconOps for PixelBuffer<Rgba> {
    fn draw_icon(
        &mut self,
        font: Option<&FontVec>,
        icon_name: &str,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
        icon_color: Option<Color<f64>>,
    ) -> Result<(), Self::Error> {
        self.view_mut().draw_icon(font, icon_name, is_active, resolve_codepoint, icon_color)
    }

    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        self.view_mut().draw_icon_placeholder(is_active, icon_color)
    }

    fn draw_codepoint(
        &mut self,
        font: &FontVec,
        codepoint: char,
        center: Vec2<Pixels, f32>,
        icon_size: Pixels<f32>,
        color: impl Into<Color<u8>>,
    ) -> Result<(), Self::Error> {
        self.view_mut().draw_codepoint(font, codepoint, center, icon_size, color)
    }

    fn draw_icon_grid(
        &mut self,
        font: &FontVec,
        icons: &[&str],
        grid: Grid,
        is_active: bool,
        resolve_codepoint: impl Fn(&str) -> Option<char>,
    ) -> Result<(), Self::Error> {
        self.view_mut().draw_icon_grid(font, icons, grid, is_active, resolve_codepoint)
    }
}
