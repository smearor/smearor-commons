//! Render-technology adapters shared by backends.
//!
//! This layer sits between the crate root and the backends: each
//! module wraps one external rendering technology (metrics, caches,
//! format conversions) in a crate-internal API that any backend may
//! use. It is not a backend itself and contains no capability impls.

mod glyph;

pub(crate) use glyph::glyph_fit_scale;
