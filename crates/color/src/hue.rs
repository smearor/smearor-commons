use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// A hue angle in degrees (`0-360`, wraps on construction).
///
/// Newtype so a hue cannot be confused with an unwrapped angle or a
/// channel value. Used by [`crate::Hsv`] and [`crate::Hsl`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Hue(f64);

impl Hue {
    /// No hue - also the value achromatic colors decompose to.
    pub const ZERO: Self = Self(0.0);

    /// Creates a hue, wrapped into `0..360`.
    pub fn new(degrees: f64) -> Self {
        Self(degrees.rem_euclid(360.0))
    }

    /// The numeric value in degrees.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl Display for Hue {
    /// The hue in degrees, e.g. `"240.0°"`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°", self.0)
    }
}

impl From<f64> for Hue {
    fn from(degrees: f64) -> Self {
        Self::new(degrees)
    }
}

impl From<Hue> for f64 {
    fn from(hue: Hue) -> Self {
        hue.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_into_the_circle() {
        assert_eq!(Hue::new(370.0).as_f64(), 10.0);
        assert_eq!(Hue::new(-10.0).as_f64(), 350.0);
    }
}
