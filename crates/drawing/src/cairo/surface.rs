//! Bridges between the buffer crate's pixel memory and cairo surfaces.
//!
//! [`ViewSurface`] lets cairo draw directly into a
//! `BufferViewMut<BgraPremultiplied>` — the memory layout cairo calls
//! `ARgb32` — so one buffer can serve both the raster backend and
//! cairo-drawn overlays without copying.
//!
//! The safe [`ImageSurface::create_for_data`] requires `'static` data,
//! which borrowed view memory cannot provide, so construction goes
//! through `create_for_data_unsafe`. The safety invariant is upheld by
//! ownership and field order: the view owns a borrow into caller memory
//! that outlives this struct, `BufferViewMut` never reallocates its
//! pixel memory, and `surface` is declared before `view` so the cairo
//! handle drops first.

use crate::cairo::DrawingError;
use crate::cairo::target::CairoTarget;
use cairo::Format;
use cairo::ImageSurface;
use smearor_commons_buffer::BgraPremultiplied;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_geometry::Size;

/// An `ImageSurface` drawing directly into a borrowed
/// [`BufferViewMut`]'s pixel memory.
///
/// Draw through [`ViewSurface::target`] (or any `Context` on
/// [`ViewSurface::surface`]); write through [`ViewSurface::view_mut`]
/// and call [`ViewSurface::mark_dirty`] afterwards so cairo invalidates
/// its caches for the externally written bytes.
pub struct ViewSurface<'a> {
    /// The cairo handle into `view`'s memory — declared first so it
    /// drops before the view it points into.
    surface: ImageSurface,
    /// The borrowed caller memory backing the surface.
    view: BufferViewMut<'a, BgraPremultiplied>,
}

impl<'a> ViewSurface<'a> {
    /// Wraps `view` in a cairo `ImageSurface` sharing its pixel memory.
    ///
    /// Fails when the view is empty, its stride cannot hold a full row
    /// of `BgraPremultiplied` pixels, or cairo refuses the surface.
    pub fn new(mut view: BufferViewMut<'a, BgraPremultiplied>) -> Result<Self, DrawingError> {
        let size = view.size();
        if size.x == 0 || size.y == 0 {
            return Err(DrawingError::InvalidSurface("empty view"));
        }
        // No stride check needed: `BufferViewMut` guarantees
        // `stride >= width * BYTES_PER_PIXEL` at construction
        // (`BufferMetadata::validate`), and cairo itself rejects
        // undersized strides with `INVALID_STRIDE`.
        let stride = view.stride().as_usize();
        // SAFETY: `data` points into `view`'s borrowed caller memory with at
        // least `height * stride` bytes live. `view` is stored next to the
        // surface and drops after it (field order), so the pointer outlives
        // every cairo reference. `BufferViewMut` never reallocates.
        let surface =
            unsafe { ImageSurface::create_for_data_unsafe(view.data_mut().as_mut_ptr(), Format::ARgb32, size.x as i32, size.y as i32, stride as i32) }?;
        Ok(Self { surface, view })
    }

    /// The wrapped cairo surface.
    pub fn surface(&self) -> &ImageSurface {
        &self.surface
    }

    /// Mutable access to the pixel memory for non-cairo writes; call
    /// [`ViewSurface::mark_dirty`] afterwards.
    pub fn view_mut(&mut self) -> &mut BufferViewMut<'a, BgraPremultiplied> {
        &mut self.view
    }

    /// A fresh [`CairoTarget`] drawing into the shared pixel memory.
    /// Each call creates a new cairo `Context`, so several targets can
    /// compose independently over the same view.
    pub fn target(&self) -> Result<CairoTarget, DrawingError> {
        let context = cairo::Context::new(&self.surface)?;
        Ok(CairoTarget::new(&context, self.view.size()))
    }

    /// Tells cairo the pixel bytes changed outside its control —
    /// required after writes through [`ViewSurface::view_mut`] before
    /// cairo draws or reads again.
    pub fn mark_dirty(&mut self) {
        self.surface.mark_dirty();
    }

    /// Drops the cairo handle and gives the view back to the caller.
    pub fn into_view(self) -> BufferViewMut<'a, BgraPremultiplied> {
        let Self { surface, view } = self;
        drop(surface);
        view
    }
}

/// Copies an `ImageSurface`'s pixels into a packed
/// [`PixelBuffer<BgraPremultiplied>`], honoring the surface's stride
/// (cairo may pad rows beyond `width·4`).
///
/// Fails when the surface is not `ARgb32`, when its pixel data is still
/// borrowed by a live `Context` or surface clone
/// ([`DrawingError::SurfaceBorrowed`]), or on a cairo status error.
/// For GTK textures follow with `.into_unpremultiplied().into_rgba()`.
pub fn pixel_buffer_from_surface(surface: &mut ImageSurface) -> Result<PixelBuffer<BgraPremultiplied>, DrawingError> {
    if surface.format() != Format::ARgb32 {
        return Err(DrawingError::InvalidSurface("expected ARgb32 image surface"));
    }
    surface.flush();
    let width = surface.width() as usize;
    let height = surface.height() as usize;
    let stride = surface.stride() as usize;
    let row_bytes = width * BgraPremultiplied::BYTES_PER_PIXEL;
    if stride < row_bytes {
        return Err(DrawingError::InvalidSurface("stride smaller than one pixel row"));
    }

    let data = surface.data().map_err(|_| DrawingError::SurfaceBorrowed)?;
    let mut buffer = PixelBuffer::<BgraPremultiplied>::new(Size::new(width as u32, height as u32));
    for y in 0..height {
        let src = &data[y * stride..y * stride + row_bytes];
        buffer.pixels_mut()[y * row_bytes..(y + 1) * row_bytes].copy_from_slice(src);
    }
    Ok(buffer)
}

