use crate::PixelFormat;
use crate::PixelFormatId;
use serde::Deserialize;
use serde::Serialize;

/// Blue, green, red, alpha byte order with premultiplied alpha - the
/// `cairo::Format::ARgb32` layout on little-endian and
/// `gdk::MemoryFormat::B8g8r8a8_PREMULTIPLIED`.
///
/// Color channels are stored multiplied by alpha; the marker keeps
/// straight-alpha buffers from flowing into premultiplied code paths
/// (and vice versa) at the type level.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgraPremultiplied;

impl PixelFormat for BgraPremultiplied {
    const BYTES_PER_PIXEL: usize = 4;
    const HAS_ALPHA: bool = true;
    const ID: PixelFormatId = PixelFormatId::BgraPremultiplied;
}
