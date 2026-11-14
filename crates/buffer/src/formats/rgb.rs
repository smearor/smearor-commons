use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Red, green, blue byte order without alpha - decoded `image::Rgb` data, JPEG and BMP payloads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb;

impl PixelFormat for Rgb {
    const BYTES_PER_PIXEL: usize = 3;
    const HAS_ALPHA: bool = false;
    const ID: PixelFormatId = PixelFormatId::Rgb;
}
