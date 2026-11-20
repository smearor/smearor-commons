use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Blue, green, red, alpha byte order - the Wayland SHM and
/// `gdk::MemoryFormat::B8g8r8a8` layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bgra;

impl PixelFormat for Bgra {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = true;
    const ID: PixelFormatId = PixelFormatId::Bgra;

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, a: u8) {
        dst.copy_from_slice(&[b, g, r, a]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [src[2], src[1], src[0], src[3]]
    }
}
