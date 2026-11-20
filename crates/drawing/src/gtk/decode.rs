//! Image decode through gdk — the generalized form of the
//! chess `decode_texture` helper: gdk handles PNG/JPEG/etc. decoding,
//! the texture pixels land in a `PixelBuffer`.

use gtk4::gdk::Texture;
use gtk4::glib;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;

use crate::gtk::TextureError;
use crate::gtk::texture::buffer_from_texture;

/// Decodes `bytes` (any image format gdk understands: PNG, JPEG, …)
/// into a straight-alpha [`PixelBuffer<Rgba>`].
///
/// Fails when gdk cannot decode the data ([`TextureError::Decode`]).
/// For a cairo surface instead, follow with
/// [`crate::cairo::image_surface_from_buffer`].
pub fn decode_image(bytes: &[u8]) -> Result<PixelBuffer<Rgba>, TextureError> {
    let texture = Texture::from_bytes(&glib::Bytes::from(bytes))?;
    Ok(buffer_from_texture(&texture))
}
