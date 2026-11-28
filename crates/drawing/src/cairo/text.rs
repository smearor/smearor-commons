//! The [`Text`] draw spec.

use crate::cairo::ToyFont;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Radians;
use smearor_commons_geometry::Vec2;

/// A rotated, centered text label spec: `text` drawn in `font` and
/// `color`, centered at `at` and rotated by `angle` - the toy-font
/// counterpart of the primitive specs, bundling what the toy API
/// leaves on the context (`select_font_face`, `set_font_size`,
/// `set_source`) into one value.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// The label text.
    pub text: String,
    /// Toy-font selection (face, size, slant, weight).
    pub font: ToyFont,
    /// Point the label is centered on before `angle` rotates it.
    pub at: Vec2<Pixels>,
    /// Rotation of the baseline around `at` (`Radians::ZERO` for
    /// upright text).
    pub angle: Radians,
    /// Text color.
    pub color: Color<f64>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl Text {
    /// Creates a text spec.
    pub fn new(text: impl Into<String>, font: ToyFont, at: Vec2<Pixels>, angle: Radians, color: impl Into<Color<f64>>, alpha: impl Into<Alpha>) -> Self {
        Self {
            text: text.into(),
            font,
            at,
            angle,
            color: color.into(),
            alpha: alpha.into(),
        }
    }
}
