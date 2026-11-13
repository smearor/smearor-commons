use crate::Millimeters;
use crate::Pixels;
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

/// A 2D point or vector tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// The phantom unit distinguishes positions in millimeters
/// (`Vec2<Millimeters>`), raw device units and screen pixels
/// (`Vec2<Pixels>`) so they cannot be mixed up; `T` defaults to `f64`
/// for domain math but can be any component type the boundaries need
/// (`Vec2<(), i32>` for smithay `Point<i32, K>` and textures,
/// `Vec2<(), f32>` for graphene). The *unit-erased* form `Vec2<(), T>`
/// is the boundary representation - it deliberately carries no
/// coordinate-space claim and converts into any framework space;
/// [`crate::Position`] and [`crate::Size`] are readable aliases for it.
/// Serializes as a flat `{"x": .., "y": ..}` map; the unit marker is
/// skipped. With the `async-graphql` feature, `Vec2<Millimeters>` is
/// exposed as the GraphQL object type `Point`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2<U = (), T = f64> {
    /// X coordinate in the unit `U`.
    pub x: T,
    /// Y coordinate in the unit `U`.
    pub y: T,
    /// Phantom marker tagging the coordinate unit; not serialized.
    #[serde(skip)]
    unit: PhantomData<U>,
}

/// `Vec2<Millimeters>` is exposed as the GraphQL object type `Point`.
/// A `SimpleObject` derive cannot express the doubly-generic struct
/// (it would require `T: OutputType` on every instantiation), so the
/// concrete object is declared as resolvers on the `f64` millimeter
/// instantiation only.
#[cfg(feature = "async-graphql")]
#[async_graphql::Object(name = "Point")]
impl Vec2<Millimeters> {
    /// X coordinate in millimeters.
    async fn x(&self) -> f64 {
        self.x
    }

    /// Y coordinate in millimeters.
    async fn y(&self) -> f64 {
        self.y
    }
}

// Manual impls: `PhantomData<U>` carries no value, so `Vec2<U>` must not
// impose `Copy`/`Clone`/`PartialEq`/`Eq`/`Hash` bounds on `U`.
impl<U, T: Copy> Copy for Vec2<U, T> {}

impl<U, T: Clone> Clone for Vec2<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Vec2<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<U, T: Eq> Eq for Vec2<U, T> {}

impl<U, T: Hash> Hash for Vec2<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<U, T> Vec2<U, T> {
    /// Creates a new 2D point in the unit `U`.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y, unit: PhantomData }
    }

    /// Applies `f` to each component - component-wise transforms like
    /// clamping, rounding or numeric conversion. The component type may
    /// change (`v.map(|c| c.round() as i32)` yields `Vec2<U, i32>`); the
    /// unit `U` is always preserved.
    pub fn map<V>(&self, f: impl Fn(T) -> V) -> Vec2<U, V>
    where
        T: Copy,
    {
        Vec2::new(f(self.x), f(self.y))
    }

    /// Component-wise product (Hadamard product): each axis scaled by the
    /// other vector's matching component - per-axis weights, not a dot
    /// product.
    pub fn component_mul(&self, other: &Vec2<U, T>) -> Self
    where
        T: Copy + Mul<Output = T>,
    {
        Self::new(self.x * other.x, self.y * other.y)
    }

    /// Component-wise maximum of two points in the same unit.
    pub fn max(&self, other: &Self) -> Self
    where
        T: Copy + Ord,
    {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Component-wise minimum of two points in the same unit.
    pub fn min(&self, other: &Self) -> Self
    where
        T: Copy + Ord,
    {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// The vector rotated 90° counterclockwise: `(-y, x)` - the
    /// perpendicular used for offset and normal computations.
    pub fn perpendicular(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Self::new(-self.y, self.x)
    }

    /// The axis-aligned rectangle anchored at this point with the given
    /// `size` extent.
    pub fn rect(&self, size: Vec2<U, T>) -> crate::Rect<U, T>
    where
        T: Copy,
    {
        crate::Rect::new(*self, size)
    }
}

impl<U, T: Default> Default for Vec2<U, T> {
    /// The origin `(0, 0)` - both components `T::default()`.
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<U, T: Display> Display for Vec2<U, T> {
    /// The `(x,y)` spelling of the point.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// ---------------------------------------------------------------------------
// `f32` / `i32` numeric bridges (boundary integer and float coordinates)
// ---------------------------------------------------------------------------

impl<U> Vec2<U, f32> {
    /// Creates a point from `u32` components.
    pub fn new_from_u32(x: u32, y: u32) -> Self {
        Self::new(x as f32, y as f32)
    }

    /// Creates a point from `i32` components.
    pub fn new_from_i32(x: i32, y: i32) -> Self {
        Self::new(x as f32, y as f32)
    }

    /// Rounds to the nearest integer (standard for display coordinates).
    pub fn to_i32_round(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.round() as i32, self.y.round() as i32)
    }

    /// Rounds down (for clipping/bounding boxes, to not exceed any pixels).
    pub fn to_i32_floor(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.floor() as i32, self.y.floor() as i32)
    }

    /// Rounds up (for damage tracking, to capture all affected pixels).
    pub fn to_i32_ceil(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.ceil() as i32, self.y.ceil() as i32)
    }
}

