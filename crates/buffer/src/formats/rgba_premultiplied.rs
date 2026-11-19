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
}
