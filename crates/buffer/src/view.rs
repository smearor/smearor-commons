use crate::BufferError;
use crate::BufferMetadata;
use crate::PixelFormat;
use crate::Rgba;
use crate::Stride;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use std::marker::PhantomData;

/// A read-only view of borrowed pixel memory with row stride.
///
/// Generalizes the `(pixels, width, height)` triple for foreign memory
/// where rows may carry padding: `stride` counts bytes per row and is
/// at least the packed row width. Wayland SHM buffers and framework
/// surfaces are the motivating cases.
#[derive(Debug, Clone, Copy)]
pub struct BufferView<'a, F = Rgba> {
    /// Borrowed pixel memory, `metadata.required_len(bpp)` bytes at least.
    pub(crate) data: &'a [u8],
    /// Extent and stride of the viewed memory.
    pub(crate) metadata: BufferMetadata,
    /// The pixel format marker.
    pub(crate) format: PhantomData<F>,
}

impl<'a, F: PixelFormat> BufferView<'a, F> {
    /// Views strided memory; `data` must cover every pixel row
    /// ([`BufferMetadata::required_len`]).
    pub fn from_raw(data: &'a [u8], metadata: BufferMetadata) -> Result<Self, BufferError> {
        metadata.validate(F::BYTES_PER_PIXEL, data.len())?;
        Ok(Self {
            data,
            metadata,
            format: PhantomData,
        })
    }

    /// Views packed memory (`stride == width * bytes_per_pixel`).
    pub fn from_packed(data: &'a [u8], size: Size<u32>) -> Result<Self, BufferError> {
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
    pub const fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Whether the rows carry no padding.
    pub fn is_packed(&self) -> bool {
        self.metadata.is_packed(F::BYTES_PER_PIXEL)
    }

    /// The pixel bytes of row `y` without padding, `None` when out of bounds.
    pub fn row(&self, y: Pixels<u32>) -> Option<&'a [u8]> {
        let start = self.metadata.row_offset(y)?;
        self.data.get(start..start + self.metadata.row_bytes(F::BYTES_PER_PIXEL))
    }

    /// The byte offset of pixel `(x, y)` inside `data`, `None` when out of bounds.
    pub fn pixel_offset(&self, x: Pixels<u32>, y: Pixels<u32>) -> Option<usize> {
        self.metadata.pixel_offset(x, y, F::BYTES_PER_PIXEL)
    }
}

#[cfg(test)]
mod tests {
    use super::BufferView;
    use crate::BufferMetadata;
    use crate::Rgba;
    use crate::Stride;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Size;

    /// `row` skips the padding of previous rows.
    #[test]
    fn row_skips_strided_padding() {
        // 2x2 RGBA with stride 12 (8 packed + 4 padding per row).
        let data: Vec<u8> = (0..24).collect();
        let view = BufferView::<Rgba>::from_raw(&data, BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(12))).unwrap();
        assert_eq!(view.row(Pixels::new(0)), Some(&data[0..8]));
        assert_eq!(view.row(Pixels::new(1)), Some(&data[12..20]));
        assert_eq!(view.row(Pixels::new(2)), None);
    }

    /// `pixel_offset` accounts for stride and pixel size.
    #[test]
    fn pixel_offset_accounts_for_stride() {
        let data = vec![0u8; 24];
        let view = BufferView::<Rgba>::from_raw(&data, BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(12))).unwrap();
        assert_eq!(view.pixel_offset(Pixels::new(0), Pixels::new(0)), Some(0));
        assert_eq!(view.pixel_offset(Pixels::new(1), Pixels::new(1)), Some(16));
        assert_eq!(view.pixel_offset(Pixels::new(2), Pixels::new(0)), None);
    }

    /// Stride below the packed row is rejected.
    #[test]
    fn stride_below_row_width_rejected() {
        let data = vec![0u8; 24];
        assert!(BufferView::<Rgba>::from_raw(&data, BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(7))).is_err());
    }

    /// Data shorter than the last pixel row is rejected.
    #[test]
    fn short_data_rejected() {
        let data = vec![0u8; 19];
        assert!(BufferView::<Rgba>::from_raw(&data, BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(12))).is_err());
    }

    /// `from_packed` derives the packed stride.
    #[test]
    fn from_packed_sets_packed_stride() {
        let data = vec![0u8; 16];
        let view = BufferView::<Rgba>::from_packed(&data, Size::new(2, 2)).unwrap();
        assert!(view.is_packed());
        assert_eq!(view.stride(), Stride::new(8));
    }
}
