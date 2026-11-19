//! Shared rendering vocabulary on pixel buffers.
//!
//! The vocabulary is split into per-capability traits —
//! [`DrawPrimitiveOps`], [`DrawIconOps`], [`DrawTextOps`] and
//! [`DrawProgressOps`] — so backends can implement them incrementally
//! and consumers can bound on exactly the capabilities they need.
//! [`DrawOps`] bundles all of them as a supertrait for full-vocabulary
//! bounds. All traits are implemented for `BufferViewMut`<`Rgba`> — the
//! strided render target — and for `PixelBuffer`<`Rgba`>, which
//! delegates to its packed view. [`RenderContext`] is the builder facade
//! over a buffer target.
//!
//! Fonts are injected as `&FontVec` parameters; font loading and caching
//! stay with the consumer. Icon-name resolution likewise stays a
//! caller-provided `resolve_codepoint` closure. Where a meaningful
//! fallback exists the parameter is `Option<&FontVec>`: icons fall back
//! to [`DrawIconOps::draw_icon_placeholder`], text to the built-in
//! bitmap renderer.
//!
//! Import [`prelude`] to bring all capability traits into scope at once.
//!
//! # Internal layering
//!
//! - [`ops`]: the public, backend-neutral vocabulary (capability traits).
//! - [`tech`]: render-technology adapters — crate-internal wrappers over
//!   external rendering tech (e.g. `ab_glyph` glyph metrics) that any
//!   backend may use; not a backend itself.
//! - [`buffer`]: the software-raster backend — one directory module per
//!   capability (`buffer_view_mut` works, `pixel_buffer` delegates) plus
//!   its private machinery in `buffer/raster`.
//! - [`RenderContext`]: the public facade, generic over [`DrawTarget`].

mod buffer;
mod ops;
mod render_context;
mod tech;

pub use ops::DrawIconOps;
pub use ops::DrawOps;
pub use ops::DrawPrimitiveOps;
pub use ops::DrawProgressOps;
pub use ops::DrawTarget;
pub use ops::DrawTextOps;
pub use render_context::RenderContext;

/// Re-exports all drawing capability traits — `use
/// smearor_commons_drawing::prelude::*;` brings the full vocabulary into
/// scope for method calls.
pub mod prelude {
    pub use crate::ops::DrawIconOps;
    pub use crate::ops::DrawOps;
    pub use crate::ops::DrawPrimitiveOps;
    pub use crate::ops::DrawProgressOps;
    pub use crate::ops::DrawTarget;
    pub use crate::ops::DrawTextOps;
}
