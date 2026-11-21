//! `Barycentric` - the three vertex weights of a point in a triangle.

use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;

/// Barycentric coordinates `a`, `b`, `c` of a point relative to the
/// three vertices of a triangle: `point = a * A + b * B + c * C`.
///
/// [`Barycentric::new`] normalizes the weights so they always sum to
/// `1`; a point is inside the triangle iff all weights are
/// non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Barycentric {
    /// The weight of the first vertex.
    pub a: f64,
    /// The weight of the second vertex.
    pub b: f64,
    /// The weight of the third vertex.
    pub c: f64,
}

impl Barycentric {
    /// Normalized barycentric weights summing to `1`. A zero or
    /// negative sum maps to the centroid weights.
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        let sum = a + b + c;
        if sum > 0.0 {
            Self {
                a: a / sum,
                b: b / sum,
                c: c / sum,
            }
        } else {
            Self {
                a: 1.0 / 3.0,
                b: 1.0 / 3.0,
                c: 1.0 / 3.0,
            }
        }
    }

    /// The barycentric coordinates of `point` in `triangle`, or
    /// `None` for a degenerate (zero-area) triangle. Use
    /// [`Self::is_inside`] for the point-in-triangle test.
    pub fn from_point<U>(point: &Vec2<U>, triangle: &[Vec2<U>; 3]) -> Option<Self> {
        let [t0, t1, t2] = triangle;
        let (e1x, e1y) = (t1.x - t0.x, t1.y - t0.y);
        let (e2x, e2y) = (t2.x - t0.x, t2.y - t0.y);
        let (dx, dy) = (point.x - t0.x, point.y - t0.y);
        let det = e1x * e2y - e1y * e2x;
        if det.abs() < f64::EPSILON {
            return None;
        }
        let b = (dx * e2y - e2x * dy) / det;
        let c = (e1x * dy - dx * e1y) / det;
        Some(Self { a: 1.0 - b - c, b, c })
    }

    /// Whether the point lies inside or on the triangle - all
    /// weights non-negative.
    pub fn is_inside(&self) -> bool {
        self.a >= -1e-9 && self.b >= -1e-9 && self.c >= -1e-9
    }

    /// The point with these weights in `triangle`.
    pub fn point<U>(&self, triangle: &[Vec2<U>; 3]) -> Vec2<U> {
        let [a, b, c] = triangle;
        Vec2::new(a.x * self.a + b.x * self.b + c.x * self.c, a.y * self.a + b.y * self.b + c.y * self.c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle() -> [Vec2; 3] {
        [Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0), Vec2::new(0.0, 4.0)]
    }

    #[test]
    fn centroid_is_center_of_mass() {
        let bary = Barycentric::from_point(&Vec2::new(4.0 / 3.0, 4.0 / 3.0), &triangle()).unwrap();
        assert!(bary.is_inside());
        for w in [bary.a, bary.b, bary.c] {
            assert!((w - 1.0 / 3.0).abs() < 1e-9);
        }
    }

    #[test]
    fn vertices_map_to_unit_weights() {
        let tri = triangle();
        for (index, vertex) in tri.iter().enumerate() {
            let bary = Barycentric::from_point(vertex, &tri).unwrap();
            let weight = [bary.a, bary.b, bary.c][index];
            assert!((weight - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn outside_point_is_detected() {
        let bary = Barycentric::from_point(&Vec2::new(3.0, 3.0), &triangle()).unwrap();
        assert!(!bary.is_inside());
    }

    #[test]
    fn point_roundtrips() {
        let tri = triangle();
        let bary = Barycentric::new(0.5, 0.25, 0.25);
        let point = bary.point(&tri);
        let back = Barycentric::from_point(&point, &tri).unwrap();
        assert!((bary.a - back.a).abs() < 1e-9 && (bary.b - back.b).abs() < 1e-9 && (bary.c - back.c).abs() < 1e-9);
    }
}
