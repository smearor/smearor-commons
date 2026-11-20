//! The cairo backend: [`CairoTarget`] implements the shared capability
//! vocabulary on a `cairo::Context`.
//!
//! Feature-gated behind `cairo` and headless-safe — `cairo-rs` alone
//! pulls no display dependency, so this backend is testable without a
//! compositor. Backend-specific extras stay reachable through
//! [`CairoTarget::context`] instead of widening the shared traits.
//!
//! - `target`: the owning [`CairoTarget`] itself
//! - `surface`: [`ViewSurface`] and readback — bridges buffer memory
//!   and cairo `ImageSurface`s
//! - `path`: generic cairo path building blocks (polyline, spline,
//!   wave, band, rounded rect, tile fill)
//! - `glyph_path`: crate-internal `ab_glyph` outline → cairo path
//!   replay used by the icon and text capability impls
//! - `<cap>_ops`: one file per capability trait — the cairo backend
//!   has a single target type, so no per-target directory split
//! - `error`: [`DrawingError`] for bridging failures outside cairo's
//!   own status model

mod error;
mod glyph_path;
mod icon_ops;
pub mod path;
mod primitive_ops;
mod progress_ops;
mod surface;
mod target;
mod text_ops;

use crate::ops::DrawTarget;
use smearor_commons_geometry::Size;

pub use error::DrawingError;
pub use surface::ViewSurface;
pub use surface::pixel_buffer_from_surface;
pub use target::CairoTarget;

impl DrawTarget for CairoTarget {
    fn size(&self) -> Size<u32> {
        self.size()
    }

    fn status(&self) -> Result<(), Self::Error> {
        self.ctx.status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RenderContext;
    use cairo::Context;
    use cairo::Format;
    use cairo::ImageSurface;
    use smearor_commons_color::Color;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;

    /// `RenderContext` owns the cairo target exactly like it owns
    /// `BufferViewMut` — the generic facade works unchanged on both
    /// backends.
    #[test]
    fn render_context_drives_cairo_target() {
        let size = Size::new(32, 32);
        let surface = ImageSurface::create(Format::ARgb32, 32, 32).unwrap();
        let target = CairoTarget::new(&Context::new(&surface).unwrap(), size);
        {
            let mut ctx = RenderContext::new(target).background(Color::<f64>::new(1.0, 0.0, 0.0));
            ctx.fill_background().unwrap();
            ctx.draw_progress_bar(Fraction::new(0.5), Pixels::new(4.0)).unwrap();
        }

        let mut surface = surface;
        surface.flush();
        let data = surface.data().unwrap();
        // ARgb32 memory order is B,G,R,A — a fully red fill.
        assert_eq!(data[2], 255);
        assert_eq!(data[3], 255);
    }
}
