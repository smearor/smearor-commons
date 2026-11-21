//! A Catmull-Rom spline trajectory - a smooth path through every
//! control point.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

/// A Catmull-Rom spline: the value travels through *all* points -
/// unlike a bezier, every control point lies on the curve. The
/// ends duplicate their neighbors, so the curve starts and stops
/// exactly at the first and last point.
///
/// Generic over the point type (`Vec2`, `Vec3`) - the segment
/// math runs through the standard Catmull-Rom to bezier
/// conversion and de Casteljau interpolation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CatmullRom<P = Vec2<()>> {
    /// The points the curve passes through - two or more.
    pub points: Vec<P>,
}

impl<P> CatmullRom<P> {
    /// Creates a spline through `points`.
    pub fn new(points: impl IntoIterator<Item = P>) -> Self {
        let points: Vec<P> = points.into_iter().collect();
        Self { points }
    }
}

/// The trait a point type needs for the Catmull-Rom segment math.
pub trait SplinePoint: Interpolate + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<f64, Output = Self> {}
impl<T> SplinePoint for T where T: Interpolate + Copy + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> {}

impl<P: SplinePoint> Trajectory for CatmullRom<P> {
    type Value = P;

    fn value_at(&self, t: Fraction) -> P {
        let points = &self.points;
        let last = points.len() - 1;
        let position = t.as_f64().clamp(0.0, 1.0) * last as f64;
        let segment = (position as usize).min(last - 1);
        let s = Fraction::new(position - segment as f64);
        // phantom endpoints duplicate the neighbors
        let at = |i: isize| points[(i.clamp(0, last as isize)) as usize];
        let (p0, p1, p2, p3) = (at(segment as isize - 1), at(segment as isize), at(segment as isize + 1), at(segment as isize + 2));
        // Catmull-Rom segment as bezier, then de Casteljau
        let b1 = p1 + (p2 - p0) * (1.0 / 6.0);
        let b2 = p2 - (p3 - p1) * (1.0 / 6.0);
        let m1 = p1.interpolate(b1, s);
        let m2 = b1.interpolate(b2, s);
        let m3 = b2.interpolate(p2, s);
        let n1 = m1.interpolate(m2, s);
        let n2 = m2.interpolate(m3, s);
        n1.interpolate(n2, s)
    }
}

#[cfg(test)]
mod tests {
    use super::CatmullRom;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The spline anchors exactly at the end points.
    #[test]
    fn the_spline_passes_through_its_endpoints() {
        let spline = CatmullRom::new([Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(1.0, 1.0), Vec2::<()>::new(2.0, 0.0)]);
        assert_eq!(spline.value_at(Fraction::ZERO), Vec2::<()>::new(0.0, 0.0));
        assert_eq!(spline.value_at(Fraction::ONE), Vec2::<()>::new(2.0, 0.0));
    }

    /// Interior control points lie on the curve - that is what
    /// sets Catmull-Rom apart from bezier paths.
    #[test]
    fn interior_points_lie_on_the_curve() {
        let spline = CatmullRom::new([Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(1.0, 1.0), Vec2::<()>::new(2.0, 0.0)]);
        assert_eq!(spline.value_at(Fraction::HALF), Vec2::<()>::new(1.0, 1.0));
    }
}
