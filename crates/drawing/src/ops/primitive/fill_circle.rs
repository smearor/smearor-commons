//! The [`FillCircle`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;

/// A circle fill's draw specification: geometry, color and alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FillCircle {
    /// Circle to fill.
    pub circle: Circle<Pixels>,
    /// Fill color.
    pub color: Color<f64>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl FillCircle {
    /// Creates a circle fill spec.
    pub fn new(circle: Circle<Pixels>, color: impl Into<Color<f64>>, alpha: impl Into<Alpha>) -> Self {
        Self {
            circle,
            color: color.into(),
            alpha: alpha.into(),
        }
    }
}
