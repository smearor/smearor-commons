//! The [`DrawPrimitiveOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer draws primitives through its packed view.

use crate::ops::DrawPrimitiveOps;
use crate::ops::FillCircle;
use crate::ops::FillRect;
use crate::ops::StrokeRect;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;

impl DrawPrimitiveOps for PixelBuffer<Rgba> {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        self.view_mut().fill_background(color)
    }

    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error> {
        self.view_mut().fill_rect(spec)
    }

    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error> {
        self.view_mut().stroke_rect(spec)
    }

    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error> {
        self.view_mut().fill_circle(spec)
    }
}
