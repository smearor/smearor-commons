//! A spiral trajectory - `Orbit` with a radius growing over time.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// An Archimedean spiral: the value circles `center` `turns` times
/// while its distance grows linearly from `0` to `radius` -
/// winding in or out depending on the direction of travel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Spiral<U = ()> {
    /// The point the spiral winds out of.
    pub center: Vec2<U>,
    /// The final radius at progress `1`, in the unit of `center`.
    pub radius: f64,
    /// Revolutions over `0..=1` - `1.0` is a single winding.
    pub turns: f64,
}

impl<U> Spiral<U> {
    /// Creates a single-winding spiral from `center` to `radius`.
    pub fn new(center: Vec2<U>, radius: f64) -> Self {
        Self { center, radius, turns: 1.0 }
    }

    /// Returns the spiral with `turns` windings per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Spiral<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let t = t.as_f64();
        let radius = self.radius * t;
        let angle = t * self.turns * TAU;
        Vec2::new(self.center.x + radius * angle.cos(), self.center.y + radius * angle.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::Spiral;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The spiral starts at the center and ends a full radius out.
    #[test]
    fn the_spiral_grows_from_the_center() {
        let spiral = Spiral::new(Vec2::<()>::new(0.5, 0.5), 0.4);
        assert_eq!(spiral.value_at(Fraction::ZERO), Vec2::<()>::new(0.5, 0.5));
        let end = spiral.value_at(Fraction::ONE);
        assert!((end.x - 0.9).abs() < 1e-9);
        assert!((end.y - 0.5).abs() < 1e-9);
    }
}
