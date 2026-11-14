use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Blue, green, red byte order without alpha - BMP and device-framebuffer layouts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bgr;

impl PixelFormat for Bgr {
    const BYTES_PER_PIXEL: usize = 3;
    const HAS_ALPHA: bool = false;
    const ID: PixelFormatId = PixelFormatId::Bgr;
}
