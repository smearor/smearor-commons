use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// A scalar extent in the `U` coordinate space - the one-dimensional
/// counterpart of [`crate::Vec2<U>`]/[`crate::Rect<U>`].
///
/// Scalars in this crate are usually bare `f64` because the unit lives
/// on the container types; `Distance` exists for APIs that want the
/// unit attached to a lone scalar - gaps, radii, margins - so a
/// `Distance<Pixels>` cannot be passed where a
/// `Distance<Millimeters>` is expected. The erased form
/// `Distance<()>` keeps unit-free math available, like `Vec2<()>`.
///
/// The unit marker carries no value: equality, ordering, arithmetic
/// and serialization see only the scalar. Serializes as a bare
/// number.
pub struct Distance<U = ()> {
    /// The scalar value, in unit `U`.
    value: f64,
    /// Unit marker - carries no value.
    marker: PhantomData<U>,
}

impl<U> Distance<U> {
    /// No distance.
    pub const ZERO: Self = Self::new(0.0);

    /// A distance of `value` in unit `U`. The unit is asserted by the
    /// call's context, not checked - like `Vec2::new`.
    pub const fn new(value: f64) -> Self {
        Self { value, marker: PhantomData }
    }

    /// The wrapped scalar - the unit-erased counterpart for scalar
    /// math like `Vec2::from_polar`.
    pub const fn as_f64(self) -> f64 {
        self.value
    }

    /// The smaller of `self` and `other` (`f64::min` semantics).
    pub fn min(self, other: Self) -> Self {
        Self::new(self.value.min(other.value))
    }

    /// The larger of `self` and `other` (`f64::max` semantics).
    pub fn max(self, other: Self) -> Self {
        Self::new(self.value.max(other.value))
    }
}

/// `f64` becomes a `Distance` of whatever unit the context expects -
/// the same escape hatch `Vec2::new` offers for vectors. This is what
/// keeps `f64` call sites compiling against `Distance`-typed APIs.
impl<U> From<f64> for Distance<U> {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

/// Field-wise equality - the unit marker `U` carries no value.
impl<U> PartialEq for Distance<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U> PartialOrd for Distance<U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U> Clone for Distance<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for Distance<U> {}

/// `Debug` shows the scalar - the unit is a compile-time tag.
impl<U> fmt::Debug for Distance<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Distance({:?})", self.value)
    }
}

impl<U> Add for Distance<U> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<U> Sub for Distance<U> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

/// Scaling a distance keeps its unit.
impl<U> Mul<f64> for Distance<U> {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self::new(self.value * factor)
    }
}

/// `factor * distance` - the mirrored spelling of `distance * factor`.
impl<U> Mul<Distance<U>> for f64 {
    type Output = Distance<U>;

    fn mul(self, distance: Distance<U>) -> Distance<U> {
        distance * self
    }
}

/// Dividing by a scalar keeps the unit.
impl<U> Div<f64> for Distance<U> {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        Self::new(self.value / divisor)
    }
}

/// The distance in the opposite direction.
impl<U> Neg for Distance<U> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

/// Serialized as the bare scalar - the unit is implicit in the
/// consumer's coordinate space, like `Rect<U>`.
impl<U> Serialize for Distance<U> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.value)
    }
}

/// Deserialized from the bare scalar.
impl<'de, U> Deserialize<'de> for Distance<U> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(f64::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::Distance;
    use crate::Millimeters;
    use crate::Pixels;

    /// The scalar is reachable both ways; `From<f64>` matches `new`.
    #[test]
    fn new_and_from() {
        assert_eq!(Distance::<Pixels>::new(24.0).as_f64(), 24.0);
        assert_eq!(Distance::<Pixels>::from(24.0), Distance::new(24.0));
    }

    /// Arithmetic keeps the unit on both sides.
    #[test]
    fn arithmetic() {
        let a = Distance::<Pixels>::new(30.0);
        let b = Distance::<Pixels>::new(12.0);
        assert_eq!((a + b).as_f64(), 42.0);
        assert_eq!((a - b).as_f64(), 18.0);
        assert_eq!((a * 2.0).as_f64(), 60.0);
        assert_eq!((2.0 * a).as_f64(), 60.0);
        assert_eq!((a / 3.0).as_f64(), 10.0);
        assert_eq!((-a).as_f64(), -30.0);
    }

    /// Ordering and equality see only the scalar; `()` erasure works.
    #[test]
    fn ordering_and_erasure() {
        assert!(Distance::<Pixels>::new(1.0) < Distance::<Pixels>::new(2.0));
        let erased = Distance::<()>::new(5.0);
        assert_eq!(erased, Distance::new(5.0));
        let _millimeters = Distance::<Millimeters>::new(5.0); // distinct type from Distance<Pixels>
    }

    /// The wire form is the bare number.
    #[test]
    fn serde_is_bare_number() {
        let distance = Distance::<Pixels>::new(24.0);
        let json = serde_json::to_string(&distance).expect("serialize");
        assert_eq!(json, "24.0");
        assert_eq!(serde_json::from_str::<Distance<Pixels>>(&json).expect("deserialize"), distance);
    }
}
