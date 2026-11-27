//! The [`StrokePolyline`] draw spec.

use crate::ops::Cap;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polyline;

/// A polyline stroke's draw specification: geometry, color, stroke
/// width and alpha. The stroke is centered on the segments with the
/// given `line_width`; the path stays open — closing it is the
/// caller's choice of repeating the first point.
#[derive(Debug, Clone, PartialEq)]
pub struct StrokePolyline {
    /// Polyline whose segments are stroked.
    pub polyline: Polyline<Pixels>,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on the path.
    pub line_width: Pixels<f32>,
    /// End style of the open path (`Cap::Butt` by default).
    pub cap: Cap,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokePolyline {
    /// Creates a polyline stroke spec.
    pub fn new(polyline: Polyline<Pixels>, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            polyline,
            color: color.into(),
            line_width,
            cap: Cap::default(),
            alpha: alpha.into(),
        }
    }

    /// Sets the end style of the open path (`Cap::Round` for
    /// semicircle endings).
    pub fn with_cap(mut self, cap: Cap) -> Self {
        self.cap = cap;
        self
    }
}
