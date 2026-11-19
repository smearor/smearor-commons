//! The [`DrawTarget`] trait: a full-vocabulary render target with a
//! known pixel extent.

use crate::ops::DrawOps;
use smearor_commons_color::Color;
use smearor_commons_geometry::Size;

/// A render target that implements the full [`DrawOps`] vocabulary and
/// reports its own pixel extent.
///
/// This is the bound [`crate::RenderContext`] places on the target it
/// owns: its convenience methods need the target's size for layout
/// (icon placement, progress bar position). Both buffer-side types
/// implement it — `PixelBuffer` and `BufferViewMut` — and backend
/// targets like `CairoTarget` carry their surface size for it.
pub trait DrawTarget: DrawOps {
    /// The target's pixel dimensions.
    fn size(&self) -> Size<u32>;

    /// The default color for text and icons in the given `is_active`
    /// state. This is the single anchor for the crate's color policy:
    /// the `Option<Color>` parameters of the capability traits resolve
    /// `None` to this value. Backends may override it — a themed
    /// target could derive its defaults from the UI theme.
    fn default_text_color(&self, is_active: bool) -> Color<u8> {
        if is_active {
            Color::<u8>::new_rgba(100, 180, 255, 255)
        } else {
            Color::<u8>::new_rgba(240, 240, 240, 255)
        }
    }
}

impl<T: DrawTarget + ?Sized> DrawTarget for &mut T {
    fn size(&self) -> Size<u32> {
        (**self).size()
    }

    fn default_text_color(&self, is_active: bool) -> Color<u8> {
        (**self).default_text_color(is_active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_buffer::Rgba;
    use smearor_commons_geometry::Size;

    #[test]
    fn default_text_color_active_is_blue() {
        let buf = PixelBuffer::<Rgba>::new(Size::new(4, 4));
        let color = buf.default_text_color(true);
        assert_eq!(color, Color::<u8>::new_rgba(100, 180, 255, 255));
    }

    #[test]
    fn default_text_color_inactive_is_light_gray() {
        let buf = PixelBuffer::<Rgba>::new(Size::new(4, 4));
        let color = buf.default_text_color(false);
        assert_eq!(color, Color::<u8>::new_rgba(240, 240, 240, 255));
    }
}
