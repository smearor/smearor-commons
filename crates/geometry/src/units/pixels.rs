use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// A coordinate or extent in screen pixels, generic over the scalar.
///
/// `Pixels` (i.e. `Pixels<f64>`) wraps the plain `f64` so pixel values
/// cannot be confused with millimeter positions, and serves as the
/// unit marker for `Vec2<Pixels>`. Integral instantiations like
/// `Pixels<u32>` carry exact pixel counts - buffer extents, texture
/// dimensions, device coordinates. Serializes as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pixels<T = f64>(pub T);

impl<T> Pixels<T> {
    /// Creates a pixel value.
    pub const fn new(pixels: T) -> Self {
        Self(pixels)
    }

    /// The wrapped scalar.
    pub fn as_raw(self) -> T {
        self.0
    }
}

impl Pixels<f64> {
    /// The numeric value in pixels.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Absolute value.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The smaller of two values (`f64::min` semantics for NaN inputs).
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// The larger of two values (`f64::max` semantics for NaN inputs).
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Clamps the value into the inclusive `[min, max]` range.
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }
}

impl<T: Add<Output = T>> Add for Pixels<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<T: Sub<Output = T>> Sub for Pixels<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Pixels<f64> {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self(self.0 * factor)
    }
}

impl Mul<Pixels<f64>> for f64 {
    type Output = Pixels<f64>;

    fn mul(self, rhs: Pixels<f64>) -> Pixels<f64> {
        rhs * self
    }
}

impl Div<f64> for Pixels<f64> {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self(self.0 / divisor)
    }
}

impl<T: Neg<Output = T>> Neg for Pixels<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl<T: Div<Output = T>> Div<Pixels<T>> for Pixels<T> {
    type Output = T;

    /// Same-unit ratio - the dimensionless scalar.
    fn div(self, rhs: Pixels<T>) -> T {
        self.0 / rhs.0
    }
}

impl<T: Default + Add<Output = T>> Sum for Pixels<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(T::default()), Add::add)
    }
}

impl<T> From<T> for Pixels<T> {
    fn from(pixels: T) -> Self {
        Self(pixels)
    }
}

impl From<Pixels<f64>> for f64 {
    fn from(pixels: Pixels<f64>) -> Self {
        pixels.0
    }
}

impl<T: Display> Display for Pixels<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Pixels;

    #[test]
    fn ops_work() {
        let px = Pixels::new(4.0);
        assert_eq!((-px).as_f64(), -4.0);
        assert_eq!((px / 2.0).as_f64(), 2.0);
        assert_eq!((2.0 * px).as_f64(), 8.0);
        assert_eq!([px, px, px].into_iter().sum::<Pixels>().as_f64(), 12.0);
    }

    /// Integral pixels carry exact counts and compare for equality.
    #[test]
    fn integral_pixels_are_exact() {
        let width = Pixels::new(96u32);
        assert_eq!(width.as_raw(), 96);
        assert_eq!(width + Pixels::new(4u32), Pixels::new(100u32));
    }
}
