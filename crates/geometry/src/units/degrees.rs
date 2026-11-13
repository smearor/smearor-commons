use crate::Radians;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// An angle in degrees - the scalar unit for boundaries where angles
/// are expressed in degrees (display strings, wire formats, `f32`
/// framework APIs like `gsk` transforms).
///
/// Unlike [`crate::RotationRad`] - the quantity type with angle
/// semantics (wrapping, `sin_cos`, deltas) - `Degrees` is a plain
/// scalar: it carries the unit but no rotation behavior. Serializes
/// as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Degrees(pub f64);

impl Degrees {
    /// No angle.
    pub const ZERO: Self = Self(0.0);

    /// A full turn - the modulus of degree-based angle arithmetic.
    pub const FULL_TURN: Self = Self(360.0);

    /// 30° - an hour mark on a clock face, 12-item radial layouts.
    pub const TWELFTH_TURN: Self = Self(30.0);

    /// 45° - a compass diagonal, 8-item radial layouts.
    pub const EIGHTH_TURN: Self = Self(45.0);

    /// 60° - an equilateral angle, hexagonal and 6-item radial layouts.
    pub const SIXTH_TURN: Self = Self(60.0);

    /// 90° - a right angle, the `QuarterTurn` increments.
    pub const QUARTER_TURN: Self = Self(90.0);

    /// 120° - three-way symmetric layouts.
    pub const THIRD_TURN: Self = Self(120.0);

    /// 180° - a straight angle, the direction opposite `ZERO`.
    pub const HALF_TURN: Self = Self(180.0);

    /// 270° - the last of the `QuarterTurn` increments.
    pub const THREE_QUARTER_TURN: Self = Self(270.0);
    /// Creates an angle in degrees.
    pub const fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    /// The numeric value in degrees.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The value as `f32` for `f32`-based framework APIs (e.g. GTK4).
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }

    /// The equivalent angle in radians.
    pub fn to_radians(self) -> Radians {
        Radians::new(self.0.to_radians())
    }

    /// Absolute value.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The value normalized to `[0, 360)` - the canonical
    /// representative of the same direction.
    pub fn normalized(self) -> Self {
        Self(self.0.rem_euclid(Self::FULL_TURN.0))
    }
}

impl Add for Degrees {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Degrees {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Degrees {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self(self.0 * factor)
    }
}

impl Mul<Degrees> for f64 {
    type Output = Degrees;

    fn mul(self, rhs: Degrees) -> Degrees {
        rhs * self
    }
}

impl Div<f64> for Degrees {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self(self.0 / divisor)
    }
}

impl Div<Degrees> for Degrees {
    type Output = f64;

    /// Same-unit ratio - the dimensionless scalar.
    fn div(self, rhs: Degrees) -> f64 {
        self.0 / rhs.0
    }
}

impl Neg for Degrees {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Sum for Degrees {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl From<f64> for Degrees {
    fn from(degrees: f64) -> Self {
        Self(degrees)
    }
}

impl From<Degrees> for f64 {
    fn from(degrees: Degrees) -> Self {
        degrees.0
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        radians.to_degrees()
    }
}

impl Display for Degrees {
    /// Formats with a degree-sign suffix (`45°`) - makes the unit
    /// explicit so the number cannot be misread as radians.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\u{b0}", self.0)
    }
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(Degrees);

#[cfg(test)]
mod tests {
    use super::Degrees;
    use std::f64::consts::PI;

    /// Degree arithmetic and scalar scaling work like the other
    /// scalar units.
    #[test]
    fn ops_work() {
        let deg = Degrees::new(90.0);
        assert_eq!((-deg).as_f64(), -90.0);
        assert_eq!((deg / 2.0).as_f64(), 45.0);
        assert_eq!((2.0 * deg).as_f64(), 180.0);
        assert_eq!([deg, deg].into_iter().sum::<Degrees>().as_f64(), 180.0);
        assert_eq!(deg / Degrees::new(45.0), 2.0);
    }

    /// `PI` radians are 180 degrees; normalization wraps into the turn.
    #[test]
    fn converts_and_normalizes() {
        let radians = Degrees::new(180.0).to_radians();
        assert!((radians.as_f64() - PI).abs() < 1e-9);
        assert_eq!(Degrees::new(450.0).normalized().as_f64(), 90.0);
        assert_eq!(Degrees::new(-90.0).normalized().as_f64(), 270.0);
    }
}
