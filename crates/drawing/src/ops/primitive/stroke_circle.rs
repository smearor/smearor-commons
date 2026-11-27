//! The [`StrokeCircle`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;

/// A circle outline's draw specification: ring geometry, color,
/// stroke width and alpha. The stroke is centered on the circle,
/// matching cairo's `arc` + `stroke` semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct StrokeCircle {
    /// Circle whose outline is stroked.
    pub circle: Circle<Pixels>,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on the outline.
    pub line_width: Pixels<f32>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokeCircle {
    /// Creates a circle stroke spec.
    pub fn new(circle: Circle<Pixels>, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            circle,
            color: color.into(),
            line_width,
            alpha: alpha.into(),
        }
    }
}
