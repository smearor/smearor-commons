use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// Runtime identifier for pixel formats - the value-level mirror of
/// the [`crate::PixelFormat`] markers for diagnostics and wire formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormatId {
    /// `r, g, b, a` byte order.
    Rgba,
    /// `b, g, r, a` byte order.
    Bgra,
    /// `r, g, b` byte order - no alpha.
    Rgb,
    /// `b, g, r` byte order - no alpha.
    Bgr,
    /// `r, g, b, x` byte order - fourth byte is ignored padding.
    Rgbx,
    /// `b, g, r, x` byte order - fourth byte is ignored padding.
    Bgrx,
    /// Single luminance byte.
    Gray,
}

impl PixelFormatId {
    /// Bytes per pixel of the format - the runtime mirror of
    /// [`crate::PixelFormat::BYTES_PER_PIXEL`].
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Rgba | Self::Bgra | Self::Rgbx | Self::Bgrx => 4,
            Self::Rgb | Self::Bgr => 3,
            Self::Gray => 1,
        }
    }

    /// Whether the format carries an alpha channel - the runtime
    /// mirror of [`crate::PixelFormat::HAS_ALPHA`].
    pub const fn has_alpha(self) -> bool {
        matches!(self, Self::Rgba | Self::Bgra)
    }
}

impl Display for PixelFormatId {
    /// The uppercase format name (`"RGBA"`, `"BGRX"`, `"GRAY"`, ...).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgba => write!(f, "RGBA"),
            Self::Bgra => write!(f, "BGRA"),
            Self::Rgb => write!(f, "RGB"),
            Self::Bgr => write!(f, "BGR"),
            Self::Rgbx => write!(f, "RGBX"),
            Self::Bgrx => write!(f, "BGRX"),
            Self::Gray => write!(f, "GRAY"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PixelFormatId;
    use crate::Bgr;
    use crate::Bgra;
    use crate::Bgrx;
    use crate::Gray;
    use crate::PixelFormat;
    use crate::Rgb;
    use crate::Rgba;
    use crate::Rgbx;

    /// Every `PixelFormatId` mirrors its marker's trait constants.
    #[test]
    fn id_mirrors_marker_constants() {
        for (marker_bpp, marker_alpha, id) in [
            (Rgba::BYTES_PER_PIXEL, Rgba::HAS_ALPHA, PixelFormatId::Rgba),
            (Bgra::BYTES_PER_PIXEL, Bgra::HAS_ALPHA, PixelFormatId::Bgra),
            (Rgb::BYTES_PER_PIXEL, Rgb::HAS_ALPHA, PixelFormatId::Rgb),
            (Bgr::BYTES_PER_PIXEL, Bgr::HAS_ALPHA, PixelFormatId::Bgr),
            (Rgbx::BYTES_PER_PIXEL, Rgbx::HAS_ALPHA, PixelFormatId::Rgbx),
            (Bgrx::BYTES_PER_PIXEL, Bgrx::HAS_ALPHA, PixelFormatId::Bgrx),
            (Gray::BYTES_PER_PIXEL, Gray::HAS_ALPHA, PixelFormatId::Gray),
        ] {
            assert_eq!(id.bytes_per_pixel(), marker_bpp);
            assert_eq!(id.has_alpha(), marker_alpha);
        }
    }
}
