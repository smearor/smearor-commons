//! The [`DrawPrimitiveOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer draws primitives through its packed view.

use crate::ops::DrawPrimitiveOps;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;

impl DrawPrimitiveOps for PixelBuffer<Rgba> {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) {
        self.view_mut().fill_background(color);
    }

    fn fill_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) {
        self.view_mut().fill_rect(rect, color, alpha);
    }

    fn stroke_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) {
        self.view_mut().stroke_rect(rect, color, line_width, alpha);
    }

    fn fill_circle(&mut self, circle: Circle<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) {
        self.view_mut().fill_circle(circle, color, alpha);
    }
}
