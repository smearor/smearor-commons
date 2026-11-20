//! The [`FillRect`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;

/// A rect fill's draw specification: geometry, color and alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FillRect {
    /// Rectangle to fill.
    pub rect: Rect<Pixels>,
    /// Fill color.
    pub color: Color<f64>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl FillRect {
    /// Creates a rect fill spec.
    pub fn new(rect: Rect<Pixels>, color: impl Into<Color<f64>>, alpha: impl Into<Alpha>) -> Self {
        Self {
            rect,
            color: color.into(),
            alpha: alpha.into(),
        }
    }
}
