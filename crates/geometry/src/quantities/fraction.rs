use crate::SmoothingAlpha;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

/// A normalized progress or blend fraction, conventionally
/// `0.0..=1.0`.
///
/// `Fraction` is *not* clamped at the type level: easing curves
/// deliberately extrapolate (an overshoot eases past `1.0`) and
/// each consumer decides what values outside the interval mean.
/// Call [`Self::clamped`] where the closed interval is required.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Fraction(pub f64);

impl Fraction {
    /// No progress - the `from` end of an interpolation.
    pub const ZERO: Self = Self(0.0);

    /// The midpoint.
    pub const HALF: Self = Self(0.5);

    /// Full progress - the `to` end of an interpolation.
    pub const ONE: Self = Self(1.0);

    /// Creates a fraction.
    pub const fn new(fraction: f64) -> Self {
        Self(fraction)
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The fraction clamped to the closed interval `0.0..=1.0` -
    /// required where extrapolated progress is not meaningful.
    pub fn clamped(self) -> Self {
        Self(self.0.clamp(0.0, 1.0))
    }

    /// The complement `1 - self` - the weight of `from` when `self`
    /// is the weight of `to` in a blend.
    pub fn inverse(self) -> Self {
        Self(1.0 - self.0)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Fraction {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self(self.0 * factor)
    }
}

impl Mul<Fraction> for f64 {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        rhs * self
    }
}

/// Adding a fractional contribution to a raw scalar yields a raw
/// scalar - `from + (to - from) * t`, the shape of every lerp:
/// `(scalar * Fraction) -> Fraction`, then `scalar + Fraction`.
impl Add<Fraction> for f64 {
    type Output = f64;

    fn add(self, fraction: Fraction) -> f64 {
        self + fraction.0
    }
}

impl Div<f64> for Fraction {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self(self.0 / divisor)
    }
}

impl Div<Fraction> for Fraction {
    type Output = f64;

    /// Same-unit ratio - the dimensionless scalar.
    fn div(self, rhs: Fraction) -> f64 {
        self.0 / rhs.0
    }
}

impl From<f64> for Fraction {
    fn from(fraction: f64) -> Self {
        Self(fraction)
    }
}

impl From<Fraction> for f64 {
    fn from(fraction: Fraction) -> Self {
        fraction.0
    }
}

/// A progress fraction is a valid blend weight - EMA smoothing is
/// interpolation along the shortest arc.
impl From<Fraction> for SmoothingAlpha {
    fn from(fraction: Fraction) -> Self {
        Self::new(fraction.0)
    }
}

/// An EMA blend weight is a fraction of `new`.
impl From<SmoothingAlpha> for Fraction {
    fn from(alpha: SmoothingAlpha) -> Self {
        Self(alpha.as_f64())
    }
}

impl Display for Fraction {
    /// Formats as a bare scalar - the unit is dimensionless.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    /// `clamped` pulls extrapolated values back into the interval.
    #[test]
    fn clamped_pulls_into_the_interval() {
        assert_eq!(Fraction::new(1.3).clamped(), Fraction::ONE);
        assert_eq!(Fraction::new(-0.2).clamped(), Fraction::ZERO);
        assert_eq!(Fraction::HALF.clamped(), Fraction::HALF);
    }

    /// `inverse` is the `from`-weight of the same blend.
    #[test]
    fn inverse_complements_to_one() {
        assert_eq!(Fraction::new(0.25).inverse().as_f64(), 0.75);
        assert_eq!(Fraction::ZERO.inverse(), Fraction::ONE);
    }
}
