//! `Segment` - a straight segment between two positions.

use crate::Fraction;
use crate::Polygon;
use crate::Projection;
use crate::Rect;
use crate::RotationRad;
use crate::Vec2;

/// A straight segment in unit `U` (`Vec2<Millimeters>` on a map,
/// `Vec2<Pixels>` on screen) - the geometry an edge draws between
/// its two endpoints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment<U> {
    /// Segment start position.
    pub from: Vec2<U>,
    /// Segment end position.
    pub to: Vec2<U>,
}

impl<U> Segment<U> {
    /// A segment from `from` to `to`.
    pub const fn new(from: Vec2<U>, to: Vec2<U>) -> Self {
        Self { from, to }
    }
}

impl<U: Copy> Segment<U> {
    /// The axis-aligned bounding box of both endpoints.
    pub fn bounding_box(&self) -> Rect<U> {
        let min = Vec2::new(self.from.x.min(self.to.x), self.from.y.min(self.to.y));
        let max = Vec2::new(self.from.x.max(self.to.x), self.from.y.max(self.to.y));
        min.rect(max - min)
    }

    /// The segment's center point.
    pub fn midpoint(&self) -> Vec2<U> {
        Vec2::new((self.from.x + self.to.x) / 2.0, (self.from.y + self.to.y) / 2.0)
    }

    /// The segment's direction as an angle (atan2 of the
    /// from→to vector).
    pub fn direction(&self) -> RotationRad {
        RotationRad::new((self.to.y - self.from.y).atan2(self.to.x - self.from.x))
    }

    /// The arrowhead drawn at `to`: the filled triangle spanning
    /// `half_span` to either side of the shaft's direction, `head`
    /// long - a scalar in the segment's unit space (the same
    /// convention [`Vec2::from_polar`] uses for `radius`). Order is
    /// tip, wing, wing - one closed polygon for a solid fill.
    pub fn arrow_head(&self, head: f64, half_span: RotationRad) -> Polygon<U> {
        let angle = self.direction();
        Polygon::new(vec![
            self.to,
            self.to - Vec2::<U>::from_polar(head, angle - half_span),
            self.to - Vec2::<U>::from_polar(head, angle + half_span),
        ])
    }

    /// Strict crossing - `true` when the interiors of both segments
    /// intersect (shared endpoints are junctions, not crossings).
    pub fn crosses(&self, other: &Self) -> bool {
        let delta = |a: Vec2<U>, b: Vec2<U>| Vec2::<U>::new(b.x - a.x, b.y - a.y);
        let orient = |p: Vec2<U>, q: Vec2<U>, r: Vec2<U>| delta(p, q).cross::<f64>(&delta(p, r));
        orient(self.from, self.to, other.from) * orient(self.from, self.to, other.to) < 0.0
            && orient(other.from, other.to, self.from) * orient(other.from, other.to, self.to) < 0.0
    }

    /// The unclamped projection of `point` onto the segment's
    /// line - `None` only when the segment is degenerate.
    fn project(&self, point: &Vec2<U>) -> Option<Projection<U>> {
        let direction = self.to - self.from;
        let len_sq = direction.dot::<f64>(&direction);
        if len_sq == 0.0 {
            return None;
        }
        let fraction = Fraction::new((*point - self.from).dot::<f64>(&direction) / len_sq);
        Some(Projection {
            fraction,
            point: self.from + direction * fraction,
        })
    }

    /// The projection of `point` onto the segment interior -
    /// `None` when the projection falls on an endpoint or
    /// outside the segment, or the segment is degenerate. Unlike
    /// `interior_distance` the endpoints are excluded (a snapped
    /// endpoint is no cut point).
    pub fn interior_projection(&self, point: &Vec2<U>) -> Option<Projection<U>> {
        let projection = self.project(point)?;
        (Fraction::new(1e-6)..=Fraction::new(1.0 - 1e-6))
            .contains(&projection.fraction)
            .then_some(projection)
    }

