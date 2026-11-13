//! `Position<T>` - the unit-erased point form for framework boundaries.
//!
//! `Position<T>` is an alias of [`Vec2`]`<(), T>`: a 2D point that
//! deliberately carries no coordinate-space claim. It converts into any
//! framework space at the boundary - smithay `Point<i32, K>` of any
//! space kind `K`, graphene `f32` points - while domain code works on
//! the unit-tagged [`Vec2`]`<U>` forms. Because `Position` and
//! [`crate::Size`] share the same underlying type, a position can be
//! translated by an extent (`position + size`), mirroring smithay's
//! `Point + Size` semantics.

use crate::Vec2;

/// A 2D point on an integer or float coordinate grid, generic over the
/// numeric type `T`. Alias of [`Vec2`]`<(), T>` - the unit-erased
/// boundary form of the geometry model.
pub type Position<T> = Vec2<(), T>;

// ---------------------------------------------------------------------------
// smithay gate — utils::Point conversions
//
// `Position` is space-erased: it converts into `Point<N, K>` for *any*
// coordinate space `K` (Logical, Physical, Buffer, ...). Pinning the
// space is the consumer's responsibility at the conversion site.
// ---------------------------------------------------------------------------

/// The `i32` position as a smithay point of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<Position<i32>> for smithay::utils::Point<i32, K> {
    fn from(position: Position<i32>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// The `i32` position as a smithay point of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<&Position<i32>> for smithay::utils::Point<i32, K> {
    fn from(position: &Position<i32>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// The `f64` position as a smithay point of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<Position<f64>> for smithay::utils::Point<f64, K> {
    fn from(position: Position<f64>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// The `f64` position as a smithay point of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<&Position<f64>> for smithay::utils::Point<f64, K> {
    fn from(position: &Position<f64>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// A smithay `i32` point as a unit-erased [`Position`].
#[cfg(feature = "smithay")]
impl<K> From<smithay::utils::Point<i32, K>> for Position<i32> {
    fn from(position: smithay::utils::Point<i32, K>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// A smithay `i32` point as a unit-erased [`Position`].
#[cfg(feature = "smithay")]
impl<K> From<&smithay::utils::Point<i32, K>> for Position<i32> {
    fn from(position: &smithay::utils::Point<i32, K>) -> Self {
        Self::new(position.x, position.y)
    }
}

// ---------------------------------------------------------------------------
// gtk4 gate — graphene point conversions
// ---------------------------------------------------------------------------

/// The `f32` position as a `graphene::Point`.
#[cfg(feature = "gtk4")]
impl From<Position<f32>> for gtk4::graphene::Point {
    fn from(position: Position<f32>) -> Self {
        Self::new(position.x, position.y)
    }
}

/// A `graphene::Point` as a unit-erased [`Position`].
#[cfg(feature = "gtk4")]
impl From<gtk4::graphene::Point> for Position<f32> {
    fn from(point: gtk4::graphene::Point) -> Self {
        Self::new(point.x(), point.y())
    }
}
