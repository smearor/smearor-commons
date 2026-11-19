//! The [`DrawIconOps] implementations for the buffer targets.
//!
//! One file per target type: [`buffer_view_mut`] holds the icon and glyph
//! raster logic plus tests, [`pixel_buffer`] the delegation impl for
//! the owned packed buffer.

mod buffer_view_mut;
mod pixel_buffer;
