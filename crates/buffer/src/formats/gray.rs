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
}
