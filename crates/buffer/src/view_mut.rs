use crate::BufferError;
use crate::BufferMetadata;
use crate::PixelFormat;
use crate::Rgba;
use crate::Stride;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use std::marker::PhantomData;

/// A mutable view of borrowed pixel memory with row stride - the
/// rendering target for draw operations on foreign buffers.
///
/// See [`crate::BufferView`] for the stride semantics; this is the
/// `&mut` counterpart draw code writes into.
pub struct BufferViewMut<'a, F = Rgba> {
    /// Borrowed pixel memory, `metadata.required_len(bpp)` bytes at least.
    pub(crate) data: &'a mut [u8],
    /// Extent and stride of the viewed memory.
    pub(crate) metadata: BufferMetadata,
    /// The pixel format marker.
    pub(crate) format: PhantomData<F>,
}

impl<'a, F: PixelFormat> BufferViewMut<'a, F> {
    /// Views strided memory; `data` must cover every pixel row
    /// ([`BufferMetadata::required_len`]).
    pub fn from_raw(data: &'a mut [u8], metadata: BufferMetadata) -> Result<Self, BufferError> {
        metadata.validate(F::BYTES_PER_PIXEL, data.len())?;
        Ok(Self {
            data,
            metadata,
            format: PhantomData,
        })
    }

    /// Views packed memory (`stride == width * bytes_per_pixel`).
    pub fn from_packed(data: &'a mut [u8], size: Size<u32>) -> Result<Self, BufferError> {
        Self::from_raw(data, BufferMetadata::packed(Pixels::new(size.x), Pixels::new(size.y), F::BYTES_PER_PIXEL))
    }

    /// Extent and stride of the viewed memory.
    pub const fn metadata(&self) -> BufferMetadata {
        self.metadata
    }

    /// The view extent.
    pub fn size(&self) -> Size<u32> {
        self.metadata.size()
    }

    /// The view width.
    pub fn width(&self) -> Pixels<u32> {
        self.metadata.width
    }

    /// The view height.
    pub fn height(&self) -> Pixels<u32> {
        self.metadata.height
    }

    /// Bytes per row including padding.
    pub const fn stride(&self) -> Stride {
        self.metadata.stride
    }

    /// The viewed memory including row padding.
    pub const fn data(&self) -> &[u8] {
        self.data
    }

    /// The viewed memory including row padding, mutably.
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data
    }

    /// Whether the rows carry no padding.
    pub fn is_packed(&self) -> bool {
        self.metadata.is_packed(F::BYTES_PER_PIXEL)
    }

    /// The pixel bytes of row `y` without padding, `None` when out of bounds.
    pub fn row(&self, y: Pixels<u32>) -> Option<&[u8]> {
        let start = self.metadata.row_offset(y)?;
        self.data.get(start..start + self.metadata.row_bytes(F::BYTES_PER_PIXEL))
    }

    /// The pixel bytes of row `y` without padding, mutably.
    pub fn row_mut(&mut self, y: Pixels<u32>) -> Option<&mut [u8]> {
        if y >= self.metadata.height {
            return None;
        }
        let start = self.metadata.row_offset(y)?;
        self.data.get_mut(start..start + self.metadata.row_bytes(F::BYTES_PER_PIXEL))
    }

    /// The byte offset of pixel `(x, y)` inside `data`, `None` when out of bounds.
    pub fn pixel_offset(&self, x: Pixels<u32>, y: Pixels<u32>) -> Option<usize> {
        self.metadata.pixel_offset(x, y, F::BYTES_PER_PIXEL)
    }

    /// Fills every pixel with `pixel` (`F::BYTES_PER_PIXEL` bytes).
    ///
    /// Packed views write the pattern over the whole slice in one pass;
    /// strided views repeat it per row, leaving padding untouched.
    /// Does nothing when `pixel` is not exactly one pixel wide.
    pub fn fill(&mut self, pixel: &[u8]) {
        if pixel.len() != F::BYTES_PER_PIXEL {
            return;
        }
        if self.is_packed() {
            let len = self.metadata.required_len(F::BYTES_PER_PIXEL);
            fill_pixels(&mut self.data[..len], F::BYTES_PER_PIXEL, pixel);
        } else {
            for y in 0..self.metadata.height.as_raw() {
                if let Some(row) = self.row_mut(Pixels::new(y)) {
                    fill_pixels(row, F::BYTES_PER_PIXEL, pixel);
                }
            }
        }
    }
}

