//! `PixelBuffer` ⇄ `gdk::Texture` conversion as inherent methods.
//!
//! Upload ([`PixelBuffer::to_memory_texture`]) is lossless: every
//! [`PixelFormatId`] has a byte-exact [`MemoryFormat`] counterpart via
//! `From<PixelFormatId>`, including the premultiplied variants (no
//! unpremultiply round-trip).
//!
//! Download ([`PixelBuffer::from_texture`]) always yields `ARgb32` —
//! i.e. premultiplied BGRA on little-endian — because
//! `gdk_texture_download` is defined as "the format cairo uses":
//! four bytes per pixel regardless of the texture's stored format.
//! The pixels are converted to straight-alpha RGBA through the
//! [`PixelFormat`] channel interface.

use gtk4::gdk::MemoryTexture;
use gtk4::gdk::Texture;
use gtk4::glib;
use gtk4::prelude::TextureExt;
use gtk4::prelude::TextureExtManual;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;

use crate::BgraPremultiplied;
use crate::PixelBuffer;
use crate::PixelFormat;
use crate::Rgba;

impl<F: PixelFormat> PixelBuffer<F> {
    /// Uploads the pixels as a [`MemoryTexture`] — the image bridge
    /// used by every GTK consumer that displays software-rendered
    /// content (demo columns, launcher icon widgets).
    ///
    /// The texture owns a copy of the pixel bytes; the buffer may be
    /// dropped or reused afterwards.
    pub fn to_memory_texture(&self) -> MemoryTexture {
        let size = self.size();
        let texture_size = Vec2::from(size);
        let bytes = glib::Bytes::from(self.pixels());
        MemoryTexture::new(texture_size.x, texture_size.y, F::ID.into(), &bytes, size.x as usize * F::BYTES_PER_PIXEL)
    }
}

impl PixelBuffer<Rgba> {
    /// Downloads `texture`'s pixels into a packed straight-alpha
    /// [`PixelBuffer<Rgba>`], unpremultiplying through the
    /// [`PixelFormat`] channel interface.
    ///
    /// `gdk_texture_download` always yields `ARgb32` pixels —
    /// premultiplied BGRA in memory order — so this function is
    /// infallible and ignores [`Texture::format`], which reports the
    /// *stored* format, not the download format.
    pub fn from_texture(texture: &Texture) -> Self {
        let size = Size::new(texture.width() as u32, texture.height() as u32);
        let stride = size.x as usize * BgraPremultiplied::BYTES_PER_PIXEL;
        let mut pixels = vec![0u8; size.y as usize * stride];
        texture.download(&mut pixels, stride);

        let mut buffer = PixelBuffer::<Rgba>::new(size);
        for (s, d) in pixels.as_chunks::<4>().0.iter().zip(buffer.pixels_mut().as_chunks_mut::<4>().0.iter_mut()) {
            let [r, g, b, a] = BgraPremultiplied::read_pixel(s);
            Rgba::write_pixel(d, r, g, b, a);
        }
        buffer
    }
}
