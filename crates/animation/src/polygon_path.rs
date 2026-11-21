//! A closed path through vertices - the value travels each edge
//! in equal time.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// A polygon path: the value travels the edges `points[0] →
/// points[1] → … → points[n-1] → points[0]` at equal time per
/// edge - patrol routes, marquee loops, shape tracing.
///
/// Generic over the point type - `PolygonPath<Vec2<Pixels>>` walks
/// screen vertices, `PolygonPath<Vec3>` a spatial polygon. Sharper
/// than [`crate::CatmullRom`]: corners stay corners.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PolygonPath<P = Vec2<()>> {
    /// The vertices to travel - two or more.
    pub points: Vec<P>,
}

impl<P> PolygonPath<P> {
    /// Creates a closed path over `points`.
    pub fn new(points: impl IntoIterator<Item = P>) -> Self {
        let points: Vec<P> = points.into_iter().collect();
        Self { points }
    }
}

impl<P: Interpolate + Copy> Trajectory for PolygonPath<P> {
    type Value = P;

    fn value_at(&self, t: Fraction) -> P {
        let edges = self.points.len().max(1);
        let position = t.as_f64().clamp(0.0, 1.0) * edges as f64;
        let edge = (position as usize).min(edges - 1);
        let from = self.points[edge];
        let to = self.points[(edge + 1) % edges];
        from.interpolate(to, Fraction::new(position - edge as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::PolygonPath;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// A triangle closes: start and end share the first vertex.
    #[test]
    fn the_path_closes_onto_itself() {
        let triangle = PolygonPath::new([Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(1.0, 0.0), Vec2::<()>::new(0.5, 1.0)]);
        assert_eq!(triangle.value_at(Fraction::ZERO), Vec2::<()>::new(0.0, 0.0));
        assert_eq!(triangle.value_at(Fraction::ONE), Vec2::<()>::new(0.0, 0.0));
    }

    /// Equal time per edge: the first third reaches vertex two.
    #[test]
    fn edges_get_equal_time() {
        let triangle = PolygonPath::new([Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(1.0, 0.0), Vec2::<()>::new(0.5, 1.0)]);
        assert_eq!(triangle.value_at(Fraction::new(1.0 / 3.0)), Vec2::<()>::new(1.0, 0.0));
    }
}
