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

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, a: u8) {
        dst.copy_from_slice(&[r, g, b, a]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[0], src[1], src[2], src[3]]
    }
}
