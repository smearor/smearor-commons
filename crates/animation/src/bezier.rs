//! A trajectory along a bezier path - the animated value is the
//! point on the curve at progress `t`, evaluated by de Casteljau.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// A bezier path trajectory: the value at progress `t` is the
/// point on the curve through `points`, evaluated by recursive
/// de Casteljau interpolation - two points give a line, three a
/// quadratic and four a cubic bezier.
///
/// Generic over the point type `P` - `BezierPath<Vec2<Pixels>>`
/// moves along a screen curve, `BezierPath<Vec3>` along a spatial
/// one. For a trajectory along a curve *between* two poses combine
/// with [`crate::Mirror`] or sequence it via [`crate::Keyframes`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct BezierPath<P = Vec2<()>> {
    /// The control points of the curve - never empty.
    pub points: Vec<P>,
}

impl<P> BezierPath<P> {
    /// Creates a bezier trajectory from its `first` control point
    /// and the `rest`.
    pub fn new(first: P, rest: impl IntoIterator<Item = P>) -> Self {
        let mut points: Vec<P> = rest.into_iter().collect();
        points.insert(0, first);
        Self { points }
    }
}

impl<P: Interpolate + Copy> Trajectory for BezierPath<P> {
    type Value = P;

    fn value_at(&self, t: Fraction) -> P {
        let mut level = self.points.clone();
        while level.len() > 1 {
            level = level.windows(2).map(|pair| pair[0].interpolate(pair[1], t)).collect();
        }
        level[0]
    }
}

#[cfg(test)]
mod tests {
    use super::BezierPath;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;
    use smearor_commons_geometry::Vec3;

    /// The endpoints of the path are the endpoints of the value.
    #[test]
    fn the_curve_anchors_at_its_endpoints() {
        let path = BezierPath::new(Vec2::<()>::new(0.0, 0.0), [Vec2::<()>::new(1.0, 2.0), Vec2::<()>::new(2.0, 0.0)]);
        assert_eq!(path.value_at(Fraction::ZERO), Vec2::<()>::new(0.0, 0.0));
        assert_eq!(path.value_at(Fraction::ONE), Vec2::<()>::new(2.0, 0.0));
    }

    /// The quadratic midpoint lies on the curve, not on the chord.
    #[test]
    fn the_midpoint_follows_the_curve() {
        let path = BezierPath::new(Vec2::<()>::new(0.0, 0.0), [Vec2::<()>::new(1.0, 2.0), Vec2::<()>::new(2.0, 0.0)]);
        // Quadratic midpoint: 0.25*a + 0.5*c + 0.25*b.
        assert_eq!(path.value_at(Fraction::HALF), Vec2::<()>::new(1.0, 1.0));
    }

    /// The generic point type lifts the same curve into 3D.
    #[test]
    fn three_dimensional_points_follow_the_curve() {
        let path = BezierPath::new(Vec3::<()>::new(0.0, 0.0, 0.0), [Vec3::<()>::new(2.0, 0.0, 1.0)]);
        assert_eq!(path.value_at(Fraction::HALF), Vec3::<()>::new(1.0, 0.0, 0.5));
        assert_eq!(path.value_at(Fraction::ONE), Vec3::<()>::new(2.0, 0.0, 1.0));
    }
}
