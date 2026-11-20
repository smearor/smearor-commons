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

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, _a: u8) {
        dst.copy_from_slice(&[r, g, b]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[0], src[1], src[2], 255]
    }
}
