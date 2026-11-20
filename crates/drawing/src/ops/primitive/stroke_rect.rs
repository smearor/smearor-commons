//! The [`StrokeRect`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;

/// A rect outline's draw specification: geometry, color, stroke width
/// and alpha. The stroke is centered on the rect edge with the given
/// `line_width` — the consumer computes any inset policy itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeRect {
    /// Rectangle whose edge is stroked.
    pub rect: Rect<Pixels>,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on the rect edge.
    pub line_width: Pixels<f32>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokeRect {
    /// Creates a rect stroke spec.
    pub fn new(rect: Rect<Pixels>, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            rect,
            color: color.into(),
            line_width,
            alpha: alpha.into(),
        }
    }
}
