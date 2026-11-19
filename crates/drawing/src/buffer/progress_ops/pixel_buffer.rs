//! The [`DrawProgressOps`] delegation impl for `PixelBuffer<Rgba>` —
//! an owned packed buffer delegates to its packed view.

use crate::ops::DrawProgressOps;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

impl DrawProgressOps for PixelBuffer<Rgba> {
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) {
        self.view_mut().draw_progress_bar(value, color);
    }

    fn draw_gauge(
        &mut self,
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    ) {
        self.view_mut().draw_gauge(center, radius, line_width, value, color, track);
    }
}
