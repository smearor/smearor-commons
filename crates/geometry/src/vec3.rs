//! `Vec3` - a point or vector in 3D space tagged with its
//! coordinate unit `U` and numeric component type `T`.

use crate::RotationRad;
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

/// A 3D point or vector tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// The phantom unit distinguishes positions in millimeters
/// (`Vec3<Millimeters>`), raw device units and screen pixels
/// (`Vec3<Pixels>`) so they cannot be mixed up; `T` defaults to `f64`
/// for domain math. The *unit-erased* form `Vec3<(), T>` is the
/// boundary representation - it deliberately carries no
/// coordinate-space claim, mirroring [`crate::Vec2`].
/// Serializes as a flat `{"x": .., "y": .., "z": ..}` map; the unit
/// marker is skipped.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Vec3<U = (), T = f64> {
    /// X coordinate in the unit `U`.
    pub x: T,
    /// Y coordinate in the unit `U`.
    pub y: T,
    /// Z coordinate in the unit `U`.
    pub z: T,
    /// Phantom marker tagging the coordinate unit; not serialized.
    #[serde(skip)]
    unit: PhantomData<U>,
}

impl<U, T: Copy> Copy for Vec3<U, T> {}

impl<U, T: Clone> Clone for Vec3<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Vec3<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<U, T: Eq> Eq for Vec3<U, T> {}

impl<U, T: Hash> Hash for Vec3<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<U, T: Default> Default for Vec3<U, T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<U, T: Display> Display for Vec3<U, T> {
    /// `x,y,z` - the unit marker is not displayed.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl<U, T> Vec3<U, T> {
    /// Creates a new 3D vector in the unit `U`.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z, unit: PhantomData }
    }

    /// Reinterprets this vector in the unit `V` - the components are
    /// unchanged; use this when a framework boundary knows the
    /// coordinate space that the erased `()` tag does not express.
    pub fn with_unit<V>(&self) -> Vec3<V, T>
    where
        T: Clone,
    {
        Vec3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }

    /// Maps the components into another component type `V`, keeping
    /// the unit.
    pub fn map<V>(&self, f: impl Fn(T) -> V) -> Vec3<U, V>
    where
        T: Clone,
    {
        Vec3::new(f(self.x.clone()), f(self.y.clone()), f(self.z.clone()))
    }
}

impl<U> Vec3<U, f64> {
    /// The Euclidean length.
    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y).hypot(self.z)
    }

    /// The Euclidean distance to `other`.
    pub fn distance(&self, other: &Self) -> f64 {
        (*self - *other).magnitude()
    }

    /// The scalar product.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// The cross product `self × other`.
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    /// The unit vector - `(0,0,0)` normalizes to itself.
    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude <= f64::EPSILON {
            return *self;
        }
        *self / magnitude
    }

    /// The vector rotated around the X axis (right-handed: +Y goes
    /// toward +Z).
    pub fn rotated_x(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.as_f64().sin_cos();
        Vec3::new(self.x, self.y * cos - self.z * sin, self.y * sin + self.z * cos)
    }

    /// The vector rotated around the Y axis (right-handed: +Z goes
    /// toward +X, so +X goes toward -Z).
    pub fn rotated_y(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.as_f64().sin_cos();
        Vec3::new(self.x * cos + self.z * sin, self.y, -self.x * sin + self.z * cos)
    }

    /// The vector rotated around the Z axis (right-handed: +X goes
    /// toward +Y).
    pub fn rotated_z(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.as_f64().sin_cos();
        Vec3::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos, self.z)
    }
}

impl<U, T: Add<Output = T>> Add for Vec3<U, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<U, T: AddAssign> AddAssign for Vec3<U, T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<U, T: Sub<Output = T>> Sub for Vec3<U, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<U, T: SubAssign> SubAssign for Vec3<U, T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<U, T: Neg<Output = T>> Neg for Vec3<U, T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<U, T: Copy + Mul<Output = T>> Mul<T> for Vec3<U, T> {
    type Output = Self;

    fn mul(self, factor: T) -> Self {
        Vec3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl<U> Mul<Vec3<U>> for f64 {
    type Output = Vec3<U>;

    fn mul(self, vector: Vec3<U>) -> Self::Output {
        vector * self
    }
}

impl<U, T: Copy + Div<Output = T>> Div<T> for Vec3<U, T> {
    type Output = Self;

    fn div(self, divisor: T) -> Self {
        Vec3::new(self.x / divisor, self.y / divisor, self.z / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn rotated_y_turns_x_toward_negative_z() {
        let rotated = Vec3::<()>::new(1.0, 0.0, 0.0).rotated_y(RotationRad::new(FRAC_PI_2));
        assert!(rotated.x.abs() < 1e-12 && (rotated.z + 1.0).abs() < 1e-12);
    }

    #[test]
    fn rotated_x_turns_y_toward_z() {
        let rotated = Vec3::<()>::new(0.0, 1.0, 0.0).rotated_x(RotationRad::new(FRAC_PI_2));
        assert!(rotated.y.abs() < 1e-12 && (rotated.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn rotated_z_turns_x_toward_y() {
        let rotated = Vec3::<()>::new(1.0, 0.0, 0.0).rotated_z(RotationRad::new(FRAC_PI_2));
        assert!(rotated.x.abs() < 1e-12 && (rotated.y - 1.0).abs() < 1e-12);
    }

    #[test]
    fn cross_of_axes_is_right_handed() {
        let x = Vec3::<()>::new(1.0, 0.0, 0.0);
        let y = Vec3::<()>::new(0.0, 1.0, 0.0);
        assert_eq!(x.cross(&y), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn dot_and_magnitude_and_distance() {
        let a = Vec3::<()>::new(3.0, 4.0, 0.0);
        let b = Vec3::<()>::new(0.0, 0.0, 1.0);
        assert_eq!(a.magnitude(), 5.0);
        assert_eq!(a.dot(&b), 0.0);
        assert_eq!(a.distance(&a), 0.0);
    }

    #[test]
    fn normalized_has_unit_length_and_zero_stays_zero() {
        let n = Vec3::<()>::new(0.0, 3.0, 4.0).normalized();
        assert!((n.magnitude() - 1.0).abs() < 1e-12);
        assert_eq!(Vec3::<()>::new(0.0, 0.0, 0.0).normalized(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn serde_skips_the_unit_marker() {
        let json = serde_json::to_string(&Vec3::<()>::new(1.0, 2.0, 3.0)).unwrap();
        assert_eq!(json, r#"{"x":1.0,"y":2.0,"z":3.0}"#);
        let parsed: Vec3<()> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Vec3::new(1.0, 2.0, 3.0));
    }
}
