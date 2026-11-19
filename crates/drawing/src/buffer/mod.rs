//! The capability trait implementations for the commons buffer types.
//!
//! One file per capability: each holds the `BufferViewMut` impl, the
//! `PixelBuffer` delegation impl and the tests for that capability.
//! `raster` holds the internal software-raster machinery
//! ([`RasterOps`]) that the capability impls share.

mod icon_ops;
mod primitive_ops;
mod progress_ops;
mod raster;
mod text_ops;