impl<U> Vec2<U, i32> {
    /// The point as `f32` components.
    pub fn to_f32(&self) -> Vec2<U, f32> {
        Vec2::new(self.x as f32, self.y as f32)
    }

    /// The point as `f64` components.
    pub fn to_f64(&self) -> Vec2<U, f64> {
        Vec2::new(self.x as f64, self.y as f64)
    }
}

impl<U> Vec2<U, f32> {
    /// The point as `f64` components.
    pub fn to_f64(&self) -> Vec2<U, f64> {
        Vec2::new(self.x as f64, self.y as f64)
    }
}

impl<U> Vec2<U, f64> {
    /// Rounds to the nearest integer (standard for display coordinates).
    pub fn to_i32_round(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.round() as i32, self.y.round() as i32)
    }

    /// Rounds down (for clipping/bounding boxes, to not exceed any pixels).
    pub fn to_i32_floor(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.floor() as i32, self.y.floor() as i32)
    }

    /// Rounds up (for damage tracking, to capture all affected pixels).
    pub fn to_i32_ceil(&self) -> Vec2<U, i32> {
        Vec2::new(self.x.ceil() as i32, self.y.ceil() as i32)
    }

    /// The point as `f32` components.
    pub fn to_f32(&self) -> Vec2<U, f32> {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

// ---------------------------------------------------------------------------
// `f64` domain math - only available for the default component type
// ---------------------------------------------------------------------------

impl<U> Vec2<U, f64> {
    /// The origin `(0, 0)` in the unit `U`.
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        unit: PhantomData,
    };

    /// The x-component as a scalar in the unit `U` - the typed
    /// counterpart of the raw `x` field for boundaries that need the
    /// unit attached (`v.x` is `f64`, `v.x_unit()` is `U`).
    pub fn x_unit(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.x)
    }

    /// The y-component as a scalar in the unit `U` - see
    /// [`Self::x_unit`].
    pub fn y_unit(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.y)
    }

    /// Both components set to the same value - `Vec2::splat(margin)` is
    /// the vector `(margin, margin)` (e.g. a uniform margin around a
    /// rectangle).
    pub fn splat(value: U) -> Self
    where
        U: Into<f64>,
    {
        let v = value.into();
        Self::new(v, v)
    }

    /// Creates a vector from polar coordinates: `radius` in the unit `U`
    /// at `angle` (counter-clockwise from the positive x-axis).
    pub fn from_polar(radius: f64, angle: RotationRad) -> Self {
        let (sin, cos) = angle.sin_cos();
        Vec2::new(radius * cos, radius * sin)
    }

    /// Euclidean distance to another point in the same unit.
    pub fn distance(&self, other: &Vec2<U>) -> U
    where
        U: From<f64>,
    {
        U::from(self.distance_f64(other))
    }

    /// Euclidean distance to another point, as a plain `f64`.
    ///
    /// Unlike [`Vec2::distance`] this needs no unit-carrying result
    /// type, so it also works on the unit-erased `Vec2<()>` where the
    /// distance itself has no meaningful unit.
    pub fn distance_f64(&self, other: &Vec2<U>) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Dot product `x·x + y·y` of two vectors in the same unit.
    ///
    /// The caller picks the product type `T` (like [`Vec2::distance`]'s
    /// unit return): `f64` for unit-free math or a unit type such as
    /// [`crate::SquareMillimeters`] for `Vec2<Millimeters>` products.
    pub fn dot<T: From<f64>>(&self, other: &Vec2<U>) -> T {
        T::from(self.x * other.x + self.y * other.y)
    }

    /// 2D determinant `x·y' − y·x'` (z component of the 3D cross
    /// product): positive when `other` turns left from `self`, negative
    /// right; zero for (anti-)parallel vectors.
    ///
    /// Same caller-chosen product type as [`Vec2::dot`].
    pub fn cross<T: From<f64>>(&self, other: &Vec2<U>) -> T {
        T::from(self.x * other.y - self.y * other.x)
    }

    /// Euclidean magnitude of the vector, in the unit `U`.
    pub fn magnitude(&self) -> U
    where
        U: From<f64>,
    {
        self.distance(&Vec2::ZERO)
    }

    /// Euclidean magnitude of the vector, as a plain `f64` - the
    /// unit-erased counterpart of [`Vec2::magnitude`] for `Vec2<()>`.
    pub fn magnitude_f64(&self) -> f64 {
        self.distance_f64(&Vec2::ZERO)
    }

    /// The unit vector pointing the same way; a vector shorter
    /// than `1e-9` normalizes to itself.
    pub fn normalized(&self) -> Self {
        let magnitude = self.x.hypot(self.y);
        if magnitude < 1e-9 {
            return Vec2::new(self.x, self.y);
        }
        Vec2::new(self.x / magnitude, self.y / magnitude)
    }

    /// The direction the vector points, as a `RotationRad` angle -
    /// the inverse of [`Vec2::from_polar`].
    pub fn direction(&self) -> RotationRad {
        RotationRad::new(self.y.atan2(self.x))
    }

    /// The vector rotated by `rotation`.
    pub fn rotated(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.as_f64().sin_cos();
        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// The bounding-box size of an axis-aligned rectangle of this size
    /// rotated by `rotation` around its center - the forward map
    /// `bb(v) = A·v` with `A = [[cos, sin], [sin, cos]]` over the
    /// absolute sine/cosine.
    ///
    /// [`Rect::rotated_bounds`](crate::Rect::rotated_bounds) delegates
    /// to this method on its `size`.
    pub fn rotated_bounds(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.sin_cos();
        let (abs_sin, abs_cos) = (sin.abs(), cos.abs());
        Vec2::new(self.x * abs_cos + self.y * abs_sin, self.x * abs_sin + self.y * abs_cos)
    }

    /// Inverse of [`Self::rotated_bounds`]: the axis-aligned size that,
    /// rotated by `rotation`, yields this bounding box - the map
    /// `A⁻¹·v` with `det(A) = cos(2θ)`.
    ///
    /// The determinant vanishes at `45° + k·90°` where no finite
    /// solution exists for non-square boxes; components then hold
    /// `±inf`/`NaN` and callers must apply their own fallback.
    pub fn unrotated_bounds(&self, rotation: RotationRad) -> Self {
        let (sin, cos) = rotation.sin_cos();
        let (abs_sin, abs_cos) = (sin.abs(), cos.abs());
        let determinant = abs_cos * abs_cos - abs_sin * abs_sin;
        Vec2::new((self.x * abs_cos - self.y * abs_sin) / determinant, (self.y * abs_cos - self.x * abs_sin) / determinant)
    }

    /// The largest rectangle of aspect ratio `aspect` (width/height)
    /// whose `rotation`-rotated bounding box fits inside a rectangle of
    /// size `self` - the inscribed, aspect-preserving counterpart of
    /// [`Vec2::unrotated_bounds`].
    ///
    /// Solves `A·(aspect·h, h) <= self` for the largest `h` over the
    /// absolute sin/cos matrix `A`: the tighter of the two axis
    /// constraints wins. Unlike the exact inverse solve this never
    /// diverges and never goes negative - rotating a rectangle by any
    /// angle keeps such an inscribed solution bounded. At `aspect == 1`
    /// it degenerates to the square fit `min(w, h) / (|cos| + |sin|)`.
    ///
    /// A non-positive `aspect` has no geometric meaning and yields
    /// [`Vec2::ZERO`]; negative components of `self` are clamped the
    /// same way.
    pub fn unrotated_fit(&self, aspect: f64, rotation: RotationRad) -> Self {
        if aspect <= 0.0 {
            return Vec2::ZERO;
        }
        let (sin, cos) = rotation.sin_cos();
        let (abs_sin, abs_cos) = (sin.abs(), cos.abs());
        let height = (self.x / (aspect * abs_cos + abs_sin)).min(self.y / (aspect * abs_sin + abs_cos)).max(0.0);
        Vec2::new(aspect * height, height)
    }

    /// Geometric centroid of a non-empty point set.
    ///
    /// Returns [`Vec2::new(0.0, 0.0)`] for an empty slice instead of panicking.
    pub fn centroid(points: &[Vec2<U>]) -> Self {
        let n = points.len() as f64;
        if n == 0.0 {
            return Vec2::new(0.0, 0.0);
        }
        Vec2::new(points.iter().map(|p| p.x).sum::<f64>() / n, points.iter().map(|p| p.y).sum::<f64>() / n)
    }
}

