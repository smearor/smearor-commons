use serde::Deserialize;
use serde::Serialize;
use std::ops::Div;
use std::ops::Sub;

/// An HSL lightness (0.0-1.0), clamped on construction.
///
/// `0.0` is black, `0.5` is the pure hue, `1.0` is white. Newtype so
/// a lightness cannot be confused with a channel value or a percent.
/// Distinct from the perceptual [`crate::Luminance`], which carries
/// Rec. 709 weights rather than the symmetric HSL mid-range. Used by
/// [`crate::Hsl`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Lightness(f64);

impl Lightness {
    /// Minimum lightness (black).
    pub const MIN: Self = Self(0.0);

    /// Maximum lightness (white).
    pub const MAX: Self = Self(1.0);

    /// Creates a lightness, clamped to `0.0-1.0`.
    pub fn new(lightness: f64) -> Self {
        Self(lightness.clamp(0.0, 1.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

/// Saturating difference - a negative result clamps to [`Lightness::MIN`].
impl Sub for Lightness {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

/// Ratio of two lightnesses as a plain factor.
impl Div for Lightness {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl From<f64> for Lightness {
    fn from(lightness: f64) -> Self {
        Self::new(lightness)
    }
}

impl From<Lightness> for f64 {
    fn from(lightness: Lightness) -> Self {
        lightness.0
    }
}
