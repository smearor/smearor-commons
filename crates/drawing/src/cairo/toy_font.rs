//! The [`ToyFont`] font selection.

use cairo::FontSlant;
use cairo::FontWeight;
use smearor_commons_geometry::Pixels;

/// A cairo toy-font selection: `face` family at `size` pixels with
/// `slant`/`weight` - the `select_font_face` + `set_font_size`
/// configuration as a value (system font names like `"Sans"`).
#[derive(Debug, Clone, PartialEq)]
pub struct ToyFont {
    /// Font family name as cairo's toy API resolves it (`"Sans"`,
    /// `"Serif"`, `"Monospace"`, ...).
    pub face: String,
    /// Font size in pixels.
    pub size: Pixels<f32>,
    /// Font slant (`FontSlant::Normal`, `Italic`, `Oblique`).
    pub slant: FontSlant,
    /// Font weight (`FontWeight::Normal`, `Bold`).
    pub weight: FontWeight,
}

impl ToyFont {
    /// Creates a toy-font selection.
    pub fn new(face: impl Into<String>, size: Pixels<f32>, slant: FontSlant, weight: FontWeight) -> Self {
        Self {
            face: face.into(),
            size,
            slant,
            weight,
        }
    }
}