    /// Perpendicular distance of `point` to the segment when the
    /// closest point lies between the endpoints - `None` when the
    /// projection falls beyond either end or the segment is
    /// degenerate.
    pub fn interior_distance(&self, point: &Vec2<U>) -> Option<U>
    where
        U: From<f64>,
    {
        let projection = self.project(point)?;
        (Fraction::ZERO..=Fraction::ONE)
            .contains(&projection.fraction)
            .then(|| point.distance(&projection.point))
    }

    /// The nearest point on the segment to `point` plus its
    /// `fraction` along `from -> to` (clamped to `[0, 1]`) -
    /// `None` for a degenerate segment.
    pub fn nearest(&self, point: &Vec2<U>) -> Option<Projection<U>> {
        let fraction = self.project(point)?.fraction.clamped();
        Some(Projection {
            fraction,
            point: self.from + (self.to - self.from) * fraction,
        })
    }

    /// The distance from `point` to the segment - the
    /// perpendicular distance when the projection lands inside,
    /// the nearer endpoint distance otherwise.
    pub fn distance(&self, point: &Vec2<U>) -> U
    where
        U: From<f64>,
    {
        self.nearest(point)
            .map_or_else(|| point.distance(&self.from), |nearest| point.distance(&nearest.point))
    }

    /// The point where the two segments strictly cross - `None`
    /// when they do not (a mere touch counts as a junction, not
    /// a crossing).
    pub fn crossing_point(&self, other: &Self) -> Option<Vec2<U>> {
        self.crosses(other).then(|| self.intersection(other)).flatten()
    }

    /// The parameter `t` where the line `anchor + axis * t` meets
    /// this segment - `None` when they are parallel or the meeting
    /// lies off the segment.
    pub fn param_on_line(&self, anchor: &Vec2<U>, axis: &Vec2<U>) -> Option<U>
    where
        U: From<f64>,
    {
        let edge_direction = self.to - self.from;
        let denom = axis.cross::<f64>(&edge_direction);
        if denom.abs() < 1e-9 {
            return None;
        }
        let offset = self.from - *anchor;
        let s = axis.cross::<f64>(&offset) / -denom;
        if !(0.0..=1.0).contains(&s) {
            return None;
        }
        Some(U::from(offset.cross::<f64>(&edge_direction) / denom))
    }

