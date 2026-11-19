//! The [`DrawPrimitiveOps`] implementation for `BufferViewMut<Rgba>`:
//! the actual software-raster logic shared behind a shape-scan helper.

use crate::buffer::raster::blend_pixel;
use crate::ops::DrawPrimitiveOps;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// Blend `color` at `alpha` onto every pixel whose center lies in
/// `bounds ∩ shape`, sampled at pixel centers like the other raster ops.
fn blend_scanned<F>(view: &mut BufferViewMut<'_, Rgba>, bounds: Rect<Pixels, i32>, color: Color<f64>, alpha: impl Into<Alpha>, shape: F)
where
    F: Fn(&Vec2<Pixels, f64>) -> bool,
{
    let size = view.size();
    let stride = view.stride().as_usize();
    let pixels = view.data_mut();
    let color: Color<u8> = color.into();
    let alpha = alpha.into();
    let (x0, y0) = (bounds.min.x.max(0), bounds.min.y.max(0));
    let (x1, y1) = (bounds.max().x.min(size.x as i32), bounds.max().y.min(size.y as i32));

    for y in y0..y1 {
        for x in x0..x1 {
            if shape(&Vec2::<Pixels>::new(x as f64 + 0.5, y as f64 + 0.5)) {
                let idx = y as usize * stride + x as usize * Rgba::BYTES_PER_PIXEL;
                blend_pixel(pixels, idx, color, alpha);
            }
        }
    }
}

impl DrawPrimitiveOps for BufferViewMut<'_, Rgba> {
    /// Fill the entire view with a solid background color.
    ///
    /// Packed views reinterpret the `&mut [u8]` data as `&mut [u32]` (RGBA
    /// = 4 bytes = one `u32` in little-endian) and use `slice::fill()` for
    /// SIMD-vectorized writes. Strided views fill each row's pixel bytes
    /// and leave row padding untouched.
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        let bytes: [u8; 4] = color.into().into();
        self.fill(&bytes);
        Ok(())
    }

    fn fill_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        let bounds = rect.to_i32_ceil();
        blend_scanned(self, bounds, color, alpha, |p| rect.contains(p));
        Ok(())
    }

    /// Stroke centered on the rect edge like cairo: the stroke region is
    /// `rect ± line_width/2` minus the interior — implemented as the
    /// expanded outline minus the shrunken one.
    fn stroke_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        if line_width.as_raw() <= 0.0 {
            return Ok(());
        }
        let half = Pixels::new(f64::from(line_width.as_raw()) / 2.0);
        let outer = rect.inset(-half);
        let inner = rect.inset(half);
        let bounds = outer.to_i32_ceil();
        blend_scanned(self, bounds, color, alpha, |p| outer.contains(p) && !inner.contains(p));
        Ok(())
    }

    fn fill_circle(&mut self, circle: Circle<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        let bounds = circle.bounding_box().to_i32_ceil();
        blend_scanned(self, bounds, color, alpha, |p| circle.contains(p));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smearor_commons_buffer::BufferMetadata;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_buffer::Stride;
    use smearor_commons_geometry::Size;

    #[test]
    fn fill_background_fills_all_pixels() {
        let mut buf = PixelBuffer::new(Size::new(4, 3));
        let color = Color::<f64>::new(10.0 / 255.0, 20.0 / 255.0, 30.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();

        for px in buf.pixels().as_chunks::<4>().0.iter() {
            assert_eq!(px, &expected);
        }
    }

    #[test]
    fn fill_background_single_pixel() {
        let mut buf = PixelBuffer::new(Size::new(1, 1));
        let color = Color::<f64>::new_rgba(1.0, 0.0, 128.0 / 255.0, 200.0 / 255.0);
        let expected: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();
        assert_eq!(buf.pixels(), &expected[..]);
    }

    #[test]
    fn fill_background_strided_view_leaves_padding_untouched() {
        // 2x2 pixels, stride 16 bytes (8 padding bytes per row):
        // the fill must write only the 8 pixel bytes of each row.
        let mut data = vec![0u8; 32];
        let metadata = BufferMetadata::new(Pixels::new(2u32), Pixels::new(2u32), Stride::new(16));
        let color = Color::<u8>::new(255, 128, 64);
        BufferViewMut::<Rgba>::from_raw(&mut data, metadata).unwrap().fill_background(color).unwrap();

        let bytes: [u8; 4] = color.into();
        assert_eq!(&data[0..8], &[bytes, bytes].concat()[..]);
        assert_eq!(&data[8..16], &[0u8; 8]);
        assert_eq!(&data[16..24], &[bytes, bytes].concat()[..]);
        assert_eq!(&data[24..32], &[0u8; 8]);
    }

    #[test]
    fn fill_background_preserves_color_via_u32_round_trip() {
        let mut buf = PixelBuffer::new(Size::new(2, 2));
        let color = Color::<f64>::new_rgba(0xAB as f64 / 255.0, 0xCD as f64 / 255.0, 0xEF as f64 / 255.0, 0x12 as f64 / 255.0);
        let expected_bytes: [u8; 4] = Color::<u8>::from(color).into();
        buf.view_mut().fill_background(color).unwrap();

        let expected = u32::from_le_bytes(expected_bytes);
        for &px in buf.pixels().as_chunks::<4>().0 {
            assert_eq!(u32::from_le_bytes(px), expected);
        }
    }

    fn px(buf: &PixelBuffer<Rgba>, x: u32, y: u32) -> [u8; 4] {
        let idx = (y * buf.size().x + x) as usize * 4;
        buf.pixels()[idx..idx + 4].try_into().unwrap()
    }

    #[test]
    fn fill_rect_fills_only_inside() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.fill_rect(Rect::<Pixels>::from_xywh(2.0, 2.0, 4.0, 4.0), Color::<f64>::new(1.0, 0.0, 0.0), 1.0);

        assert_eq!(px(&buf, 3, 3), [255, 0, 0, 255]);
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0]);
        assert_eq!(px(&buf, 7, 7), [0, 0, 0, 0]);
    }

    #[test]
    fn fill_rect_blends_alpha_over_background() {
        let mut buf = PixelBuffer::new(Size::new(4, 4));
        buf.fill_background(Color::<u8>::new(0, 0, 255)).unwrap();
        buf.fill_rect(Rect::<Pixels>::from_xywh(0.0, 0.0, 4.0, 4.0), Color::<f64>::new(1.0, 0.0, 0.0), 0.5);

        let [r, _g, b, a] = px(&buf, 1, 1);
        assert!((120..=135).contains(&r), "half alpha red over blue ~127, got {r}");
        assert!((120..=135).contains(&b));
        assert_eq!(a, 255);
    }

    #[test]
    fn stroke_rect_strokes_border_only() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.stroke_rect(Rect::<Pixels>::from_xywh(2.0, 2.0, 6.0, 6.0), Color::<f64>::new(0.0, 1.0, 0.0), Pixels::new(2.0f32), 1.0);

        assert_eq!(px(&buf, 2, 2), [0, 255, 0, 255], "corner border");
        assert_eq!(px(&buf, 4, 4), [0, 0, 0, 0], "interior untouched");
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0], "outside untouched");
    }

    #[test]
    fn fill_circle_fills_inside_not_corners() {
        let mut buf = PixelBuffer::new(Size::new(10, 10));
        buf.fill_circle(Circle::<Pixels>::new(Vec2::<Pixels>::new(5.0, 5.0), 3.0), Color::<f64>::new(0.0, 0.0, 1.0), 1.0);

        assert_eq!(px(&buf, 5, 5), [0, 0, 255, 255]);
        assert_eq!(px(&buf, 0, 0), [0, 0, 0, 0]);
        assert_eq!(px(&buf, 9, 9), [0, 0, 0, 0]);
    }

    #[test]
    fn fill_rect_respects_strided_padding() {
        // 2x2 view, stride 16 bytes (8 padding per row)
        let mut data = vec![0u8; 32];
        {
            let metadata = BufferMetadata::new(Pixels::new(2u32), Pixels::new(2u32), Stride::new(16));
            let mut view = BufferViewMut::<Rgba>::from_raw(&mut data, metadata).unwrap();
            view.fill_rect(Rect::<Pixels>::from_xywh(0.0, 0.0, 2.0, 2.0), Color::<f64>::new(1.0, 0.0, 0.0), 1.0);
        }
        assert_eq!(&data[8..16], &[0u8; 8], "padding row untouched");
        assert_eq!(&data[24..32], &[0u8; 8], "padding row untouched");
    }
}
