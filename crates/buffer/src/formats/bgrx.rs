use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Blue, green, red with an ignored fourth byte - cairo `FORMAT_RGB24` and X11 24-bit layouts on little-endian.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bgrx;

impl PixelFormat for Bgrx {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = false;
    const ID: PixelFormatId = PixelFormatId::Bgrx;
}
