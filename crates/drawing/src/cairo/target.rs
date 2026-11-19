//! The cairo render target: an owned [`cairo::Context`] handle plus a
//! declared pixel extent.

use crate::ops::DrawBackend;
use cairo::Context;
use smearor_commons_geometry::Size;

/// A cairo render target: owns a [`cairo::Context`] handle plus the
/// pixel extent it draws to.
///
/// Cloning a `Context` is a refcount bump — `CairoTarget` is a value
/// type, deliberately not a borrow wrapper. GTK callbacks hand out
/// `&Context`; [`new`](Self::new) clones that handle, so a target can be
/// constructed inside `snapshot`/`set_draw_func` callbacks without
/// ceremony, and [`RenderContext`](crate::RenderContext) can own it like
/// `BufferViewMut`.
///
/// The extent is *declared*, not read back: not every surface type
/// reports one (recording surfaces are unbounded), and `DrawTarget`
/// needs a reliable `size()`.
///
/// Use [`context`](Self::context) for backend-specific operations that
/// are not part of the shared vocabulary — `save`/`restore`,
/// transforms, patterns, surface sources.
pub struct CairoTarget {
    pub(crate) ctx: Context,
    size: Size<u32>,
}

impl CairoTarget {
    /// Wraps `ctx`, declaring `size` as the drawable pixel extent.
    /// Clones the handle (refcount bump only).
    pub fn new(ctx: &Context, size: Size<u32>) -> Self {
        Self { ctx: ctx.clone(), size }
    }

    /// The wrapped context, for backend-specific operations outside the
    /// shared vocabulary (transforms, `save`/`restore`, patterns).
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    /// The declared drawable pixel extent.
    pub fn size(&self) -> Size<u32> {
        self.size
    }

    /// The deferred cairo status of the wrapped context.
    ///
    /// cairo reports errors lazily: once a surface or context fails,
    /// every subsequent operation is a no-op and `status()` keeps
    /// reporting the sticky error. Draw methods already propagate
    /// `Self::Error`; this is the optional end-of-pass check.
    pub fn status(&self) -> Result<(), cairo::Error> {
        self.ctx.status()
    }
}

impl DrawBackend for CairoTarget {
    /// cairo errors — deferred/sticky by cairo's own error model.
    type Error = cairo::Error;
}
