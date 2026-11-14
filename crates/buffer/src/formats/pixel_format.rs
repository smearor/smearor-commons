use crate::PixelFormatId;

/// The byte layout of a pixel format - implemented by marker types.
///
/// Implementors are zero-sized tags carried as `PhantomData` on
/// [`crate::PixelBuffer`] and the buffer views; the associated
/// constants supply the byte arithmetic and a runtime identifier.
pub trait PixelFormat {
    /// Bytes per pixel.
    const BYTES_PER_PIXEL: usize;

    /// Whether the format carries an alpha channel - `false` for
    /// formats without alpha and for padding-channel formats like
    /// [`crate::Rgbx`], where the fourth byte is ignored.
    const HAS_ALPHA: bool;

    /// The runtime identifier for diagnostics and wire formats.
    const ID: PixelFormatId;
}
