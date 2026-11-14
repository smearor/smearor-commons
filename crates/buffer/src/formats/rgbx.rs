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
}
