//! GTK bridge (`gtk4` feature): `PixelBuffer` ⇄ `gdk::Texture`.
//!
//! - [`texture_from_buffer`] uploads buffer pixels as a
//!   [`MemoryTexture`](gtk4::gdk::MemoryTexture) — display them via
//!   `gtk4::Image::from_paintable` or any paintable consumer.
//! - [`buffer_from_texture`] downloads a texture's pixels back into a
//!   `PixelBuffer<Rgba>`, converting through the `PixelFormat`
//!   channel interface (premultiplied → straight alpha included).
//! - [`decode_image`] decodes embedded image bytes (PNG, JPEG, …)
//!   through `gdk::Texture::from_bytes` — the generalized chess
//!   `decode_texture` helper.

mod decode;
mod texture;
mod texture_error;

pub use decode::decode_image;
pub use texture::buffer_from_texture;
pub use texture::texture_from_buffer;
pub use texture_error::TextureError;

#[cfg(test)]
mod tests {
    use super::*;
    use gtk4::gdk::MemoryFormat;
    use gtk4::gdk::Texture;
    use gtk4::prelude::Cast;
    use smearor_commons_buffer::BgraPremultiplied;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_buffer::PixelFormat;
    use smearor_commons_buffer::PixelFormatId;
    use smearor_commons_buffer::Rgba;
    use smearor_commons_geometry::Size;

    /// Builds a texture over `rgba` bytes through a
    /// `PixelBuffer<Rgba>` — the upload path under test.
    fn rgba_texture(width: u32, height: u32, rgba: &[u8]) -> Texture {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(width, height));
        buffer.pixels_mut().copy_from_slice(rgba);
        texture_from_buffer(&buffer).upcast()
    }

    #[test]
    fn texture_upload_preserves_all_buffer_formats() {
        // Every PixelFormatId has a byte-exact MemoryFormat counterpart —
        // upload never fails and never converts.
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
            assert_eq!(super::texture::memory_format(format), expected);
        }
        assert_eq!(super::texture::memory_format(PixelFormatId::Gray), MemoryFormat::G8);
    }

    #[test]
    fn buffer_texture_roundtrip_keeps_pixels() {
        let expected: [u8; 16] = [10, 20, 30, 255, 40, 50, 60, 128, 70, 80, 90, 0, 100, 110, 120, 255];
        let texture = rgba_texture(2, 2, &expected);
        let buffer = buffer_from_texture(&texture);
        // Download goes through premultiplied BGRA — a=0 pixels lose
        // their color channels; ±1 tolerance for the alpha round-trip.
        for (s, d) in expected.as_chunks::<4>().0.iter().zip(buffer.pixels().as_chunks::<4>().0.iter()) {
            if s[3] == 0 {
                assert_eq!(d, &[0, 0, 0, 0]);
            } else {
                for (x, y) in s.iter().zip(d) {
                    assert!((*x as i32 - *y as i32).abs() <= 1, "{s:?} != {d:?}");
                }
            }
        }
    }

    #[test]
    fn premultiplied_source_unpremultiplies() {
        let mut buffer = PixelBuffer::<BgraPremultiplied>::new(Size::new(1, 1));
        // straight rgba(200, 100, 50, 128) premultiplied ≈ b25,g50,r100
        BgraPremultiplied::write_pixel(&mut buffer.pixels_mut()[..4], 200, 100, 50, 128);
        let texture: Texture = texture_from_buffer(&buffer).upcast();
        let back = buffer_from_texture(&texture);
        let [r, g, b, a] = Rgba::read_pixel(back.pixels());
        assert_eq!(a, 128);
        assert!((r as i32 - 200).abs() <= 1 && (g as i32 - 100).abs() <= 1 && (b as i32 - 50).abs() <= 1);
    }

    #[test]
    fn decode_image_roundtrips_a_png() {
        // Encode a 2x1 PNG via cairo — red pixel + blue pixel.
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 2, 1).unwrap();
        {
            let ctx = cairo::Context::new(&surface).unwrap();
            ctx.set_source_rgb(1.0, 0.0, 0.0);
            ctx.rectangle(0.0, 0.0, 1.0, 1.0);
            ctx.fill().unwrap();
            ctx.set_source_rgb(0.0, 0.0, 1.0);
            ctx.rectangle(1.0, 0.0, 1.0, 1.0);
            ctx.fill().unwrap();
        }
        let mut png = Vec::new();
        surface.write_to_png(&mut png).unwrap();

        let buffer = decode_image(&png).unwrap();
        assert_eq!(buffer.size(), Size::new(2, 1));
        let [r0, g0, b0, _] = Rgba::read_pixel(&buffer.pixels()[..4]);
        let [r1, g1, b1, _] = Rgba::read_pixel(&buffer.pixels()[4..8]);
        assert!(r0 > 200 && g0 < 60 && b0 < 60, "left pixel red: {r0},{g0},{b0}");
        assert!(r1 < 60 && g1 < 60 && b1 > 200, "right pixel blue: {r1},{g1},{b1}");
    }

    #[test]
    fn decode_image_rejects_garbage() {
        assert!(matches!(decode_image(b"not an image"), Err(TextureError::Decode(_))));
    }
}
