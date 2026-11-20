//! Error type for the GTK texture bridge.

/// Failures of [`PixelBuffer::decode_image`](crate::PixelBuffer::decode_image).
#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    /// `gdk::Texture::from_bytes` could not decode the image data.
    #[error("could not decode image data: {0}")]
    Decode(#[from] gtk4::glib::Error),
    /// The texture's [`MemoryFormat`](gtk4::gdk::MemoryFormat) has no
    /// `PixelFormatId` counterpart (16/32-bit or float formats) —
    /// raised through `TryFrom<MemoryFormat> for PixelFormatId`.
    #[error("unsupported texture memory format {0:?}")]
    UnsupportedFormat(gtk4::gdk::MemoryFormat),
}
