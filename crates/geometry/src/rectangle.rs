//! `Rectangle<T>` - the unit-erased rectangle form for framework
//! boundaries.
//!
//! `Rectangle<T>` is an alias of [`Rect`]`<(), T>`: a positioned extent
//! (`min` corner + `size`, `size.x` = width) that deliberately carries
//! no coordinate-space claim. It converts into graphene `Rect`s and
//! smithay `Rectangle<N, K>`s of any space kind at the boundary.

use crate::Rect;

/// An axis-aligned rectangle on an integer or float coordinate grid,
/// generic over the numeric type `T`. Alias of [`Rect`]`<(), T>` - the
/// unit-erased boundary form of the geometry model.
pub type Rectangle<T> = Rect<(), T>;

// ---------------------------------------------------------------------------
// gtk4 gate — graphene conversions
// ---------------------------------------------------------------------------

/// The `f32` rectangle as a `graphene::Rect`.
#[cfg(feature = "gtk4")]
impl From<Rectangle<f32>> for gtk4::graphene::Rect {
    fn from(rectangle: Rectangle<f32>) -> Self {
        Self::new(rectangle.min.x, rectangle.min.y, rectangle.size.x, rectangle.size.y)
    }
}

/// The `i32` rectangle as a `graphene::Rect`.
#[cfg(feature = "gtk4")]
impl From<Rectangle<i32>> for gtk4::graphene::Rect {
    fn from(rectangle: Rectangle<i32>) -> Self {
        Self::new(rectangle.min.x as f32, rectangle.min.y as f32, rectangle.size.x as f32, rectangle.size.y as f32)
    }
}

// ---------------------------------------------------------------------------
// smithay gate — utils::Rectangle conversions
//
// `Rectangle` is space-erased like [`crate::Position`]: it converts into
// `Rectangle<N, K>` for any coordinate space `K`.
// ---------------------------------------------------------------------------

/// The `i32` rectangle as a smithay rectangle of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<Rectangle<i32>> for smithay::utils::Rectangle<i32, K> {
    fn from(rectangle: Rectangle<i32>) -> Self {
        Self::new(rectangle.min.into(), smithay::utils::Size::new(rectangle.size.x, rectangle.size.y))
    }
}

/// A smithay `i32` rectangle as a unit-erased [`Rectangle`].
#[cfg(feature = "smithay")]
impl<K> From<smithay::utils::Rectangle<i32, K>> for Rectangle<i32> {
    fn from(rectangle: smithay::utils::Rectangle<i32, K>) -> Self {
        let position: smithay::utils::Point<i32, K> = rectangle.loc;
        Self::new(position.into(), crate::Size::new(rectangle.size.w, rectangle.size.h))
    }
}

/// The `f32` rectangle as a smithay rectangle of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<Rectangle<f32>> for smithay::utils::Rectangle<f32, K> {
    fn from(rectangle: Rectangle<f32>) -> Self {
        Self::new(
            smithay::utils::Point::new(rectangle.min.x, rectangle.min.y),
            smithay::utils::Size::new(rectangle.size.x, rectangle.size.y),
        )
    }
}

/// A smithay `f32` rectangle as a unit-erased [`Rectangle`].
#[cfg(feature = "smithay")]
impl<K> From<smithay::utils::Rectangle<f32, K>> for Rectangle<f32> {
    fn from(rectangle: smithay::utils::Rectangle<f32, K>) -> Self {
        Self::new(crate::Position::new(rectangle.loc.x, rectangle.loc.y), crate::Size::new(rectangle.size.w, rectangle.size.h))
    }
}
