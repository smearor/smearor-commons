//! GTK bridge (`gtk4` feature): `PixelBuffer` ⇄ `gdk::Texture`.
//!
//! - [`PixelBuffer::to_memory_texture`] uploads buffer pixels as a
//!   [`MemoryTexture`](gtk4::gdk::MemoryTexture) — display them via
//!   `gtk4::Image::from_paintable` or any paintable consumer.
//! - [`PixelBuffer::from_texture`] downloads a texture's pixels back
//!   into a `PixelBuffer<Rgba>`, converting through the `PixelFormat`
//!   channel interface (premultiplied → straight alpha included).
//! - [`PixelBuffer::decode_image`] decodes embedded image bytes
//!   (PNG, JPEG, …) through `gdk::Texture::from_bytes` — the
//!   generalized chess `decode_texture` helper.
//! - `From<PixelFormatId> for MemoryFormat` and the fallible inverse
//!   carry the byte-exact format mapping next to the type it
//!   describes.

mod decode;
mod memory_format;
mod texture;
mod texture_error;

pub use texture_error::TextureError;

#[cfg(test)]
mod tests {
    use crate::BgraPremultiplied;
    use crate::PixelBuffer;
    use crate::PixelFormat;
    use crate::PixelFormatId;
    use crate::Rgba;
    use crate::gtk::TextureError;
    use gtk4::gdk::MemoryFormat;
    use gtk4::gdk::Texture;
    use gtk4::prelude::Cast;
    use smearor_commons_geometry::Size;

    /// A 2x1 RGBA PNG (red + blue pixel), generated offline — keeps
    /// the decode test free of encoder dependencies.
    const TEST_PNG: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0, 1, 8, 6, 0, 0, 0, 244, 34, 127, 138, 0, 0, 0, 14, 73, 68, 65, 84,
        120, 156, 99, 248, 207, 192, 0, 66, 255, 1, 15, 249, 3, 253, 133, 17, 153, 118, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];

    /// Builds a texture over `rgba` bytes through a
    /// `PixelBuffer<Rgba>` — the upload path under test.
    fn rgba_texture(width: u32, height: u32, rgba: &[u8]) -> Texture {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(width, height));
        buffer.pixels_mut().copy_from_slice(rgba);
        buffer.to_memory_texture().upcast()
    }

    #[test]
    fn memory_format_mapping_is_total() {
        for (format, expected) in [
            (PixelFormatId::Rgba, MemoryFormat::R8g8b8a8),
            (PixelFormatId::Bgra, MemoryFormat::B8g8r8a8),
            (PixelFormatId::RgbaPremultiplied, MemoryFormat::R8g8b8a8Premultiplied),
            (PixelFormatId::BgraPremultiplied, MemoryFormat::B8g8r8a8Premultiplied),
            (PixelFormatId::Rgb, MemoryFormat::R8g8b8),
            (PixelFormatId::Bgr, MemoryFormat::B8g8r8),
            (PixelFormatId::Rgbx, MemoryFormat::R8g8b8x8),
            (PixelFormatId::Bgrx, MemoryFormat::B8g8r8x8),
            (PixelFormatId::Gray, MemoryFormat::G8),
        ] {
            assert_eq!(MemoryFormat::from(format), expected);
            assert_eq!(PixelFormatId::try_from(expected), Ok(format));
        }
    }

    #[test]
    fn memory_format_rejects_wide_formats() {
        assert_eq!(PixelFormatId::try_from(MemoryFormat::G8a8), Err(MemoryFormat::G8a8));
        assert_eq!(PixelFormatId::try_from(MemoryFormat::R16g16b16a16), Err(MemoryFormat::R16g16b16a16));
    }

    #[test]
    fn buffer_texture_roundtrip_keeps_pixels() {
        let expected: [u8; 16] = [10, 20, 30, 255, 40, 50, 60, 128, 70, 80, 90, 0, 100, 110, 120, 255];
        let texture = rgba_texture(2, 2, &expected);
        let buffer = PixelBuffer::<Rgba>::from_texture(&texture);
        // Download goes through premultiplied BGRA — a=0 pixels lose
        // their color channels; ±1 tolerance for the alpha round-trip.
        for (s, d) in expected.as_chunks::<4>().0.iter().zip(buffer.pixels().as_chunks::<4>().0.iter()) {
            if s[3] == 0 {
                assert_eq!(d, &[0, 0, 0, 0]);
            } else {
                for (x, y) in s.iter().zip(d) {
                    assert!((i32::from(*x) - i32::from(*y)).abs() <= 1, "{s:?} != {d:?}");
                }
            }
        }
    }

    #[test]
    fn premultiplied_source_unpremultiplies() {
        let mut buffer = PixelBuffer::<BgraPremultiplied>::new(Size::new(1, 1));
        BgraPremultiplied::write_pixel(&mut buffer.pixels_mut()[..4], 200, 100, 50, 128);
        let texture: Texture = buffer.to_memory_texture().upcast();
        let back = PixelBuffer::<Rgba>::from_texture(&texture);
        let [r, g, b, a] = Rgba::read_pixel(back.pixels());
        assert_eq!(a, 128);
        assert!((r as i32 - 200).abs() <= 1 && (g as i32 - 100).abs() <= 1 && (b as i32 - 50).abs() <= 1);
    }

    #[test]
    fn decode_image_decodes_a_png() {
        let buffer = PixelBuffer::<Rgba>::decode_image(TEST_PNG).unwrap();
        assert_eq!(buffer.size(), Size::new(2, 1));
        let [r0, g0, b0, _] = Rgba::read_pixel(&buffer.pixels()[..4]);
        let [r1, g1, b1, _] = Rgba::read_pixel(&buffer.pixels()[4..8]);
        assert!(r0 > 200 && g0 < 60 && b0 < 60, "left pixel red: {r0},{g0},{b0}");
        assert!(r1 < 60 && g1 < 60 && b1 > 200, "right pixel blue: {r1},{g1},{b1}");
    }

    #[test]
    fn decode_image_rejects_garbage() {
        assert!(matches!(PixelBuffer::<Rgba>::decode_image(b"not an image"), Err(TextureError::Decode(_))));
    }
}
