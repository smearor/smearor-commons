//! `Size<T>` - the unit-erased extent form for framework boundaries.
//!
//! `Size<T>` is an alias of [`Vec2`]`<(), T>`: a width/height pair
//! (`size.x` = width, `size.y` = height, same convention as
//! [`crate::Rect`]'s `size` field) that deliberately carries no
//! coordinate-space claim. It converts into smithay `Size<i32, K>` of
//! any space kind or describes texture extents at the GTK boundary.

use crate::Vec2;

/// A 2D extent on an integer or float coordinate grid, generic over the
/// numeric type `T`. Alias of [`Vec2`]`<(), T>` - the unit-erased
/// boundary form; `x` is the width, `y` the height.
pub type Size<T> = Vec2<(), T>;

// ---------------------------------------------------------------------------
// gtk4 gate — texture extents
// ---------------------------------------------------------------------------

/// The pixel extent of a `gdk::Texture`.
#[cfg(feature = "gtk4")]
impl From<&gtk4::gdk::Texture> for Size<i32> {
    fn from(texture: &gtk4::gdk::Texture) -> Self {
        use gtk4::prelude::TextureExt;
        Self::new(texture.width(), texture.height())
    }
}

// ---------------------------------------------------------------------------
// smithay gate — utils::Size conversions
//
// `Size` is space-erased like [`crate::Position`]: it converts into
// `Size<N, K>` for any coordinate space `K`.
// ---------------------------------------------------------------------------

/// The `i32` size as a smithay size of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<Size<i32>> for smithay::utils::Size<i32, K> {
    fn from(size: Size<i32>) -> Self {
        Self::new(size.x, size.y)
    }
}

/// The `i32` size as a smithay size of the given space kind.
#[cfg(feature = "smithay")]
impl<K> From<&Size<i32>> for smithay::utils::Size<i32, K> {
    fn from(size: &Size<i32>) -> Self {
        Self::new(size.x, size.y)
    }
}

/// A smithay `i32` size as a unit-erased [`Size`].
#[cfg(feature = "smithay")]
impl<K> From<smithay::utils::Size<i32, K>> for Size<i32> {
    fn from(size: smithay::utils::Size<i32, K>) -> Self {
        Self::new(size.w, size.h)
    }
}

/// A smithay `i32` size as a unit-erased [`Size`].
#[cfg(feature = "smithay")]
impl<K> From<&smithay::utils::Size<i32, K>> for Size<i32> {
    fn from(size: &smithay::utils::Size<i32, K>) -> Self {
        Self::new(size.w, size.h)
    }
}
