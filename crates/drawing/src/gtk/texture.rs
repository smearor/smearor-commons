//! `PixelBuffer` ⇄ `gdk::Texture` conversion.
//!
//! Upload ([`texture_from_buffer`]) is lossless: every
//! [`PixelFormatId`] has a byte-exact [`MemoryFormat`] counterpart,
//! including the premultiplied variants (no unpremultiply round-trip).
//!
//! Download ([`buffer_from_texture`]) always yields `ARgb32` — i.e.
//! premultiplied BGRA on little-endian — because
//! `gdk_texture_download` is defined as "the format cairo uses":
//! four bytes per pixel regardless of the texture's stored format.
//! The pixels are converted to straight-alpha RGBA through the
//! [`PixelFormat`] channel interface.

use gtk4::gdk::MemoryFormat;
use gtk4::gdk::MemoryTexture;
use gtk4::gdk::Texture;
use gtk4::glib;
use gtk4::prelude::TextureExt;
use gtk4::prelude::TextureExtManual;
use smearor_commons_buffer::BgraPremultiplied;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::PixelFormatId;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Size;

/// Uploads `buffer`'s pixels as a [`MemoryTexture`] — the image
/// bridge used by every GTK consumer that displays software-rendered
/// content (left demo column, launcher icon widgets).
///
/// The texture owns a copy of the pixel bytes; the buffer may be
/// dropped or reused afterwards.
pub fn texture_from_buffer<F: PixelFormat>(buffer: &PixelBuffer<F>) -> MemoryTexture {
    let size = buffer.size();
    let bytes = glib::Bytes::from(buffer.pixels());
    MemoryTexture::new(size.x as i32, size.y as i32, memory_format(F::ID), &bytes, size.x as usize * F::BYTES_PER_PIXEL)
}

/// Downloads `texture`'s pixels into a packed
/// [`PixelBuffer<Rgba>`], unpremultiplying through the [`PixelFormat`]
/// channel interface.
///
/// `gdk_texture_download` always yields `ARgb32` pixels — premultiplied
/// BGRA in memory order — so this function is infallible and ignores
/// [`Texture::format`], which reports the *stored* format, not the
/// download format.
pub fn buffer_from_texture(texture: &Texture) -> PixelBuffer<Rgba> {
    let (width, height) = (texture.width() as usize, texture.height() as usize);
    let stride = width * BgraPremultiplied::BYTES_PER_PIXEL;
    let mut pixels = vec![0u8; height * stride];
    texture.download(&mut pixels, stride);

    let mut buffer = PixelBuffer::<Rgba>::new(Size::new(width as u32, height as u32));
    for (s, d) in pixels.as_chunks::<4>().0.iter().zip(buffer.pixels_mut().as_chunks_mut::<4>().0.iter_mut()) {
        let [r, g, b, a] = BgraPremultiplied::read_pixel(s);
        Rgba::write_pixel(d, r, g, b, a);
    }
    buffer
}

/// The [`MemoryFormat`] matching a [`PixelFormatId`] byte-for-byte.
pub(crate) fn memory_format(format: PixelFormatId) -> MemoryFormat {
    match format {
        PixelFormatId::Rgba => MemoryFormat::R8g8b8a8,
        PixelFormatId::Bgra => MemoryFormat::B8g8r8a8,
        PixelFormatId::RgbaPremultiplied => MemoryFormat::R8g8b8a8Premultiplied,
        PixelFormatId::BgraPremultiplied => MemoryFormat::B8g8r8a8Premultiplied,
        PixelFormatId::Rgb => MemoryFormat::R8g8b8,
        PixelFormatId::Bgr => MemoryFormat::B8g8r8,
        PixelFormatId::Rgbx => MemoryFormat::R8g8b8x8,
        PixelFormatId::Bgrx => MemoryFormat::B8g8r8x8,
        PixelFormatId::Gray => MemoryFormat::G8,
    }
}
