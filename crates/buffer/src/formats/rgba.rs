use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Red, green, blue, alpha byte order - the canonical
/// software-rendering layout (`image::RgbaImage`, `pixel-drawing`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgba;

impl PixelFormat for Rgba {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = true;
    const ID: PixelFormatId = PixelFormatId::Rgba;
}
