use serde::Deserialize;
use serde::Serialize;

/// One horizontal strip of a rasterized rotated rectangle.
///
/// Produced by [`crate::Rect::rotated_region_scanlines`]. Coordinates are
/// relative to the rect's center and one pixel high; callers add the
/// actual center to map strips onto integer regions such as
/// `cairo::RectangleInt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scanline {
    /// Scanline y-coordinate relative to the rect's center.
    pub y: i32,
    /// Left x edge relative to the center (inclusive).
    pub x_start: i32,
    /// Right x edge relative to the center (inclusive).
    pub x_end: i32,
}

impl Scanline {
    /// Width of the strip in pixels (`x_end - x_start`).
    pub fn width(&self) -> i32 {
        self.x_end - self.x_start
    }
}
