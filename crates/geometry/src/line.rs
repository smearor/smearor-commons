//! `Line` - a parametric point + direction line.

use crate::Vec2;

/// A parametric line `origin + t * direction` in unit `U` - the
/// shape two offset edges intersect along.
#[derive(Debug, Clone, Copy)]
pub struct Line<U> {
    /// A point the line runs through.
    pub origin: Vec2<U>,
    /// The line's direction (any scale).
    pub direction: Vec2<U>,
}

impl<U: Copy> Line<U> {
    /// A line through `origin` along `direction`.
    pub const fn new(origin: Vec2<U>, direction: Vec2<U>) -> Self {
        Self { origin, direction }
    }

    /// The intersection of the two lines; `None` for
    /// (near-)parallel ones.
    pub fn intersect(&self, other: &Line<U>) -> Option<Vec2<U>> {
        let denom = self.direction.cross::<f64>(&other.direction);
        if denom.abs() < 1e-9 {
            return None;
        }
        let offset = other.origin - self.origin;
        let s = offset.cross::<f64>(&other.direction) / denom;
        Some(self.origin + self.direction * s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Millimeters;

    #[test]
    fn two_crossing_lines_meet_at_their_point() {
        let horizontal = Line::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let vertical = Line::<Millimeters>::new(Vec2::new(3.0, -5.0), Vec2::new(0.0, 1.0));
        assert_eq!(horizontal.intersect(&vertical), Some(Vec2::new(3.0, 0.0)));
    }

    #[test]
    fn scaled_directions_still_meet_at_the_point() {
        let diagonal = Line::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let other = Line::<Millimeters>::new(Vec2::new(4.0, 0.0), Vec2::new(-1.0, 1.0));
        assert_eq!(diagonal.intersect(&other), Some(Vec2::new(2.0, 2.0)));
    }

    #[test]
    fn parallel_lines_do_not_intersect() {
        let a = Line::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let b = Line::<Millimeters>::new(Vec2::new(0.0, 5.0), Vec2::new(1.0, 0.0));
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn collinear_lines_read_as_parallel() {
        let a = Line::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let b = Line::<Millimeters>::new(Vec2::new(3.0, 0.0), Vec2::new(2.0, 0.0));
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn nearly_parallel_lines_do_not_intersect() {
        let a = Line::<Millimeters>::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let b = Line::<Millimeters>::new(Vec2::new(0.0, 1.0), Vec2::new(1.0, 1e-10));
        assert_eq!(a.intersect(&b), None);
    }
}
