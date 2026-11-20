//! The [`FillPolygon`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;

/// A polygon fill's draw specification: geometry, color and alpha.
/// The fill covers the even-odd interior of the vertex ring, matching
/// [`Polygon::contains`](smearor_commons_geometry::Polygon) semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct FillPolygon {
    /// Polygon to fill.
    pub polygon: Polygon<Pixels>,
    /// Fill color.
    pub color: Color<f64>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl FillPolygon {
    /// Creates a polygon fill spec.
    pub fn new(polygon: Polygon<Pixels>, color: impl Into<Color<f64>>, alpha: impl Into<Alpha>) -> Self {
        Self {
            polygon,
            color: color.into(),
            alpha: alpha.into(),
        }
    }
}
