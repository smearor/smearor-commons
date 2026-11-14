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
}
