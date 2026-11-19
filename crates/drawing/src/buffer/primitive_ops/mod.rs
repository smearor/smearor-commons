//! The [`DrawPrimitiveOps`] implementations for the buffer targets.
//!
//! One file per target type: [`buffer_view_mut`] holds the raster logic
//! (a shared shape-scan helper plus the `BufferViewMut` impl and tests),
//! [`pixel_buffer`] the delegation impl for the owned packed buffer.

mod buffer_view_mut;
mod pixel_buffer;
