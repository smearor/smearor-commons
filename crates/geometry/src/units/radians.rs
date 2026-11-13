use crate::Degrees;
use crate::SmoothingAlpha;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::TAU;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// An angle in radians - the scalar unit for boundaries where angles
/// are expressed in radians (math APIs, `angle_delta` gesture
/// values).
///
/// Unlike [`crate::RotationRad`] - the quantity type with angle
/// semantics (wrapping, `sin_cos`, deltas) - `Radians` is a plain
/// scalar: it carries the unit but no rotation behavior. Serializes
/// as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Radians(pub f64);

impl Radians {
    /// No angle.
    pub const ZERO: Self = Self(0.0);

    /// A full turn (`2*PI` radians) - the modulus of radian-based
    /// angle arithmetic.
    pub const FULL_TURN: Self = Self(TAU);

    /// 30° - an hour mark on a clock face, 12-item radial layouts.
    pub const TWELFTH_TURN: Self = Self(TAU / 12.0);

    /// 45° - a compass diagonal, 8-item radial layouts.
    pub const EIGHTH_TURN: Self = Self(TAU / 8.0);

    /// 60° - an equilateral angle, hexagonal and 6-item radial layouts.
    pub const SIXTH_TURN: Self = Self(TAU / 6.0);

    /// 90° - a right angle, the `QuarterTurn` increments.
    pub const QUARTER_TURN: Self = Self(TAU / 4.0);

    /// 120° - three-way symmetric layouts.
    pub const THIRD_TURN: Self = Self(TAU / 3.0);

    /// 180° - a straight angle, the direction opposite `ZERO`.
    pub const HALF_TURN: Self = Self(TAU / 2.0);

    /// 270° - the last of the `QuarterTurn` increments.
    pub const THREE_QUARTER_TURN: Self = Self(3.0 * TAU / 4.0);
    /// Creates an angle in radians.
    pub const fn new(radians: f64) -> Self {
        Self(radians)
    }

    /// The numeric value in radians.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The value as `f32` for `f32`-based framework APIs (e.g. GTK4).
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }

    /// The equivalent angle in degrees.
    pub fn to_degrees(self) -> Degrees {
        Degrees::new(self.0.to_degrees())
    }

    /// `(sin, cos)` of the angle - trigonometry for transforms and
    /// geometry (the `f64::sin_cos` delegate).
    pub fn sin_cos(self) -> (f64, f64) {
        self.0.sin_cos()
    }

    /// Absolute value.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The value normalized to `[0, 2*PI)` - the canonical
    /// representative of the same direction.
    pub fn normalized(self) -> Self {
        Self(self.0.rem_euclid(Self::FULL_TURN.0))
    }
}

impl Add for Radians {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Radians {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Radians {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self(self.0 * factor)
    }
}

impl Mul<Radians> for f64 {
    type Output = Radians;

    fn mul(self, rhs: Radians) -> Radians {
        rhs * self
    }
}

impl Div<f64> for Radians {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self(self.0 / divisor)
    }
}

impl Div<Radians> for Radians {
    type Output = f64;

    /// Same-unit ratio - the dimensionless scalar.
    fn div(self, rhs: Radians) -> f64 {
        self.0 / rhs.0
    }
}

/// A blend weight scales an angle - the EMA/lerp-weighted
/// counterpart of `Radians * f64`.
impl Mul<SmoothingAlpha> for Radians {
    type Output = Radians;

    fn mul(self, weight: SmoothingAlpha) -> Self {
        self * weight.as_f64()
    }
}

impl Mul<Radians> for SmoothingAlpha {
    type Output = Radians;

    fn mul(self, radians: Radians) -> Self::Output {
        radians * self
    }
}

impl Neg for Radians {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Sum for Radians {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl From<f64> for Radians {
    fn from(radians: f64) -> Self {
        Self(radians)
    }
}

impl From<Radians> for f64 {
    fn from(radians: Radians) -> Self {
        radians.0
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        degrees.to_radians()
    }
}

impl Display for Radians {
    /// Formats with a `rad` suffix (`0.785rad`) - makes the unit
    /// explicit so the number cannot be misread as degrees.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}rad", self.0)
    }
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(Radians);

#[cfg(test)]
mod tests {
    use super::Radians;
    use crate::Degrees;
    use std::f64::consts::PI;
    use std::f64::consts::TAU;

    /// Radian arithmetic and scalar scaling work like the other
    /// scalar units.
    #[test]
    fn ops_work() {
        let rad = Radians::new(PI);
        assert_eq!((-rad).as_f64(), -PI);
        assert_eq!((rad / 2.0).as_f64(), PI / 2.0);
        assert_eq!((2.0 * rad).as_f64(), TAU);
        assert_eq!([rad, rad].into_iter().sum::<Radians>().as_f64(), TAU);
    }

    /// The named turn fractions on `Radians` and `Degrees` agree.
    #[test]
    fn turn_constants_agree_with_degrees() {
        for (radians, degrees) in [
            (Radians::TWELFTH_TURN, 30.0),
            (Radians::EIGHTH_TURN, 45.0),
            (Radians::SIXTH_TURN, 60.0),
            (Radians::QUARTER_TURN, 90.0),
            (Radians::THIRD_TURN, 120.0),
            (Radians::HALF_TURN, 180.0),
            (Radians::THREE_QUARTER_TURN, 270.0),
            (Radians::FULL_TURN, 360.0),
        ] {
            assert!((radians.to_degrees().as_f64() - degrees).abs() < 1e-9, "{radians} should be {degrees} degrees");
            assert_eq!(radians.to_degrees(), Degrees::new(degrees).to_radians().to_degrees());
        }
    }

    /// `PI` radians are 180 degrees; normalization wraps into the turn.
    #[test]
    fn converts_and_normalizes() {
        let degrees = Radians::new(PI).to_degrees();
        assert!((degrees.as_f64() - 180.0).abs() < 1e-9);
        assert_eq!(Radians::new(TAU + PI).normalized().as_f64(), PI);
        assert!((Radians::new(-PI / 2.0).normalized().as_f64() - 3.0 * PI / 2.0).abs() < 1e-9);
    }
}
