use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

/// An aspect ratio - `width / height` of a [`crate::Size`]-like
/// extent (`1.0` = square).
///
/// Wraps the plain `f64` so ratios cannot be confused with other
/// dimensionless scalars such as [`crate::ScaleFactor`]. The value is
/// strictly positive and finite by construction: [`AspectRatio::new`]
/// rejects non-positive and non-finite inputs, so consumers like
/// [`crate::Vec2::unrotated_fit`] need no defensive `aspect <= 0`
/// guard - "invalid" is unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct AspectRatio(f64);

impl AspectRatio {
    /// Square aspect - width equals height (`1:1`).
    pub const ONE: Self = Self(1.0);

    /// Classic TV and monitor aspect `4:3` (~1.333).
    pub const FOUR_THREE: Self = Self(4.0 / 3.0);

    /// Photo aspect `3:2` - 35 mm film and DSLRs.
    pub const THREE_TWO: Self = Self(3.0 / 2.0);

    /// Monitor aspect `5:4` (1280x1024).
    pub const FIVE_FOUR: Self = Self(5.0 / 4.0);

    /// Widescreen aspect `16:9` - HDTV and common displays.
    pub const SIXTEEN_NINE: Self = Self(16.0 / 9.0);

    /// Widescreen aspect `16:10` - workstation displays.
    pub const SIXTEEN_TEN: Self = Self(16.0 / 10.0);

    /// Ultrawide aspect `21:9` (~2.333; panel-natively often `64:27`).
    pub const TWENTY_ONE_NINE: Self = Self(21.0 / 9.0);

    /// ISO 216 paper aspect `1:sqrt(2)` - DIN A series (~1.414). The
    /// half/hold property: halving a sheet preserves the aspect.
    pub const ISO_PAPER: Self = Self(std::f64::consts::SQRT_2);

    /// Golden ratio `phi` = (1 + sqrt(5)) / 2 (~1.618).
    pub const GOLDEN: Self = Self(std::f64::consts::GOLDEN_RATIO);

    /// Creates an aspect ratio, rejecting non-positive and non-finite
    /// ratios: a degenerate extent has no meaningful aspect.
    pub fn new(ratio: f64) -> Option<Self> {
        (ratio.is_finite() && ratio > 0.0).then_some(Self(ratio))
    }

    /// The ratio `width / height` (`1.0` = square).
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The ratio as `f32` for `f32`-based framework APIs (e.g. GTK4).
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }

    /// Whether the ratio is `1.0` - width equals height.
    pub fn is_square(self) -> bool {
        self.0 == 1.0
    }

    /// The reciprocal `height / width` - the aspect of the same extent
    /// rotated a quarter turn (`a.inverse().inverse() == a`). The
    /// named constants are landscape; `inverse()` yields their
    /// portrait form.
    pub const fn inverse(self) -> Self {
        Self(1.0 / self.0)
    }
}

impl Default for AspectRatio {
    /// Square aspect.
    fn default() -> Self {
        Self::ONE
    }
}

/// Deserialization validates: a stored ratio must satisfy the same
/// `> 0` and finite invariant as [`AspectRatio::new`] - serde cannot
/// smuggle in an invalid value.
impl<'de> Deserialize<'de> for AspectRatio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        Self::new(f64::deserialize(deserializer)?).ok_or_else(|| D::Error::custom("aspect ratio must be positive and finite"))
    }
}

impl From<AspectRatio> for f64 {
    fn from(ratio: AspectRatio) -> Self {
        ratio.0
    }
}

/// No `From<f64>`: raw ratios must go through the validating
/// [`AspectRatio::new`] instead.
impl TryFrom<f64> for AspectRatio {
    type Error = ();

    fn try_from(ratio: f64) -> Result<Self, Self::Error> {
        Self::new(ratio).ok_or(())
    }
}

impl Display for AspectRatio {
    /// Formats as `width:height` (`1.5` prints as `1.5:1`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:1", self.0)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for AspectRatio {
    const SIGNATURE: &'static zvariant::Signature = <f64 as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(AspectRatio);

#[cfg(test)]
mod tests {
    use super::AspectRatio;

    /// Construction enforces the `> 0` and finite invariant.
    #[test]
    fn new_rejects_invalid_ratios() {
        assert_eq!(AspectRatio::new(0.0), None);
        assert_eq!(AspectRatio::new(-2.0), None);
        assert_eq!(AspectRatio::new(f64::NAN), None);
        assert_eq!(AspectRatio::new(f64::INFINITY), None);
        assert_eq!(AspectRatio::new(2.5).map(AspectRatio::as_f64), Some(2.5));
    }

    /// `a.inverse().inverse()` lands back at `a`; `a * a.inverse()`
    /// composes to the square aspect.
    #[test]
    fn inverse_flips_orientation() {
        let wide = AspectRatio::new(5.25);
        let narrow = wide.map(AspectRatio::inverse);
        assert_eq!(narrow.map(AspectRatio::as_f64), Some(1.0 / 5.25));
        assert_eq!(narrow.map(AspectRatio::inverse), wide);
    }

    /// The default aspect is square.
    #[test]
    fn default_is_square() {
        assert_eq!(AspectRatio::default(), AspectRatio::ONE);
        assert!(AspectRatio::ONE.is_square());
    }

    /// The named constants hold their nominal ratios; `inverse()`
    /// yields the portrait form.
    #[test]
    fn named_ratios_match_their_fractions() {
        assert_eq!(AspectRatio::FOUR_THREE.as_f64(), 4.0 / 3.0);
        assert_eq!(AspectRatio::SIXTEEN_NINE.as_f64(), 16.0 / 9.0);
        assert_eq!(AspectRatio::SIXTEEN_TEN.as_f64(), 1.6);
        assert_eq!(AspectRatio::ISO_PAPER.as_f64(), std::f64::consts::SQRT_2);
        assert_eq!(AspectRatio::SIXTEEN_NINE.inverse().as_f64(), 9.0 / 16.0);
    }

    /// Halving an ISO sheet along its long edge preserves the aspect:
    /// `sqrt(2) = 2 / sqrt(2)`.
    #[test]
    fn iso_paper_halving_preserves_aspect() {
        let a4 = AspectRatio::ISO_PAPER;
        let a5 = a4.inverse().as_f64() * 2.0;
        assert!((a5 - a4.as_f64()).abs() < 1e-9);
    }
}
