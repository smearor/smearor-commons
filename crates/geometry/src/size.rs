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

/// Convenience geometry accessors for `gtk4::Widget`s.
///
/// Import this trait to call [`WidgetSizeExt::allocated_size`] on any
/// widget. Named after `gtk_widget_get_allocated_*` rather than `size`,
/// which collides with `gtk4::prelude::WidgetExt::size`.
#[cfg(feature = "gtk4")]
pub trait WidgetSizeExt {
    /// The widget's allocated pixel extent.
    fn allocated_size(&self) -> Size<i32>;

    /// The widget's measured minimum and natural pixel extents as
    /// `(minimum, natural)`, measured unconstrained (`for_size = -1`)
    /// in both orientations.
    fn measure_size(&self) -> (Size<i32>, Size<i32>);

    /// The center of the widget's allocated area in pixel
    /// coordinates - the natural origin for pointer hit tests and
    /// radial layouts.
    fn center(&self) -> Vec2<crate::Pixels, f64> {
        self.allocated_size().with_unit::<Pixels>().to_f64().half()
    }
}

#[cfg(feature = "gtk4")]
impl<W: gtk4::prelude::IsA<gtk4::Widget>> WidgetSizeExt for W {
    fn allocated_size(&self) -> Size<i32> {
        use gtk4::prelude::Cast;
        use gtk4::prelude::WidgetExt;
        let widget = self.upcast_ref::<gtk4::Widget>();
        Size::new(widget.width(), widget.height())
    }

    fn measure_size(&self) -> (Size<i32>, Size<i32>) {
        use gtk4::prelude::Cast;
        use gtk4::prelude::WidgetExt;
        let widget = self.upcast_ref::<gtk4::Widget>();
        let (min_w, nat_w, _, _) = widget.measure(gtk4::Orientation::Horizontal, -1);
        let (min_h, nat_h, _, _) = widget.measure(gtk4::Orientation::Vertical, -1);
        (Size::new(min_w, min_h), Size::new(nat_w, nat_h))
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