/// Repeats `pixel` across `data`, which must be a whole number of
/// `bpp`-sized pixels. The `4`-byte specialization keeps the literal
/// width visible to codegen (word-sized stores).
fn fill_pixels(data: &mut [u8], bpp: usize, pixel: &[u8]) {
    if bpp == 4 {
        if let Ok(pixel) = <&[u8; 4]>::try_from(pixel) {
            data.as_chunks_mut::<4>().0.fill(*pixel);
        }
    } else {
        for chunk in data.chunks_exact_mut(bpp) {
            chunk.copy_from_slice(pixel);
        }
    }
}

impl<F> std::fmt::Debug for BufferViewMut<'_, F> {
    /// Shows metadata; the data itself is opaque.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferViewMut").field("metadata", &self.metadata).finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::BufferViewMut;
    use crate::BufferMetadata;
    use crate::Rgba;
    use crate::Stride;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Size;

    /// `row_mut` writes through the stride, leaving padding untouched.
    #[test]
    fn row_mut_writes_through_stride() {
        let mut data = vec![0u8; 24];
        {
            let mut view = BufferViewMut::<Rgba>::from_raw(&mut data, BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(12))).unwrap();
            if let Some(row) = view.row_mut(Pixels::new(1)) {
                row.fill(7);
            }
        }
        assert_eq!(&data[12..20], &[7; 8]);
        assert_eq!(&data[20..24], &[0; 4]);
    }

    /// `fill` covers packed and strided views alike.
    #[test]
    fn fill_writes_every_pixel_and_leaves_padding() {
        let mut packed = vec![0u8; 16];
        {
            let mut view = BufferViewMut::<Rgba>::from_packed(&mut packed, Size::new(2, 2)).unwrap();
            view.fill(&[9, 8, 7, 6]);
        }
        assert_eq!(packed, [9, 8, 7, 6].repeat(4));

        let mut strided = vec![0u8; 24];
        {
            let meta = BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(12));
            let mut view = BufferViewMut::<Rgba>::from_raw(&mut strided, meta).unwrap();
            view.fill(&[1, 2, 3, 4]);
        }
        assert_eq!(&strided[..8], &[1, 2, 3, 4, 1, 2, 3, 4]);
        assert_eq!(&strided[8..12], &[0; 4]);
        assert_eq!(&strided[12..20], &[1, 2, 3, 4, 1, 2, 3, 4]);
        assert_eq!(&strided[20..24], &[0; 4]);
    }

    /// A wrong-length pattern leaves the buffer untouched.
    #[test]
    fn fill_ignores_wrong_pixel_length() {
        let mut data = vec![0u8; 16];
        {
            let mut view = BufferViewMut::<Rgba>::from_packed(&mut data, Size::new(2, 2)).unwrap();
            view.fill(&[1, 2, 3]);
        }
        assert_eq!(data, vec![0u8; 16]);
    }

    /// `from_packed` is the common rendering-target construction.
    #[test]
    fn from_packed_views_the_whole_slice() {
        let mut data = vec![0u8; 16];
        let mut view = BufferViewMut::<Rgba>::from_packed(&mut data, Size::new(2, 2)).unwrap();
        view.data_mut().fill(1);
        assert_eq!(view.row(Pixels::new(0)), Some(&[1u8; 8][..]));
    }
}
