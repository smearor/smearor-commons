use super::pixel_format::luma;
use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Single luminance byte - `image::Luma`, grayscale masks and coverage maps.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gray;

impl PixelFormat for Gray {
    const BYTES_PER_PIXEL: usize = 1;
    const HAS_ALPHA: bool = false;
    const ID: PixelFormatId = PixelFormatId::Gray;

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, _a: u8) {
        dst[0] = luma(r, g, b);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[0], src[0], src[0], 255]
    }
}
