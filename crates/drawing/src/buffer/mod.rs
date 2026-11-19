//! The capability trait implementations for the commons buffer types.
//!
//! One directory module per capability: each holds the `BufferViewMut`
//! impl, the `PixelBuffer` delegation impl and the tests for that
//! capability. `raster` holds the internal software-raster machinery
//! that the capability impls share.

mod icon_ops;
mod primitive_ops;
mod progress_ops;
mod raster;
mod text_ops;

use crate::ops::DrawTarget;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Size;

impl DrawTarget for BufferViewMut<'_, Rgba> {
    fn size(&self) -> Size<u32> {
        BufferViewMut::size(self)
    }
}

impl DrawTarget for PixelBuffer<Rgba> {
    fn size(&self) -> Size<u32> {
        PixelBuffer::size(self)
    }
}