    /// The point where the two segments meet - `None` when they are
    /// parallel, degenerate or the meeting lies outside either
    /// segment.
    pub fn intersection(&self, other: &Self) -> Option<Vec2<U>> {
        let direction = Vec2::<U>::new(self.to.x - self.from.x, self.to.y - self.from.y);
        let other_direction = Vec2::<U>::new(other.to.x - other.from.x, other.to.y - other.from.y);
        let denom = direction.cross::<f64>(&other_direction);
        if denom == 0.0 {
            return None;
        }
        let offset = Vec2::<U>::new(other.from.x - self.from.x, other.from.y - self.from.y);
        let t = offset.cross::<f64>(&other_direction) / denom;
        let u = offset.cross::<f64>(&direction) / denom;
        if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
            return None;
        }
        Some(Vec2::new(self.from.x + t * direction.x, self.from.y + t * direction.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The arrowhead triangle points along the segment direction.
    #[test]
    fn arrow_head_spans_shaft_direction() {
        let shaft = Segment::new(Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(10.0, 0.0));
        let head = shaft.arrow_head(4.0, RotationRad::from(0.5));
        let [tip, wing_a, wing_b] = head.0[..] else {
            panic!("triangle");
        };
        assert_eq!(tip, Vec2::new(10.0, 0.0));
        assert!((wing_a.x - (10.0 - 4.0 * 0.5_f64.cos())).abs() < 1e-12);
        assert!((wing_a.y - (-4.0 * (-0.5_f64).sin())).abs() < 1e-12);
        assert!((wing_b.y - (-4.0 * 0.5_f64.sin())).abs() < 1e-12);
    }

    use crate::Millimeters;

    /// `midpoint` averages both endpoints; `direction` is the
    /// atan2 of the from→to vector.
    #[test]
    fn midpoint_and_direction() {
        let segment = Segment::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        assert_eq!(segment.midpoint(), Vec2::new(5.0, 0.0));
        assert_eq!(segment.direction().as_f64(), 0.0);
    }

    /// `crosses` is strict: interior intersections count, shared
    /// endpoints do not.
    #[test]
    fn crosses_interior_only() {
        let horizontal = Segment::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        let crossing = Segment::<Millimeters>::new(Vec2::new(5.0, -5.0), Vec2::new(5.0, 5.0));
        let touching = Segment::<Millimeters>::new(Vec2::new(10.0, 0.0), Vec2::new(10.0, 5.0));
        assert!(horizontal.crosses(&crossing));
        assert!(!horizontal.crosses(&touching));
        assert_eq!(horizontal.crossing_point(&crossing), Some(Vec2::new(5.0, 0.0)));
        assert_eq!(horizontal.crossing_point(&touching), None);
    }

    /// `nearest` clamps the fraction to [0, 1]; `distance` falls
    /// back to the nearer endpoint.
    #[test]
    fn nearest_clamps_to_endpoints() {
        let segment = Segment::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        let nearest = segment.nearest(&Vec2::new(15.0, 3.0));
        assert!(nearest.is_some());
        if let Some(nearest) = nearest {
            assert_eq!(nearest.fraction, Fraction::ONE);
            assert_eq!(nearest.point, Vec2::new(10.0, 0.0));
        }
        let distance = segment.distance(&Vec2::new(15.0, 4.0));
        assert!((distance.as_f64() - 41.0_f64.sqrt()).abs() < 1e-9);
        assert_eq!(segment.interior_distance(&Vec2::new(15.0, 4.0)), None);
    }

    /// `interior_projection` excludes the endpoints and degenerate
    /// segments.
    #[test]
    fn interior_projection_excludes_endpoints() {
        let segment = Segment::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        assert!(segment.interior_projection(&Vec2::new(5.0, 3.0)).is_some());
        assert!(segment.interior_projection(&Vec2::new(0.0, 3.0)).is_none());
        let degenerate = Segment::<Millimeters>::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0));
        assert!(degenerate.interior_projection(&Vec2::new(1.0, 1.0)).is_none());
        assert!(degenerate.nearest(&Vec2::new(1.0, 1.0)).is_none());
    }

    /// `intersection` returns the meeting point inside both
    /// segments, `None` for parallel or off-segment meetings.
    #[test]
    fn intersection_inside_both() {
        let horizontal = Segment::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        let crossing = Segment::<Millimeters>::new(Vec2::new(5.0, -5.0), Vec2::new(5.0, 5.0));
        let parallel = Segment::<Millimeters>::new(Vec2::new(0.0, 1.0), Vec2::new(10.0, 1.0));
        assert_eq!(horizontal.intersection(&crossing), Some(Vec2::new(5.0, 0.0)));
        assert_eq!(horizontal.intersection(&parallel), None);
    }

    /// `param_on_line` gives the `t` where `anchor + axis * t`
    /// meets the segment.
    #[test]
    fn param_on_line_meets_inside() {
        let segment = Segment::<Millimeters>::new(Vec2::new(0.0, 5.0), Vec2::new(10.0, 5.0));
        let t = segment.param_on_line(&Vec2::new(5.0, 0.0), &Vec2::new(0.0, 1.0));
        assert_eq!(t, Some(Millimeters::new(5.0)));
        let miss = segment.param_on_line(&Vec2::new(15.0, 0.0), &Vec2::new(0.0, 1.0));
        assert_eq!(miss, None);
    }
}
