//! `Vec4` - a point or vector in 4D space tagged with its
//! coordinate unit `U` and numeric component type `T`.

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

/// A 4D point or vector tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// The phantom unit distinguishes coordinate spaces the way
/// [`crate::Vec3`] does; `T` defaults to `f64` for domain math. The
/// *unit-erased* form `Vec4<(), T>` is the boundary representation -
/// it deliberately carries no coordinate-space claim. Four-component
/// vectors also serve as plain data quadruples (homogeneous
/// coordinates, `hash4x` noise outputs). Serializes as a flat
/// `{"x": .., "y": .., "z": .., "w": ..}` map; the unit marker is
/// skipped.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Vec4<U = (), T = f64> {
    /// X component in the unit `U`.
    pub x: T,
    /// Y component in the unit `U`.
    pub y: T,
    /// Z component in the unit `U`.
    pub z: T,
    /// W component in the unit `U`.
    pub w: T,
    /// Phantom marker tagging the coordinate unit; not serialized.
    #[serde(skip)]
    unit: PhantomData<U>,
}

impl<U, T: Copy> Copy for Vec4<U, T> {}

impl<U, T: Clone> Clone for Vec4<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Vec4<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<U, T: Eq> Eq for Vec4<U, T> {}

impl<U, T: Hash> Hash for Vec4<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}

impl<U, T: Default> Default for Vec4<U, T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default(), T::default())
    }
}

impl<U, T: Display> Display for Vec4<U, T> {
    /// `x,y,z,w` - the unit marker is not displayed.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.x, self.y, self.z, self.w)
    }
}

impl<U, T> Vec4<U, T> {
    /// Creates a new 4D vector in the unit `U`.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w, unit: PhantomData }
    }

    /// Reinterprets this vector in the unit `V` - the components are
    /// unchanged; use this when a framework boundary knows the
    /// coordinate space that the erased `()` tag does not express.
    pub fn with_unit<V>(&self) -> Vec4<V, T>
    where
        T: Clone,
    {
        Vec4::new(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
    }

    /// Maps the components into another component type `V`, keeping
    /// the unit.
    pub fn map<V>(&self, f: impl Fn(T) -> V) -> Vec4<U, V>
    where
        T: Clone,
    {
        Vec4::new(f(self.x.clone()), f(self.y.clone()), f(self.z.clone()), f(self.w.clone()))
    }
}

impl<U> Vec4<U, f64> {
    /// The Euclidean length.
    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y).hypot(self.z).hypot(self.w)
    }

    /// The Euclidean distance to `other`.
    pub fn distance(&self, other: &Self) -> f64 {
        (*self - *other).magnitude()
    }

    /// The scalar product.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// The unit vector - the zero vector normalizes to itself.
    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude <= f64::EPSILON {
            return *self;
        }
        *self / magnitude
    }
}

impl<U, T: Add<Output = T>> Add for Vec4<U, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl<U, T: AddAssign> AddAssign for Vec4<U, T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<U, T: Sub<Output = T>> Sub for Vec4<U, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl<U, T: SubAssign> SubAssign for Vec4<U, T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<U, T: Neg<Output = T>> Neg for Vec4<U, T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<U, T: Copy + Mul<Output = T>> Mul<T> for Vec4<U, T> {
    type Output = Self;

    fn mul(self, factor: T) -> Self {
        Vec4::new(self.x * factor, self.y * factor, self.z * factor, self.w * factor)
    }
}

impl<U> Mul<Vec4<U>> for f64 {
    type Output = Vec4<U>;

    fn mul(self, vector: Vec4<U>) -> Self::Output {
        vector * self
    }
}

impl<U, T: Copy + Div<Output = T>> Div<T> for Vec4<U, T> {
    type Output = Self;

    fn div(self, divisor: T) -> Self {
        Vec4::new(self.x / divisor, self.y / divisor, self.z / divisor, self.w / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_and_magnitude_and_distance() {
        let a = Vec4::<()>::new(1.0, 2.0, 2.0, 4.0);
        let b = Vec4::<()>::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(a.magnitude(), 5.0);
        assert_eq!(a.dot(&b), 4.0);
        assert_eq!(a.distance(&a), 0.0);
    }

    #[test]
    fn normalized_has_unit_length_and_zero_stays_zero() {
        let n = Vec4::<()>::new(0.0, 0.0, 3.0, 4.0).normalized();
        assert!((n.magnitude() - 1.0).abs() < 1e-12);
        assert_eq!(Vec4::<()>::new(0.0, 0.0, 0.0, 0.0).normalized(), Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn serde_skips_the_unit_marker() {
        let json = serde_json::to_string(&Vec4::<()>::new(1.0, 2.0, 3.0, 4.0)).unwrap();
        assert_eq!(json, r#"{"x":1.0,"y":2.0,"z":3.0,"w":4.0}"#);
        let parsed: Vec4<()> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }
}
