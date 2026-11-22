//! A star polygon trajectory - `PolygonPath` visiting every
//! `step`-th vertex.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A star polygon `{n/step}`: travels `points` like
/// [`crate::PolygonPath`] but steps `step` vertices ahead per
/// edge - five points with `step = 2` draw the pentagram, six with
/// `step = 2` trace a triangle twice. `step = 1` is the plain
/// polygon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StarPath<P = Vec2<()>> {
    /// The vertices - two or more.
    pub points: Vec<P>,
    /// The vertex stride between connected points.
    pub step: usize,
}

impl<P> StarPath<P> {
    /// Creates a `{n/1}` polygon over `points`.
    pub fn new(points: impl IntoIterator<Item = P>) -> Self {
        let points: Vec<P> = points.into_iter().collect();
        Self { points, step: 1 }
    }

    /// Returns the path connecting every `step`-th vertex.
    pub fn with_step(mut self, step: usize) -> Self {
        self.step = step.max(1);
        self
    }
}

impl<U> StarPath<Vec2<U>> {
    /// A regular `vertices`-gon of `radius` around `center`, the
    /// first vertex on the +x axis - `with_step(2)` on five draws
    /// the pentagram.
    pub fn regular(center: Vec2<U>, radius: f64, vertices: usize) -> Self {
        let count = vertices.max(2);
        let points = (0..count)
            .map(|i| {
                let angle = i as f64 / count as f64 * TAU;
                Vec2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin())
            })
            .collect();
        Self { points, step: 1 }
    }
}

impl<P: Interpolate + Copy> Trajectory for StarPath<P> {
    type Value = P;

    fn value_at(&self, t: Fraction) -> P {
        let edges = self.points.len().max(1);
        let position = t.as_f64().clamp(0.0, 1.0) * edges as f64;
        let edge = (position as usize).min(edges - 1);
        let from = self.points[(edge * self.step) % edges];
        let to = self.points[((edge + 1) * self.step) % edges];
        from.interpolate(to, Fraction::new(position - edge as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::StarPath;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The pentagram skips a vertex: edge one spans 0 -> 2.
    #[test]
    fn the_step_skips_vertices() {
        let pentagram = StarPath::new([
            Vec2::<()>::new(0.0, 0.0),
            Vec2::<()>::new(1.0, 0.0),
            Vec2::<()>::new(2.0, 0.0),
            Vec2::<()>::new(3.0, 0.0),
            Vec2::<()>::new(4.0, 0.0),
        ])
        .with_step(2);
        // fifth of the run: first star edge ends at vertex 2
        assert_eq!(pentagram.value_at(Fraction::new(0.2)), Vec2::<()>::new(2.0, 0.0));
    }

    /// A regular pentagon's first vertex sits on the +x axis.
    #[test]
    fn the_regular_polygon_starts_at_full_radius() {
        let pentagon = StarPath::<Vec2<()>>::regular(Vec2::<()>::new(0.5, 0.5), 0.3, 5);
        assert_eq!(pentagon.points.len(), 5);
        let start = pentagon.value_at(Fraction::ZERO);
        assert!((start.x - 0.8).abs() < 1e-9);
        assert!((start.y - 0.5).abs() < 1e-9);
    }
}
