/// A perceived luminance value (0.0-1.0), clamped on construction.
///
/// Newtype so a luminance threshold cannot be confused with a
/// channel value or a percent. Returned by
/// [`crate::Color::luminance_709`] and [`crate::Color::luminance_601`].
use std::ops::Div;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Luminance(f64);

impl Luminance {
    /// Minimum luminance (black).
    pub const MIN: Self = Self(0.0);

    /// Maximum luminance (white).
    pub const MAX: Self = Self(1.0);

    /// Creates a luminance value, clamped to `0.0-1.0`.
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

/// Saturating difference - a negative result clamps to [`Luminance::MIN`].
impl Sub for Luminance {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

/// Ratio of two luminances as a plain factor (e.g. a blend amount).
impl Div for Luminance {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl From<f64> for Luminance {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<Luminance> for f64 {
    fn from(luminance: Luminance) -> Self {
        luminance.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamps_to_unit_range() {
        assert_eq!(Luminance::new(1.5).as_f64(), 1.0);
        assert_eq!(Luminance::new(-0.5).as_f64(), 0.0);
        assert_eq!(Luminance::MIN.as_f64(), 0.0);
        assert_eq!(Luminance::MAX.as_f64(), 1.0);
    }

    #[test]
    fn sub_saturates_at_min() {
        let delta = Luminance::new(0.5) - Luminance::new(0.2);
        assert!((delta.as_f64() - 0.3).abs() < 1e-9);
        assert_eq!(Luminance::new(0.2) - Luminance::new(0.5), Luminance::MIN);
    }

    #[test]
    fn div_yields_ratio() {
        assert!((Luminance::new(0.5) / Luminance::new(0.25) - 2.0).abs() < 1e-9);
        assert_eq!(Luminance::new(0.5) / Luminance::MAX, 0.5);
    }
}
