use crate::RotationRad;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::TAU;

/// A directed angular interval on the circle - a `width`-wide span
/// starting at the `start` direction and reaching around the full
/// turn.
///
/// This is an angular interval, not a positioned curve: an arc placed
/// on a circle is a [`crate::Circle`] plus an `Arc`. Both endpoints are
/// inclusive. The type is a technical primitive without validation:
/// `width` should be in `0..=2*PI` for meaningful arcs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Arc {
    /// The starting direction of the interval.
    pub start: RotationRad,
    /// The unsigned angular span of the interval.
    pub width: RotationRad,
}

impl Arc {
    /// The full turn - an arc covering every direction.
    pub const FULL: Self = Self {
        start: RotationRad::ZERO,
        width: RotationRad(TAU),
    };

    /// Creates an arc from its start direction and span.
    pub const fn new(start: RotationRad, width: RotationRad) -> Self {
        Self { start, width }
    }

    /// The directed arc from `from` to `to` going the positive way
    /// around the circle - the wrap across the full turn needs no
    /// special case: `spanning(350deg, 10deg)` spans `20deg`.
    pub fn spanning(from: RotationRad, to: RotationRad) -> Self {
        Self::new(from.normalized(), (to - from).normalized())
    }

    /// The direction the arc ends at.
    pub fn end(&self) -> RotationRad {
        (self.start + self.width).normalized()
    }

    /// Whether `direction` lies inside the interval, boundary
    /// inclusive. A [`Arc::FULL`]-width arc contains every direction.
    pub fn contains(&self, direction: RotationRad) -> bool {
        (direction - self.start).normalized().as_f64() <= self.width.as_f64()
    }

    /// The direction at the normalized position `t` along the arc -
    /// `at(0.0)` is `start`, `at(1.0)` is [`Arc::end`]. Values outside
    /// `0..=1` extrapolate around the turn.
    pub fn at(&self, t: f64) -> RotationRad {
        (self.start + RotationRad::new(self.width.as_f64() * t)).normalized()
    }

    /// The direction halfway along the arc.
    pub fn midpoint(&self) -> RotationRad {
        self.at(0.5)
    }

    /// The arcs between consecutive directions of a sorted set,
    /// including the wrap-around arc from the last back to the first.
    ///
    /// A single direction yields one [`Arc::FULL`] arc starting at it
    /// (the gap going once around back to itself); an empty set
    /// yields no arcs.
    pub fn gaps_between(sorted: &[RotationRad]) -> Vec<Arc> {
        if sorted.len() == 1 {
            return vec![Self::new(sorted[0].normalized(), RotationRad::new(TAU))];
        }
        let mut gaps = Vec::with_capacity(sorted.len());
        for window in sorted.windows(2) {
            gaps.push(Self::spanning(window[0], window[1]));
        }
        if let (Some(&first), Some(&last)) = (sorted.first(), sorted.last()) {
            // The wrap goes once around back to `first`: a zero span
            // (identical first and last) is the full turn, not nothing.
            let wrap = Self::spanning(last, first);
            gaps.push(if wrap.width.is_zero() {
                Self::new(first.normalized(), RotationRad::new(TAU))
            } else {
                wrap
            });
        }
        gaps
    }
}

#[cfg(test)]
mod tests {
    use super::Arc;
    use crate::RotationRad;

    /// `spanning` crosses the full-turn wrap without a special case.
    #[test]
    fn spanning_wraps_the_turn() {
        let arc = Arc::spanning(RotationRad::from_degrees(350.0), RotationRad::from_degrees(10.0));
        assert!((arc.width.to_degrees() - 20.0).abs() < 1e-9);
        assert!((arc.end().to_degrees() - 10.0).abs() < 1e-9);
    }

    /// Membership is boundary-inclusive and wrap-aware.
    #[test]
    fn contains_across_wrap() {
        let arc = Arc::spanning(RotationRad::from_degrees(350.0), RotationRad::from_degrees(10.0));
        assert!(arc.contains(RotationRad::from_degrees(0.0)));
        assert!(arc.contains(RotationRad::from_degrees(350.0)));
        assert!(arc.contains(RotationRad::from_degrees(10.0)));
        assert!(!arc.contains(RotationRad::from_degrees(180.0)));
        assert!(!arc.contains(RotationRad::from_degrees(30.0)));
        assert!(Arc::FULL.contains(RotationRad::from_degrees(123.0)));
    }

    /// `at` interpolates inside the arc; `midpoint` is `at(0.5)`.
    #[test]
    fn at_interpolates() {
        let arc = Arc::spanning(RotationRad::from_degrees(350.0), RotationRad::from_degrees(10.0));
        assert!((arc.at(0.0).to_degrees() - 350.0).abs() < 1e-9);
        assert!((arc.at(1.0).to_degrees() - 10.0).abs() < 1e-9);
        assert!((arc.midpoint().to_degrees() - 0.0).abs() < 1e-9);
    }

    /// Sorted directions split the circle into consecutive gaps.
    #[test]
    fn gaps_between_sorted_directions() {
        let angles = [
            RotationRad::from_degrees(0.0),
            RotationRad::from_degrees(90.0),
            RotationRad::from_degrees(270.0),
        ];
        let gaps = Arc::gaps_between(&angles);
        assert_eq!(gaps.len(), 3);
        assert!((gaps[0].width.to_degrees() - 90.0).abs() < 1e-9);
        assert!((gaps[1].width.to_degrees() - 180.0).abs() < 1e-9);
        // wrap gap 270 -> 0(+360) spans 90 degrees
        assert!((gaps[2].width.to_degrees() - 90.0).abs() < 1e-9);
        assert_eq!(gaps[2].start.to_degrees(), 270.0);
    }

    /// One direction leaves a single full-turn gap; none leaves none.
    #[test]
    fn gaps_between_edge_cases() {
        let single = Arc::gaps_between(&[RotationRad::from_degrees(45.0)]);
        assert_eq!(single.len(), 1);
        assert!((single[0].width.to_degrees() - 360.0).abs() < 1e-9);
        assert!((single[0].start.to_degrees() - 45.0).abs() < 1e-9);
        assert!(Arc::gaps_between(&[]).is_empty());

        // All directions identical: the wrap gap is the full turn.
        let same = Arc::gaps_between(&[RotationRad::from_degrees(90.0), RotationRad::from_degrees(90.0)]);
        assert_eq!(same.len(), 2);
        assert!(same[0].width.is_zero());
        assert!((same[1].width.to_degrees() - 360.0).abs() < 1e-9);
    }
}
