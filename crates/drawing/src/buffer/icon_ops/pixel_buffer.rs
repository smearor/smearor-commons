//! The [`DrawIconOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer delegates to its packed view.

use crate::ops::Codepoint;
use crate::ops::DrawIconOps;
use crate::ops::Icon;
use crate::ops::IconGrid;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;

impl DrawIconOps for PixelBuffer<Rgba> {
    fn draw_icon(&mut self, icon: Icon<'_>) -> Result<(), Self::Error> {
        self.view_mut().draw_icon(icon)
    }

    fn draw_icon_placeholder(&mut self, is_active: bool, icon_color: Option<Color<f64>>) -> Result<(), Self::Error> {
        self.view_mut().draw_icon_placeholder(is_active, icon_color)
    }

    fn draw_codepoint(&mut self, glyph: Codepoint<'_>) -> Result<(), Self::Error> {
        self.view_mut().draw_codepoint(glyph)
    }

    fn draw_icon_grid(&mut self, spec: IconGrid<'_>) -> Result<(), Self::Error> {
        self.view_mut().draw_icon_grid(spec)
    }
}
