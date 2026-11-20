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

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, _a: u8) {
        dst.copy_from_slice(&[b, g, r]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[2], src[1], src[0], 255]
    }
}