impl Vec2<Millimeters> {
    /// Creates a point from typed millimeter components.
    pub fn from_mm(x: Millimeters, y: Millimeters) -> Self {
        Self::new(x.as_f64(), y.as_f64())
    }
}

impl Vec2<Pixels> {
    /// Creates a point from typed pixel components.
    pub fn from_px(x: Pixels, y: Pixels) -> Self {
        Self::new(x.as_f64(), y.as_f64())
    }
}

// ---------------------------------------------------------------------------
// Arithmetic and conversions (component-type generic)
// ---------------------------------------------------------------------------

impl<U, T: Add<Output = T>> Add for Vec2<U, T> {
    type Output = Self;

    /// Component-wise sum of two points in the same unit.
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<U, T: AddAssign> AddAssign for Vec2<U, T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<U, T: Sub<Output = T>> Sub for Vec2<U, T> {
    type Output = Self;

    /// Component-wise difference of two points in the same unit.
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<U, T: SubAssign> SubAssign for Vec2<U, T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<U, T: Neg<Output = T>> Neg for Vec2<U, T> {
    type Output = Self;

    /// The point mirrored at the origin.
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<U, T: Copy + Mul<Output = T>> Mul<T> for Vec2<U, T> {
    type Output = Self;

    /// The vector scaled by a dimensionless factor.
    fn mul(self, factor: T) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }
}

impl<U> Mul<Vec2<U>> for f64 {
    type Output = Vec2<U>;

    /// The vector scaled by a dimensionless factor.
    fn mul(self, vector: Vec2<U>) -> Vec2<U> {
        vector * self
    }
}

impl<U, T: Copy + Div<Output = T>> Div<T> for Vec2<U, T> {
    type Output = Self;

    /// The vector divided by a dimensionless factor.
    fn div(self, divisor: T) -> Self {
        Self::new(self.x / divisor, self.y / divisor)
    }
}

// Numeric casts between component types, preserving the unit `U`.

impl<U> From<Vec2<U, i32>> for Vec2<U, f32> {
    fn from(v: Vec2<U, i32>) -> Self {
        Self::new(v.x as f32, v.y as f32)
    }
}

impl<U> From<Vec2<U, i32>> for Vec2<U, f64> {
    fn from(v: Vec2<U, i32>) -> Self {
        Self::new(v.x as f64, v.y as f64)
    }
}

impl<U> From<Vec2<U, i32>> for Vec2<U, u32> {
    fn from(v: Vec2<U, i32>) -> Self {
        Self::new(v.x as u32, v.y as u32)
    }
}

impl<U> From<Vec2<U, u32>> for Vec2<U, i32> {
    fn from(v: Vec2<U, u32>) -> Self {
        Self::new(v.x as i32, v.y as i32)
    }
}

impl<U> From<Vec2<U, f64>> for Vec2<U, f32> {
    fn from(v: Vec2<U, f64>) -> Self {
        Self::new(v.x as f32, v.y as f32)
    }
}

impl<U, T> From<Vec2<U, T>> for [T; 2] {
    /// The point as `[x, y]`.
    fn from(v: Vec2<U, T>) -> Self {
        [v.x, v.y]
    }
}

impl<U, T> From<[T; 2]> for Vec2<U, T> {
    /// The pair as a `Vec2` in the unit `U`.
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

// ---------------------------------------------------------------------------
// gtk4 gate — graphene point conversions
//
// Unit-generic: the boundary call interprets the components in GTK's
// coordinate space regardless of the source unit `U` (the caller owns
// that reinterpretation); reading a graphene point back tags it with
// whatever unit the caller assigns.
// ---------------------------------------------------------------------------

/// The `f32` point as a `graphene::Point`.
#[cfg(feature = "gtk4")]
impl<U> From<Vec2<U, f32>> for gtk4::graphene::Point {
    fn from(position: Vec2<U, f32>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// A `graphene::Point` as a `Vec2` in the caller-chosen unit `U`.
#[cfg(feature = "gtk4")]
impl<U> From<gtk4::graphene::Point> for Vec2<U, f32> {
    fn from(point: gtk4::graphene::Point) -> Self {
        Self::new(point.x(), point.y())
    }
}

/// Serde adapter for `#[serde(with)]`: `Vec2<U>` as a compact
/// `[x, y]` pair instead of the default `{"x": .., "y": ..}` map.
///
/// Dataset-style formats like figure profiles or game maps often
/// prefer the tighter pair shape; downstream crates use this module as
/// `#[serde(with = "smearor_commons_geometry::serde_pair")]`.
pub mod serde_pair {
    use super::Vec2;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    /// Serializes the point as `[x, y]`.
    pub fn serialize<U, S: Serializer>(v: &Vec2<U>, serializer: S) -> Result<S::Ok, S::Error> {
        [v.x, v.y].serialize(serializer)
    }

    /// Deserializes `[x, y]` into a `Vec2` in the unit `U`.
    pub fn deserialize<'de, U, D: Deserializer<'de>>(deserializer: D) -> Result<Vec2<U>, D::Error> {
        <[f64; 2]>::deserialize(deserializer).map(Vec2::from)
    }
}

/// Serde adapter for `#[serde(with)]`: `Vec<Vec2<U>>` as a compact
/// `[[x, y], ...]` list - the polygon form of [`serde_pair`].
pub mod serde_points {
    use super::Vec2;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    /// Serializes the points as `[[x, y], ...]`.
    pub fn serialize<U, S: Serializer>(points: &[Vec2<U>], serializer: S) -> Result<S::Ok, S::Error> {
        points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>().serialize(serializer)
    }

    /// Deserializes `[[x, y], ...]` into `Vec2`s in the unit `U`.
    pub fn deserialize<'de, U, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Vec2<U>>, D::Error> {
        <Vec<[f64; 2]>>::deserialize(deserializer).map(|pairs| pairs.into_iter().map(Vec2::from).collect())
    }
}

#[cfg(test)]
mod tests {
    /// `map` transforms each component and may change the numeric type;
    /// `component_mul` scales per axis.
    #[test]
    fn map_and_component_mul() {
        let v = Vec2::<Pixels>::new(-3.0, 4.0);
        assert_eq!(v.map(|c| c.max(0.0)).x, 0.0);
        assert_eq!(v.map(|c| c.max(0.0)).y, 4.0);
        let w = Vec2::<Pixels>::new(2.0, 0.5).component_mul(&v);
        assert_eq!((w.x, w.y), (-6.0, 2.0));
        let ints = v.map(|c| c.round() as i32);
        assert_eq!(ints, Vec2::<Pixels, i32>::new(-3, 4));
    }

    /// `splat` fills both components from one unit value.
    #[test]
    fn splat_fills_both_axes() {
        let v = Vec2::<Pixels>::splat(Pixels::new(7.5));
        assert_eq!((v.x, v.y), (7.5, 7.5));
    }

    /// `x()`/`y()` wrap the raw components in the unit type.
    #[test]
    fn components_as_units() {
        let v = Vec2::<Pixels>::new(3.0, 4.0);
        assert_eq!(v.x_unit(), Pixels::new(3.0));
        assert_eq!(v.y_unit(), Pixels::new(4.0));
    }

    use super::Vec2;
    use crate::Millimeters;
    use crate::Pixels;
    use crate::Position;
    use crate::RotationRad;
    use crate::Size;

    #[test]
    fn distance_of_axis_aligned_points() {
        let a = Vec2::<Millimeters>::new(0.0, 0.0);
        let b = Vec2::<Millimeters>::new(3.0, 4.0);
        assert!((a.distance(&b).as_f64() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn distance_f64_on_unit_erased_vectors() {
        let a = Vec2::<()>::new(1.0, 0.0);
        let b = Vec2::<()>::new(0.0, 1.0);
        assert!((a.distance_f64(&b) - 2.0f64.sqrt()).abs() < 1e-9);
        assert!((a.distance_f64(&a)).abs() < 1e-9);
        assert!((a.magnitude_f64() - 1.0).abs() < 1e-9);
        assert!((b.magnitude_f64() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn centroid_of_triangle() {
        let points = [
            Vec2::<Millimeters>::new(0.0, 0.0),
            Vec2::<Millimeters>::new(6.0, 0.0),
            Vec2::<Millimeters>::new(0.0, 6.0),
        ];
        let c = Vec2::centroid(&points);
        assert!((c.x - 2.0).abs() < 1e-9);
        assert!((c.y - 2.0).abs() < 1e-9);
    }

    #[test]
    fn dot_and_cross_of_perpendiculars() {
        let right = Vec2::<Millimeters>::new(1.0, 0.0);
        let up = Vec2::<Millimeters>::new(0.0, 1.0);
        let dot: f64 = right.dot(&up);
        assert!(dot.abs() < 1e-9);
        let (cross, reverse): (f64, f64) = (right.cross(&up), up.cross(&right));
        assert!((cross - 1.0).abs() < 1e-9);
        assert!((reverse - -1.0).abs() < 1e-9);
        let area: crate::SquareMillimeters = right.cross(&up);
        assert!((area.as_f64() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn dot_and_cross_of_diagonal() {
        let diagonal = Vec2::<Millimeters>::new(1.0, 1.0);
        let axis = Vec2::<Millimeters>::new(1.0, 0.0);
        let (dot, cross): (f64, f64) = (diagonal.dot(&axis), diagonal.cross(&axis));
        assert!((dot - 1.0).abs() < 1e-9);
        assert!((cross - -1.0).abs() < 1e-9);
    }

    #[test]
    fn centroid_of_empty_set_is_origin() {
        let empty: Vec<Vec2<Millimeters>> = Vec::new();
        assert_eq!(Vec2::centroid(&empty), Vec2::new(0.0, 0.0));
    }

    /// Vectors add, subtract, negate and scale component-wise.
    #[test]
    fn arithmetic_ops() {
        let a = Vec2::<Millimeters>::new(10.0, 4.0);
        let b = Vec2::<Millimeters>::new(2.0, 1.0);
        assert_eq!(a + b, Vec2::new(12.0, 5.0));
        assert_eq!(a - b, Vec2::new(8.0, 3.0));
        assert_eq!(-b, Vec2::new(-2.0, -1.0));
        assert_eq!(a * 2.0, Vec2::new(20.0, 8.0));
        assert_eq!(2.0 * b, Vec2::new(4.0, 2.0));
        assert_eq!(a / 2.0, Vec2::new(5.0, 2.0));
    }

    /// Integer vectors share the same arithmetic.
    #[test]
    fn arithmetic_ops_i32() {
        let mut a = Position::<i32>::new(10, 4);
        let b = Size::<i32>::new(2, 1);
        a += b;
        assert_eq!(a, Position::new(12, 5));
        a -= b;
        assert_eq!(a, Position::new(10, 4));
        assert_eq!(a + b, Position::new(12, 5));
        assert_eq!(a - b, Position::new(8, 3));
        assert_eq!(-b, Vec2::<(), i32>::new(-2, -1));
        assert_eq!(a * 2, Vec2::<(), i32>::new(20, 8));
        assert_eq!(a / 2, Vec2::<(), i32>::new(5, 2));
    }

    /// Boundary numerics: `f32` rounds to `i32` in all three modes and
    /// `i32` widens back to `f32` - unit `()` is preserved.
    #[test]
    fn numeric_bridges() {
        let v = Position::<f32>::new(10.4, 20.6);
        assert_eq!(v.to_i32_round(), Position::new(10, 21));
        assert_eq!(v.to_i32_floor(), Position::new(10, 20));
        assert_eq!(v.to_i32_ceil(), Position::new(11, 21));
        let i = Position::<i32>::new(10, 20);
        assert_eq!(i.to_f32(), Position::new(10.0, 20.0));
        assert_eq!(Position::<f32>::new_from_u32(3, 4), Position::new(3.0, 4.0));
        assert_eq!(Position::<f32>::new_from_i32(-1, 2), Position::new(-1.0, 2.0));
    }

    /// `f64` points round to `i32` in all three modes - the unit `U`
    /// is preserved, so `Vec2<Pixels>` stays tagged at the boundary.
    #[test]
    fn f64_to_i32_rounding() {
        let v = Vec2::<Pixels>::new(10.4, 20.6);
        assert_eq!(v.to_i32_round(), Vec2::<Pixels, i32>::new(10, 21));
        assert_eq!(v.to_i32_floor(), Vec2::<Pixels, i32>::new(10, 20));
        assert_eq!(v.to_i32_ceil(), Vec2::<Pixels, i32>::new(11, 21));
        assert_eq!(v.to_i32_round().to_f64(), Vec2::<Pixels>::new(10.0, 21.0));
        assert_eq!(v.to_f32().to_i32_round(), Vec2::<Pixels, i32>::new(10, 21));
    }

    /// Graphene points convert both ways for any unit - the caller
    /// owns the space claim at the boundary.
    #[cfg(feature = "gtk4")]
    #[test]
    fn graphene_point_conversions() {
        let p = Vec2::<Pixels, f32>::new(10.5, 20.5);
        let g: gtk4::graphene::Point = p.into();
        assert_eq!((g.x(), g.y()), (10.5, 20.5));
        let back: Vec2<Pixels, f32> = g.into();
        assert_eq!(back, p);
    }

    /// `max`/`min` combine component-wise for integer coordinates.
    #[test]
    fn componentwise_max_min() {
        let a = Position::<i32>::new(10, 50);
        let b = Position::<i32>::new(30, 20);
        assert_eq!(a.max(&b), Position::new(30, 50));
        assert_eq!(a.min(&b), Position::new(10, 20));
    }

    /// A point anchors a rectangle of the same unit and numeric type.
    #[test]
    fn point_anchors_rect() {
        let p = Position::<i32>::new(10, 20);
        let r = p.rect(Size::new(100, 200));
        assert_eq!(r.min, p);
        assert_eq!(r.size.x, 100);
    }

    /// `magnitude` measures the vector's length; `perpendicular`
    /// turns it 90° left.
    #[test]
    fn magnitude_and_perpendicular() {
        let vector = Vec2::<Millimeters>::new(3.0, 4.0);
        assert_eq!(vector.magnitude(), Millimeters::new(5.0));
        assert_eq!(vector.perpendicular(), Vec2::new(-4.0, 3.0));
        assert_eq!(vector.normalized(), Vec2::new(0.6, 0.8));
        assert_eq!(Vec2::<Millimeters>::ZERO.normalized(), Vec2::ZERO);
        assert_eq!(vector.direction().as_f64(), 4.0_f64.atan2(3.0));
        let rotated = vector.rotated(RotationRad::new(std::f64::consts::FRAC_PI_2));
        assert!((rotated.x - -4.0).abs() < 1e-9 && (rotated.y - 3.0).abs() < 1e-9);
    }

    /// `rotated` applies `R(theta)`: identity at `0`, the
    /// perpendicular at `pi/2`, negation at `pi`, and the full
    /// circle lands back on the vector.
    #[test]
    fn rotated_by_quarter_turns() {
        let vector = Vec2::<Millimeters>::new(3.0, 4.0);
        let pi = std::f64::consts::PI;
        assert_eq!(vector.rotated(RotationRad::ZERO), vector);
        assert!(vector.rotated(RotationRad::new(pi / 2.0)).distance(&vector.perpendicular()).as_f64() < 1e-9);
        assert!(vector.rotated(RotationRad::new(pi)).distance(&-vector).as_f64() < 1e-9);
        assert!(vector.rotated(RotationRad::new(2.0 * pi)).distance(&vector).as_f64() < 1e-9);
    }

    /// `rotated` is an isometry: length preserved, direction
    /// shifted by exactly `theta`.
    #[test]
    fn rotated_is_an_isometry() {
        let vector = Vec2::<Millimeters>::new(3.0, 4.0);
        let rotated = vector.rotated(RotationRad::new(0.7));
        assert!((rotated.magnitude().as_f64() - vector.magnitude().as_f64()).abs() < 1e-9);
        assert!((rotated.direction().as_f64() - (vector.direction().as_f64() + 0.7)).abs() < 1e-9);
    }

    /// Rotations compose additively: `rotated(a).rotated(b)`
    /// equals `rotated(a + b)`.
    #[test]
    fn rotated_composes_additively() {
        let vector = Vec2::<Millimeters>::new(3.0, 4.0);
        let (a, b) = (RotationRad::new(0.3), RotationRad::new(0.4));
        let stepwise = vector.rotated(a).rotated(b);
        let combined = vector.rotated(a + b);
        assert!(stepwise.distance(&combined).as_f64() < 1e-9);
    }

    /// `unrotated_bounds` inverts `rotated_bounds` outside the
    /// singular angles.
    #[test]
    fn unrotated_bounds_inverts_rotated_bounds() {
        let size = Vec2::<Pixels>::new(10.0, 6.0);
        let rotation = RotationRad::from_degrees(30.0);
        let back = size.rotated_bounds(rotation).unrotated_bounds(rotation);
        assert!(back.x - size.x < 1e-9 && size.x - back.x < 1e-9);
        assert!(back.y - size.y < 1e-9 && size.y - back.y < 1e-9);
    }

    /// At `45°` the determinant `cos(2θ)` vanishes and the inverse
    /// solve diverges to ~1e16 components.
    #[test]
    fn unrotated_bounds_diverges_at_singularity() {
        let solved = Vec2::<Pixels>::new(10.0, 6.0).unrotated_bounds(RotationRad::from_degrees(45.0));
        assert!(solved.x.abs() > 1e10 || solved.y.abs() > 1e10 || solved.x.is_nan() || solved.y.is_nan());
    }

    /// At zero rotation the inscribed fit is the plain aspect fit:
    /// the tighter axis constraint wins.
    #[test]
    fn unrotated_fit_at_zero_rotation_is_aspect_fit() {
        let fit = Vec2::<Pixels>::new(100.0, 30.0).unrotated_fit(2.0, RotationRad::ZERO);
        assert!((fit.x - 60.0).abs() < 1e-9 && (fit.y - 30.0).abs() < 1e-9);
    }

    /// At the 45° singularity `unrotated_fit` still returns a bounded,
    /// positive size - for a square aspect it is the square fit.
    #[test]
    fn unrotated_fit_stays_bounded_at_singularity() {
        let fit = Vec2::<Pixels>::new(10.0, 6.0).unrotated_fit(1.0, RotationRad::from_degrees(45.0));
        let expected = 6.0 / (2.0_f64.sqrt());
        assert!((fit.x - expected).abs() < 1e-9 && (fit.y - expected).abs() < 1e-9);
    }

    /// The rotated bounds of the inscribed fit never exceed the
    /// target, for a sweep of angles and aspects.
    #[test]
    fn unrotated_fit_rotated_bounds_stay_inside() {
        let target = Vec2::<Pixels>::new(100.0, 30.0);
        for &degrees in &[12.3, 30.0, 45.0, 77.2, 90.0, 150.0] {
            for &aspect in &[0.5, 1.0, 2.0, 4.0] {
                let rotation = RotationRad::from_degrees(degrees);
                let bounds = target.unrotated_fit(aspect, rotation).rotated_bounds(rotation);
                assert!(bounds.x <= target.x + 1e-9 && bounds.y <= target.y + 1e-9, "deg {degrees} aspect {aspect}: {bounds:?}");
                assert!(bounds.x >= 0.0 && bounds.y >= 0.0);
            }
        }
    }

    /// A non-positive aspect has no geometric meaning.
    #[test]
    fn unrotated_fit_rejects_non_positive_aspect() {
        assert_eq!(Vec2::<Pixels>::new(10.0, 6.0).unrotated_fit(0.0, RotationRad::ZERO), Vec2::ZERO);
        assert_eq!(Vec2::<Pixels>::new(10.0, 6.0).unrotated_fit(-2.0, RotationRad::ZERO), Vec2::ZERO);
    }

    /// `direction` reads the atan2 angle of each axis and quadrant.
    #[test]
    fn direction_of_axes_and_quadrants() {
        let pi = std::f64::consts::PI;
        assert_eq!(Vec2::<Millimeters>::new(1.0, 0.0).direction().as_f64(), 0.0);
        assert_eq!(Vec2::<Millimeters>::new(0.0, 1.0).direction().as_f64(), pi / 2.0);
        assert_eq!(Vec2::<Millimeters>::new(-1.0, 0.0).direction().as_f64(), pi);
        assert_eq!(Vec2::<Millimeters>::new(0.0, -1.0).direction().as_f64(), -pi / 2.0);
        assert_eq!(Vec2::<Millimeters>::new(1.0, 1.0).direction().as_f64(), pi / 4.0);
        assert_eq!(Vec2::<Millimeters>::new(-1.0, 1.0).direction().as_f64(), 3.0 * pi / 4.0);
        // the degenerate case has no direction - atan2(0, 0) yields 0
        assert_eq!(Vec2::<Millimeters>::ZERO.direction().as_f64(), 0.0);
    }

    /// `normalized` yields a unit vector in the same direction.
    #[test]
    fn normalized_preserves_direction() {
        let vector = Vec2::<Millimeters>::new(-6.0, 8.0);
        let unit = vector.normalized();
        assert!((unit.x.hypot(unit.y) - 1.0).abs() < 1e-9);
        // no component along the perpendicular - the direction is unchanged
        assert!(unit.dot::<f64>(&vector.perpendicular()).abs() < 1e-9);
        // scaling the unit vector back up reproduces the original
        assert!((unit * vector.magnitude().as_f64() - vector).magnitude().as_f64() < 1e-9);
    }

    /// Vectors below the `1e-9` threshold normalize to themselves -
    /// no direction can be read off a degenerate vector.
    #[test]
    fn normalized_keeps_degenerate_vectors() {
        assert_eq!(Vec2::<Millimeters>::ZERO.normalized(), Vec2::ZERO);
        let tiny = Vec2::<Millimeters>::new(1e-12, 0.0);
        assert_eq!(tiny.normalized(), tiny);
        // just above the threshold still normalizes to unit length
        let small = Vec2::<Millimeters>::new(1e-6, 0.0);
        assert_eq!(small.normalized(), Vec2::new(1.0, 0.0));
    }

    /// `from_mm`/`from_px` wrap typed components without
    /// `as_f64` at the call site.
    #[test]
    fn from_typed_components() {
        let mm = Vec2::<Millimeters>::from_mm(Millimeters::new(3.0), Millimeters::new(4.0));
        assert_eq!((mm.x, mm.y), (3.0, 4.0));
        let px = Vec2::<Pixels>::from_px(Pixels::new(3.0), Pixels::new(4.0));
        assert_eq!((px.x, px.y), (3.0, 4.0));
    }

    #[test]
    fn array_conversions_roundtrip() {
        let v = Vec2::<Millimeters>::new(1.5, -2.25);
        let arr: [f64; 2] = v.into();
        assert_eq!(arr, [1.5, -2.25]);
        assert_eq!(Vec2::<Millimeters>::from(arr), v);
        let ints: Position<i32> = Vec2::<(), i32>::from([3, 4]);
        assert_eq!(ints, Position::new(3, 4));
    }

    #[test]
    fn polar_roundtrip() {
        let v = Vec2::<Pixels>::from_polar(10.0, RotationRad::from_degrees(90.0));
        assert!((v.x).abs() < 1e-10 && (v.y - 10.0).abs() < 1e-10);
        assert!((v.direction().as_f64() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
        let roundtrip = Vec2::<Pixels>::from_polar(10.0, v.direction());
        assert!((roundtrip.x - v.x).abs() < 1e-10 && (roundtrip.y - v.y).abs() < 1e-10);
    }

    /// `Vec2` displays as `(x,y)` for every component type.
    #[test]
    fn display_format() {
        assert_eq!(format!("{}", Position::<i32>::new(10, 20)), "(10,20)");
        assert_eq!(format!("{}", Vec2::<Pixels>::new(1.5, -2.0)), "(1.5,-2)");
    }

    /// Unit-erased `Position`/`Size` are the same type, so a position
    /// can be translated by an extent - mirroring `Point + Size` in
    /// smithay.
    #[test]
    fn position_plus_size_translates() {
        let p = Position::<i32>::new(10, 20);
        let s = Size::<i32>::new(100, 200);
        assert_eq!(p + s, Position::new(110, 220));
    }

    #[test]
    fn serde_pair_serializes_as_pair() {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        struct Row {
            #[serde(with = "crate::serde_pair")]
            pos: Vec2<Millimeters>,
        }
        let row = Row { pos: Vec2::new(80.0, 220.0) };
        let json = serde_json::to_string(&row).unwrap_or_default();
        assert_eq!(json, r#"{"pos":[80.0,220.0]}"#);
        assert_eq!(serde_json::from_str::<Row>(&json).ok(), Some(row));
    }
}