/// Copies a [`PixelBuffer`]'s pixels into a fresh `ARgb32`
/// [`ImageSurface`] — the inverse of [`pixel_buffer_from_surface`].
/// Channels are converted through the [`PixelFormat`] interface:
/// whatever the source format, each pixel is re-encoded as
/// premultiplied BGRA (the ARgb32 memory layout).
///
/// This is the surface upload consumers need for cairo path helpers
/// like `tile_fill` (e.g. a decoded texture tile).
pub fn image_surface_from_buffer<F: PixelFormat>(buffer: &PixelBuffer<F>) -> Result<ImageSurface, DrawingError> {
    let size = buffer.size();
    let mut surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32)?;
    let stride = surface.stride() as usize;
    let row_bytes = size.x as usize * BgraPremultiplied::BYTES_PER_PIXEL;
    {
        let mut data = surface.data().map_err(|_| DrawingError::SurfaceBorrowed)?;
        for (y, row) in buffer.pixels().chunks_exact(size.x as usize * F::BYTES_PER_PIXEL).enumerate() {
            let dst = &mut data[y * stride..y * stride + row_bytes];
            for (s, d) in row.chunks_exact(F::BYTES_PER_PIXEL).zip(dst.as_chunks_mut::<4>().0.iter_mut()) {
                let [r, g, b, a] = F::read_pixel(s);
                BgraPremultiplied::write_pixel(d, r, g, b, a);
            }
        }
    }
    surface.mark_dirty();
    Ok(surface)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::DrawPrimitiveOps;
    use smearor_commons_buffer::BufferMetadata;
    use smearor_commons_buffer::Stride;
    use smearor_commons_color::Color;
    use smearor_commons_geometry::Pixels;

    fn packed_buffer(size: Size<u32>) -> PixelBuffer<BgraPremultiplied> {
        PixelBuffer::new(size)
    }

    #[test]
    fn view_surface_draws_into_shared_memory() {
        let mut buffer = packed_buffer(Size::new(8, 8));
        {
            let view_surface = ViewSurface::new(buffer.view_mut()).unwrap();
            let mut target = view_surface.target().unwrap();
            target.fill_background(Color::<f64>::new(1.0, 0.0, 0.0)).unwrap();
            target.status().unwrap();
            // Return the view so the pixel bytes are inspectable.
            let _ = view_surface.into_view();
        }
        let pixel: [u8; 4] = buffer.pixels()[0..4].try_into().unwrap();
        // ARgb32 memory order is B,G,R,A.
        assert_eq!(pixel, [0, 0, 255, 255]);
    }

    #[test]
    fn view_surface_external_write_requires_mark_dirty() {
        let mut buffer = packed_buffer(Size::new(4, 4));
        {
            let mut view_surface = ViewSurface::new(buffer.view_mut()).unwrap();
            view_surface.view_mut().data_mut()[0] = 0x7F;
            view_surface.mark_dirty();
        }
        assert_eq!(buffer.pixels()[0], 0x7F);
    }

    #[test]
    fn view_surface_rejects_empty_view() {
        let mut buffer = PixelBuffer::<BgraPremultiplied>::new(Size::new(0, 4));
        assert!(ViewSurface::new(buffer.view_mut()).is_err());
    }

    #[test]
    fn view_surface_accepts_strided_view() {
        let mut data = vec![0u8; 2 * 12];
        let metadata = BufferMetadata::new(Pixels::new(2u32), Pixels::new(2u32), Stride::new(12));
        let view = BufferViewMut::<BgraPremultiplied>::from_raw(&mut data, metadata).unwrap();
        assert!(ViewSurface::new(view).is_ok());
    }

    #[test]
    fn pixel_buffer_from_surface_reads_rows() {
        let mut surface = ImageSurface::create(Format::ARgb32, 3, 2).unwrap();
        {
            let context = cairo::Context::new(&surface).unwrap();
            context.set_source_rgba(0.0, 1.0, 0.0, 1.0);
            context.paint().unwrap();
        }
        let buffer = pixel_buffer_from_surface(&mut surface).unwrap();
        assert_eq!(buffer.size(), Size::new(3, 2));
        assert_eq!(&buffer.pixels()[0..4], &[0, 255, 0, 255]);
    }

    #[test]
    fn pixel_buffer_from_surface_reports_borrowed_data() {
        let surface = ImageSurface::create(Format::ARgb32, 2, 2).unwrap();
        let _context = cairo::Context::new(&surface).unwrap();
        let mut clone = surface;
        assert!(matches!(pixel_buffer_from_surface(&mut clone), Err(DrawingError::SurfaceBorrowed)));
    }

    #[test]
    fn pixel_buffer_from_surface_rejects_non_argb32() {
        let mut surface = ImageSurface::create(Format::A8, 2, 2).unwrap();
        assert!(matches!(pixel_buffer_from_surface(&mut surface), Err(DrawingError::InvalidSurface(_))));
    }
}
