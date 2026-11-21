use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

/// A CIELAB perceptual lightness `L*` on the `0.0-100.0` scale.
///
/// `0` is black, `100` is diffuse white. The value is *not* clamped:
/// extrapolated `L*` values beyond the interval occur legitimately
/// for out-of-gamut or HDR colors, so [`Self::new`] stores the value
/// as given and [`Self::clamped`] provides the closed interval.
/// Distinct from the HSL [`crate::Lightness`] (`0.0-1.0` scale) and
/// the Rec. 709 [`crate::Luminance`]. Used by [`crate::Lab`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct LabLightness(f64);

impl LabLightness {
    /// The lightness of black.
    pub const ZERO: Self = Self(0.0);

    /// The lightness of diffuse white.
    pub const WHITE: Self = Self(100.0);

    /// Creates a `L*` value without clamping - extrapolated values
    /// beyond `0.0-100.0` are stored as given.
    pub const fn new(lightness: f64) -> Self {
        Self(lightness)
    }

    /// The value clamped to the conventional `0.0-100.0` interval.
    pub fn clamped(self) -> Self {
        Self(self.0.clamp(0.0, 100.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<f64> for LabLightness {
    fn from(lightness: f64) -> Self {
        Self::new(lightness)
    }
}

impl From<LabLightness> for f64 {
    fn from(lightness: LabLightness) -> Self {
        lightness.0
    }
}

impl Display for LabLightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L*{:.1}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrapolated_values_are_stored_unclamped() {
        assert_eq!(LabLightness::new(120.0).as_f64(), 120.0);
        assert_eq!(LabLightness::new(-5.0).as_f64(), -5.0);
        assert_eq!(LabLightness::new(120.0).clamped().as_f64(), 100.0);
    }
}
