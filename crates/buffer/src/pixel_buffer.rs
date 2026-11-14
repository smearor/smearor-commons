use crate::Bgra;
use crate::BufferError;
use crate::BufferMetadata;
use crate::BufferView;
use crate::BufferViewMut;
use crate::PixelFormat;
use crate::Rgba;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use std::marker::PhantomData;

/// An owned, packed pixel buffer tagged with its pixel format.
///
/// Owns `width * height * F::BYTES_PER_PIXEL` bytes in row-major
/// order - always packed, never strided. Foreign or padded memory is
/// represented by [`BufferView`]/[`BufferViewMut`] instead.
///
/// The default format is [`Rgba`]; `PixelBuffer` without a parameter
/// is an RGBA buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelBuffer<F = Rgba> {
    /// Raw pixel data, `size.x * size.y * bpp` bytes, row-major.
    pixels: Vec<u8>,
    /// Buffer extent in pixels.
    size: Size<u32>,
    /// The pixel format marker.
    format: PhantomData<F>,
}

impl<F: PixelFormat> PixelBuffer<F> {
    /// A zeroed buffer of the given extent.
    pub fn new(size: Size<u32>) -> Self {
        Self {
            pixels: vec![0; Self::expected_len(size)],
            size,
            format: PhantomData,
        }
    }

    /// Adopts raw pixel data when its length matches the packed extent.
    pub fn from_raw(size: Size<u32>, pixels: Vec<u8>) -> Result<Self, BufferError> {
        let expected = Self::expected_len(size);
        if pixels.len() != expected {
            return Err(BufferError::LengthMismatch {
                expected,
                actual: pixels.len(),
                width: Pixels::new(size.x),
                height: Pixels::new(size.y),
                bytes_per_pixel: F::BYTES_PER_PIXEL,
            });
        }
        Ok(Self {
            pixels,
            size,
            format: PhantomData,
        })
    }

    /// The packed byte length for `size` in format `F`.
    pub fn expected_len(size: Size<u32>) -> usize {
        size.x as usize * size.y as usize * F::BYTES_PER_PIXEL
    }

    /// The buffer extent in pixels.
    pub const fn size(&self) -> Size<u32> {
        self.size
    }

    /// The buffer width.
    pub fn width(&self) -> Pixels<u32> {
        Pixels::new(self.size.x)
    }

    /// The buffer height.
    pub fn height(&self) -> Pixels<u32> {
        Pixels::new(self.size.y)
    }

    /// The pixel count (`width * height`).
    pub fn area(&self) -> Pixels<u64> {
        Pixels::new(self.size.x as u64 * self.size.y as u64)
    }

    /// The packed pixel bytes.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// The packed pixel bytes, mutably.
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    /// Whether every byte is zero - an untouched buffer.
    pub fn is_zero(&self) -> bool {
        self.pixels.iter().all(|&byte| byte == 0)
    }

    /// A strided view of this buffer - packed, so `stride` is the row width.
    pub fn view(&self) -> BufferView<'_, F> {
        BufferView {
            data: &self.pixels,
            metadata: BufferMetadata::packed(Pixels::new(self.size.x), Pixels::new(self.size.y), F::BYTES_PER_PIXEL),
            format: PhantomData,
        }
    }

    /// A mutable strided view of this buffer.
    pub fn view_mut(&mut self) -> BufferViewMut<'_, F> {
        BufferViewMut {
            data: &mut self.pixels,
            metadata: BufferMetadata::packed(Pixels::new(self.size.x), Pixels::new(self.size.y), F::BYTES_PER_PIXEL),
            format: PhantomData,
        }
    }
}

impl PixelBuffer<Rgba> {
    /// The same pixels in [`Bgra`] order - swaps the `r`/`b` channel
    /// pairs in place and retags the buffer.
    pub fn into_bgra(mut self) -> PixelBuffer<Bgra> {
        swap_first_third_channel(&mut self.pixels);
        PixelBuffer {
            pixels: self.pixels,
            size: self.size,
            format: PhantomData,
        }
    }
}

