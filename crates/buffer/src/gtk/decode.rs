//! Image decode through gdk — the generalized form of the
//! chess `decode_texture` helper: gdk handles PNG/JPEG/etc. decoding,
//! the texture pixels land in a `PixelBuffer`.

use gtk4::gdk::Texture;
use gtk4::glib;

use crate::PixelBuffer;
use crate::Rgba;
use crate::gtk::TextureError;

impl PixelBuffer<Rgba> {
    /// Decodes `bytes` (any image format gdk understands: PNG, JPEG, …)
    /// into a straight-alpha [`PixelBuffer<Rgba>`].
    ///
    /// Fails when gdk cannot decode the data ([`TextureError::Decode`]).
    /// Consumers needing a cairo surface can follow with their own
    /// buffer→surface bridge downstream.
    pub fn decode_image(bytes: &[u8]) -> Result<Self, TextureError> {
        let texture = Texture::from_bytes(&glib::Bytes::from(bytes))?;
        Ok(Self::from_texture(&texture))
    }
}
