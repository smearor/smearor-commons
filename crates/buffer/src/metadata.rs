use crate::BufferError;
use crate::Stride;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use std::fmt::Display;
use std::fmt::Formatter;

/// Buffer dimensions and row stride - format-agnostic metadata.
///
/// `width` and `height` count pixels, `stride` counts *bytes per row*
/// including row padding: for strided foreign memory (Wayland SHM,
/// framework surfaces) `stride >= width * bytes_per_pixel`, for packed
/// buffers equality holds. The pixel format is deliberately not part
/// of the metadata - it lives at the type level on [`crate::PixelBuffer`]
/// and the views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferMetadata {
    /// Buffer width.
    pub width: Pixels<u32>,
    /// Buffer height.
    pub height: Pixels<u32>,
    /// Bytes per row including padding.
    pub stride: Stride,
}

impl BufferMetadata {
    /// Creates buffer metadata.
    pub const fn new(width: Pixels<u32>, height: Pixels<u32>, stride: Stride) -> Self {
        Self { width, height, stride }
    }

    /// Metadata for a packed buffer (`stride == width * bytes_per_pixel`).
    pub fn packed(width: Pixels<u32>, height: Pixels<u32>, bytes_per_pixel: usize) -> Self {
        Self::new(width, height, Stride::new((width.as_raw() as usize * bytes_per_pixel) as u32))
    }

    /// The buffer extent.
    pub fn size(&self) -> Size<u32> {
        Size::new(self.width.as_raw(), self.height.as_raw())
    }

    /// The packed row width in bytes (`width * bytes_per_pixel`).
    pub fn row_bytes(&self, bytes_per_pixel: usize) -> usize {
        self.width.as_raw() as usize * bytes_per_pixel
    }

    /// The minimal data length covering every pixel row - the last
    /// row carries no trailing padding: `(height - 1) * stride +
    /// row_bytes` (`0` for empty buffers).
    pub fn required_len(&self, bytes_per_pixel: usize) -> usize {
        if self.height.as_raw() == 0 {
            return 0;
        }
        (self.height.as_raw() as usize - 1) * self.stride.as_usize() + self.row_bytes(bytes_per_pixel)
    }

    /// The length of a fully strided allocation (`stride * height`).
    pub fn buffer_len(&self) -> usize {
        self.stride.as_usize() * self.height.as_raw() as usize
    }

    /// The byte offset where row `y`'s pixel data begins, `None`
    /// when `y` is out of bounds.
    pub fn row_offset(&self, y: Pixels<u32>) -> Option<usize> {
        if y >= self.height {
            return None;
        }
        Some(y.as_usize() * self.stride.as_usize())
    }

    /// The byte offset of pixel `(x, y)` inside the buffer for a
    /// format with `bytes_per_pixel`, `None` when out of bounds.
    pub fn pixel_offset(&self, x: Pixels<u32>, y: Pixels<u32>, bytes_per_pixel: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y.as_usize() * self.stride.as_usize() + x.as_usize() * bytes_per_pixel)
    }

    /// Whether the stride carries no padding for `bytes_per_pixel`.
    pub fn is_packed(&self, bytes_per_pixel: usize) -> bool {
        self.stride.as_usize() == self.row_bytes(bytes_per_pixel)
    }

    /// Validates that a byte buffer of `data_len` can hold this
    /// metadata in a format with `bytes_per_pixel`: the stride must
    /// cover a packed pixel row and the data every pixel row.
    pub fn validate(&self, bytes_per_pixel: usize, data_len: usize) -> Result<(), BufferError> {
        let row_bytes = self.row_bytes(bytes_per_pixel);
        if self.stride.as_usize() < row_bytes {
            return Err(BufferError::StrideTooSmall {
                stride: self.stride,
                row_bytes,
            });
        }
        let required = self.required_len(bytes_per_pixel);
        if data_len < required {
            return Err(BufferError::DataTooShort { required, actual: data_len });
        }
        Ok(())
    }
}

impl Display for BufferMetadata {
    /// `"{width}x{height} stride {stride}"`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} stride {}", self.width, self.height, self.stride)
    }
}

#[cfg(test)]
mod tests {
    use super::BufferMetadata;
    use crate::Stride;
    use smearor_commons_geometry::Pixels;

    /// `required_len` drops the trailing padding of the last row.
    #[test]
    fn required_len_skips_last_row_padding() {
        let metadata = BufferMetadata::new(Pixels::new(4), Pixels::new(3), Stride::new(20));
        assert_eq!(metadata.required_len(4), 2 * 20 + 16);
        assert_eq!(metadata.buffer_len(), 60);
        assert!(!metadata.is_packed(4));
    }

    /// Packed metadata has no padding.
    #[test]
    fn packed_metadata_has_no_padding() {
        let metadata = BufferMetadata::packed(Pixels::new(4), Pixels::new(3), 4);
        assert_eq!(metadata.stride, Stride::new(16));
        assert!(metadata.is_packed(4));
        assert_eq!(metadata.required_len(4), metadata.buffer_len());
    }

    /// Empty buffers require no data.
    #[test]
    fn empty_buffer_requires_nothing() {
        assert_eq!(BufferMetadata::new(Pixels::new(4), Pixels::new(0), Stride::new(20)).required_len(4), 0);
    }

    #[test]
    fn display_shows_extent_and_stride() {
        assert_eq!(
            BufferMetadata::new(Pixels::new(1920), Pixels::new(1080), Stride::new(7680)).to_string(),
            "1920x1080 stride 7680"
        );
    }
}
