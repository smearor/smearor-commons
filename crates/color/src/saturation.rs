use serde::Deserialize;
use serde::Serialize;
use std::ops::Div;
use std::ops::Sub;

/// A saturation value (0.0-1.0), clamped on construction.
///
/// `0.0` is gray (achromatic), `1.0` is full chroma. Newtype so a
/// saturation cannot be confused with a channel value or a percent.
/// Used by [`crate::Hsv`] and [`crate::Hsl`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Saturation(f64);

impl Saturation {
    /// Minimum saturation (achromatic).
    pub const MIN: Self = Self(0.0);

    /// Maximum saturation (full chroma).
    pub const MAX: Self = Self(1.0);

    /// Creates a saturation, clamped to `0.0-1.0`.
    pub fn new(saturation: f64) -> Self {
        Self(saturation.clamp(0.0, 1.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

/// Saturating difference - a negative result clamps to [`Saturation::MIN`].
impl Sub for Saturation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

/// Ratio of two saturations as a plain factor.
impl Div for Saturation {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl From<f64> for Saturation {
    fn from(saturation: f64) -> Self {
        Self::new(saturation)
    }
}

impl From<Saturation> for f64 {
    fn from(saturation: Saturation) -> Self {
        saturation.0
    }
}
