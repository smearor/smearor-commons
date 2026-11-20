use super::pixel_format::premultiply;
use super::pixel_format::unpremultiply;
use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Red, green, blue, alpha byte order with premultiplied alpha - the
/// `gdk::MemoryFormat::R8g8b8a8_PREMULTIPLIED` layout and the format GDK
/// textures download in.
///
/// Color channels are stored multiplied by alpha; the marker keeps
/// straight-alpha buffers from flowing into premultiplied code paths
/// (and vice versa) at the type level.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbaPremultiplied;

impl PixelFormat for RgbaPremultiplied {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = true;
    const ID: PixelFormatId = PixelFormatId::RgbaPremultiplied;

    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, a: u8) {
        dst.copy_from_slice(&[premultiply(r, a), premultiply(g, a), premultiply(b, a), a]);
    }

    fn read_pixel(src: &[u8]) -> [u8; 4] {
        [
            unpremultiply(src[0], src[3]),
            unpremultiply(src[1], src[3]),
            unpremultiply(src[2], src[3]),
            src[3],
        ]
    }
}
