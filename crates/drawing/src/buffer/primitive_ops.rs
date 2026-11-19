//! The [`DrawPrimitiveOps`] implementations for the buffer targets.

use crate::ops::DrawPrimitiveOps;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;

impl DrawPrimitiveOps for BufferViewMut<'_, Rgba> {
    /// Fill the entire view with a solid background color.
    ///
    /// Packed views reinterpret the `&mut [u8]` data as `&mut [u32]` (RGBA
    /// = 4 bytes = one `u32` in little-endian) and use `slice::fill()` for
    /// SIMD-vectorized writes. Strided views fill each row's pixel bytes
    /// and leave row padding untouched.
    fn fill_background(&mut self, color: impl Into<Color<u8>>) {
        let bytes: [u8; 4] = color.into().into();
        self.fill(&bytes);
    }
}

/// An owned packed buffer fills through its packed view.
impl DrawPrimitiveOps for PixelBuffer<Rgba> {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) {
        self.view_mut().fill_background(color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_buffer::BufferMetadata;
    use smearor_commons_buffer::Stride;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Size;

    #[test]
    fn fill_background_fills_all_pixels() {
        let mut buf = PixelBuffer::new(Size::new(4, 3));
        let color = Color::<f64>::new(10.0 / 255.0, 20.0 / 255.0, 30.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color);

        for px in buf.pixels().as_chunks::<4>().0.iter() {
            assert_eq!(px, &expected);
        }
    }

    #[test]
    fn fill_background_single_pixel() {
        let mut buf = PixelBuffer::new(Size::new(1, 1));
        let color = Color::<f64>::new_rgba(1.0, 0.0, 128.0 / 255.0, 200.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color);
        assert_eq!(buf.pixels(), &expected[..]);
    }

    #[test]
    fn fill_background_strided_view_leaves_padding_untouched() {
        // 2x2 pixels, stride 16 bytes (8 padding bytes per row):
        // required = (2 - 1) * 16 + 2 * 4 = 24, plus a trailing slice tail.
        let mut data = vec![0u8; 32];
        let metadata = BufferMetadata::new(Pixels::new(2), Pixels::new(2), Stride::new(16));
        let color = Color::<u8>::new(200, 100, 50);

        BufferViewMut::<Rgba>::from_raw(&mut data, metadata).unwrap().fill_background(color);

        let bytes: [u8; 4] = color.into();
        // row 0 pixels at [0..8), padding at [8..16)
        assert_eq!(&data[0..4], &bytes);
        assert_eq!(&data[4..8], &bytes);
        assert!(data[8..16].iter().all(|&b| b == 0), "row padding must stay untouched");
        // row 1 pixels at [16..24), memory beyond the view extent untouched
        assert_eq!(&data[16..20], &bytes);
        assert_eq!(&data[20..24], &bytes);
        assert!(data[24..].iter().all(|&b| b == 0), "memory beyond the view extent must stay untouched");
    }

    #[test]
    fn fill_background_preserves_color_via_u32_round_trip() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        let color = Color::<f64>::new_rgba(0xAB as f64 / 255.0, 0xCD as f64 / 255.0, 0xEF as f64 / 255.0, 0x12 as f64 / 255.0);
        let expected_bytes: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color);

        let expected = u32::from_le_bytes(expected_bytes);
        for &px in buf.pixels().as_chunks::<4>().0 {
            assert_eq!(u32::from_le_bytes(px), expected);
        }
    }
}
