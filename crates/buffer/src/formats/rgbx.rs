use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Red, green, blue with an ignored fourth byte - OpenGL `RGBX`, `image::Rgba`-shaped buffers without alpha semantics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgbx;

impl PixelFormat for Rgbx {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = false;
    const ID: PixelFormatId = PixelFormatId::Rgbx;

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, _a: u8) {
        dst.copy_from_slice(&[r, g, b, 255]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[0], src[1], src[2], 255]
    }
}
