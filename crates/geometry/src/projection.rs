//! `Projection` - a point's orthogonal projection onto a segment.

use crate::Fraction;
use crate::Vec2;

/// The orthogonal projection of a point onto a segment interior:
/// where along the segment it lands and at which point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection<U> {
    /// The fraction along the segment: the projected point is
    /// `from + fraction * (to - from)`. Values outside `0.0..=1.0`
    /// mean the projection lands off the segment - deliberately
    /// unclamped, like [`Fraction`] itself.
    pub fraction: Fraction,
    /// The projected point on the segment, in unit `U`.
    pub point: Vec2<U>,
}
