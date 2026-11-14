use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::Alpha;

/// Alpha/opacity spelled in whole percent (`0-100`), clamped - the
/// `Eq`-capable wire form of an [`Alpha`]: serializes as a bare u8,
/// orders and compares exactly where a fraction cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AlphaPercent(pub u8);

impl AlphaPercent {
    /// Fully opaque.
    pub const OPAQUE: Self = Self(100);

    /// Creates an alpha clamped into the valid range (`0-100`).
    pub const fn new(percent: u8) -> Self {
        Self(if percent > 100 { 100 } else { percent })
    }

    /// The raw percent value.
    pub const fn percent(self) -> u8 {
        self.0
    }

    /// Fraction form, `0.0-1.0`, for renderers (e.g. cairo alpha).
    pub fn as_f64(self) -> f64 {
        Alpha::from_percent(self.0).as_f64()
    }
}

impl Default for AlphaPercent {
    /// Fully opaque - the alpha a canvas has without an explicit
    /// alpha setting.
    fn default() -> Self {
        Self::OPAQUE
    }
}

impl From<u8> for AlphaPercent {
    /// Clamps the percent into the valid range.
    fn from(percent: u8) -> Self {
        Self::new(percent)
    }
}

impl From<AlphaPercent> for u8 {
    fn from(alpha: AlphaPercent) -> Self {
        alpha.0
    }
}

impl From<AlphaPercent> for Alpha {
    /// The technical alpha the percent value spells.
    fn from(percent: AlphaPercent) -> Self {
        Self::from_percent(percent.0)
    }
}

impl From<Alpha> for AlphaPercent {
    /// The percent spelling of the alpha, rounded.
    fn from(alpha: Alpha) -> Self {
        Self::new(alpha.percent())
    }
}

impl fmt::Display for AlphaPercent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Alpha;
    use super::AlphaPercent;

    #[test]
    fn new_clamps_above_hundred() {
        assert_eq!(AlphaPercent::new(150).percent(), 100);
        assert_eq!(AlphaPercent::new(0).percent(), 0);
    }

    #[test]
    fn default_is_opaque() {
        assert_eq!(AlphaPercent::default(), AlphaPercent::OPAQUE);
    }

    #[test]
    fn alpha_round_trip() {
        assert_eq!(AlphaPercent::from(Alpha::new(0.75)).percent(), 75);
        assert!((Alpha::from(AlphaPercent::new(75)).as_f64() - 0.75).abs() < 1e-9);
    }
}
