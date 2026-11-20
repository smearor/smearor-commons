//! Error type for the GTK texture bridge.

/// Failures of [`crate::gtk::decode_image`].
#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    /// `gdk::Texture::from_bytes` could not decode the image data.
    #[error("could not decode image data: {0}")]
    Decode(#[from] gtk4::glib::Error),
}
