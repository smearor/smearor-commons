use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;

/// The four corners of a [`crate::Rect`], as named fields instead of an
/// index-addressed array.
///
/// Field names follow the y-down screen convention: `top_left` is the
/// smallest corner, `bottom_right` the largest.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Corners<U> {
    /// Top-left corner (the rect's `min`).
    pub top_left: Vec2<U>,
    /// Top-right corner (`right`, `min.y`).
    pub top_right: Vec2<U>,
    /// Bottom-right corner (the rect's `max`).
    pub bottom_right: Vec2<U>,
    /// Bottom-left corner (`min.x`, `bottom`).
    pub bottom_left: Vec2<U>,
}

impl<U> Corners<U> {
    /// The corners as an array in declaration order
    /// (`top_left`, `top_right`, `bottom_right`, `bottom_left`).
    pub fn into_array(self) -> [Vec2<U>; 4] {
        [self.top_left, self.top_right, self.bottom_right, self.bottom_left]
    }
}

impl<U> From<Corners<U>> for [Vec2<U>; 4] {
    /// Same order as [`Corners::into_array`].
    fn from(corners: Corners<U>) -> Self {
        corners.into_array()
    }
}

impl<U> IntoIterator for Corners<U> {
    type Item = Vec2<U>;
    type IntoIter = std::array::IntoIter<Vec2<U>, 4>;

    /// Iterates in declaration order (starting at `top_left`).
    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}