impl PixelBuffer<Bgra> {
    /// The same pixels in [`Rgba`] order - swaps the `b`/`r` channel
    /// pairs in place and retags the buffer.
    pub fn into_rgba(mut self) -> PixelBuffer<Rgba> {
        swap_first_third_channel(&mut self.pixels);
        PixelBuffer {
            pixels: self.pixels,
            size: self.size,
            format: PhantomData,
        }
    }
}

impl<F: PixelFormat> AsRef<[u8]> for PixelBuffer<F> {
    fn as_ref(&self) -> &[u8] {
        &self.pixels
    }
}

impl<F: PixelFormat> AsMut<[u8]> for PixelBuffer<F> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }
}

/// Swaps byte `0` and `2` of every 4-byte pixel (`r <-> b`, `a` kept).
fn swap_first_third_channel(pixels: &mut [u8]) {
    for pixel in pixels.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::PixelBuffer;
    use crate::Bgra;
    use crate::Rgba;
    use smearor_commons_geometry::Size;

    /// `area` counts pixels, not bytes.
    #[test]
    fn area_counts_pixels() {
        let buffer = PixelBuffer::<Rgba>::new(Size::new(4, 3));
        assert_eq!(buffer.area().as_raw(), 12);
    }

    /// `AsRef`/`AsMut` expose the packed bytes.
    #[test]
    fn as_ref_as_mut_expose_bytes() {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(1, 1));
        let bytes: &mut [u8] = buffer.as_mut();
        bytes[0] = 42;
        let bytes: &[u8] = buffer.as_ref();
        assert_eq!(bytes, &[42, 0, 0, 0]);
    }

    /// `new` allocates the packed length zeroed.
    #[test]
    fn new_allocates_packed_and_zeroed() {
        let buffer = PixelBuffer::<Rgba>::new(Size::new(4, 3));
        assert_eq!(buffer.pixels().len(), 48);
        assert!(buffer.is_zero());
        assert_eq!(buffer.size(), Size::new(4, 3));
    }

    /// `from_raw` rejects length mismatches.
    #[test]
    fn from_raw_validates_length() {
        assert!(PixelBuffer::<Rgba>::from_raw(Size::new(2, 2), vec![0; 16]).is_ok());
        assert!(PixelBuffer::<Rgba>::from_raw(Size::new(2, 2), vec![0; 15]).is_err());
    }

    /// `into_bgra`/`into_rgba` swap the outer channels and roundtrip.
    #[test]
    fn channel_swap_roundtrips() {
        let pixels = vec![1, 2, 3, 4, 10, 20, 30, 40];
        let rgba = PixelBuffer::<Rgba>::from_raw(Size::new(2, 1), pixels).unwrap();
        let bgra = rgba.into_bgra();
        assert_eq!(bgra.pixels(), &[3, 2, 1, 4, 30, 20, 10, 40]);
        assert_eq!(bgra.into_rgba().pixels(), &[1, 2, 3, 4, 10, 20, 30, 40]);
    }

    /// `Bgra` is a distinct format at the type level.
    #[test]
    fn bgra_buffer_works() {
        let buffer = PixelBuffer::<Bgra>::new(Size::new(2, 2));
        assert_eq!(buffer.pixels().len(), 16);
    }

    /// Every format marker allocates its own packed size.
    #[test]
    fn formats_allocate_their_packed_size() {
        assert_eq!(PixelBuffer::<crate::Rgb>::new(Size::new(2, 2)).pixels().len(), 12);
        assert_eq!(PixelBuffer::<crate::Bgr>::new(Size::new(2, 2)).pixels().len(), 12);
        assert_eq!(PixelBuffer::<crate::Rgbx>::new(Size::new(2, 2)).pixels().len(), 16);
        assert_eq!(PixelBuffer::<crate::Bgrx>::new(Size::new(2, 2)).pixels().len(), 16);
        assert_eq!(PixelBuffer::<crate::Gray>::new(Size::new(2, 2)).pixels().len(), 4);
        use crate::PixelFormat;
        const {
            assert!(!crate::Rgb::HAS_ALPHA && crate::Rgba::HAS_ALPHA);
        }
    }
}
