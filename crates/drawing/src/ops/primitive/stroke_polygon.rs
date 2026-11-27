//! The [`StrokePolygon`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;

/// A closed polygon outline's draw specification: vertex ring, color,
/// stroke width and alpha. The stroke is centered on each edge and the
/// ring closes with a join - like cairo's `close_path` + `stroke`, in
/// contrast to [`StrokePolyline`](crate::StrokePolyline)'s open caps.
#[derive(Debug, Clone, PartialEq)]
pub struct StrokePolygon {
    /// Polygon whose closed edge ring is stroked.
    pub polygon: Polygon<Pixels>,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on each edge.
    pub line_width: Pixels<f32>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokePolygon {
    /// Creates a polygon stroke spec.
    pub fn new(polygon: Polygon<Pixels>, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            polygon,
            color: color.into(),
            line_width,
            alpha: alpha.into(),
        }
    }
}
