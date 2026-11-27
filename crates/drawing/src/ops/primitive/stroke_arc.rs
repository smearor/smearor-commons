//! The [`StrokeArc`] draw spec.

use crate::ops::Cap;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;

/// A circular-arc stroke's draw specification: the `arc` interval of
/// `circle`'s rim stroked with `line_width` (centered on the rim) -
/// the level-bar / progress-ring primitive. End style via `cap`.
#[derive(Debug, Clone, PartialEq)]
pub struct StrokeArc {
    /// Circle whose rim carries the stroked arc.
    pub circle: Circle<Pixels>,
    /// The stroked angular interval of the rim.
    pub arc: Arc,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on the rim.
    pub line_width: Pixels<f32>,
    /// End style at both ends of the interval (`Cap::Butt` default).
    pub cap: Cap,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokeArc {
    /// Creates a stroked-arc spec.
    pub fn new(circle: Circle<Pixels>, arc: Arc, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            circle,
            arc,
            color: color.into(),
            line_width,
            cap: Cap::default(),
            alpha: alpha.into(),
        }
    }

    /// Sets the end style (`Cap::Round` for semicircle endings - the
    /// progress-bar look).
    pub fn with_cap(mut self, cap: Cap) -> Self {
        self.cap = cap;
        self
    }
}
