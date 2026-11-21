//! A circular 2D trajectory - the value orbits a center point.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A circular orbit: the value travels `turns` revolutions around
/// `center` at distance `radius` over the progress range - orbits,
/// spinners, loading wheels.
///
/// The lap is continuous: an integer `turns` closes the curve, so
/// `value_at(0) == value_at(1)`. For an out-and-back swing combine
/// with [`crate::Mirror`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Orbit<U = ()> {
    /// The point the value circles.
    pub center: Vec2<U>,
    /// The orbit radius in the unit of `center`.
    pub radius: f64,
    /// Revolutions over `0..=1` - `1.0` is a single lap.
    pub turns: f64,
}

impl<U> Orbit<U> {
    /// Creates a single-lap orbit around `center` at `radius`.
    pub fn new(center: Vec2<U>, radius: f64) -> Self {
        Self { center, radius, turns: 1.0 }
    }

    /// Returns the orbit with `turns` revolutions per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Orbit<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU;
        Vec2::new(self.center.x + self.radius * angle.cos(), self.center.y + self.radius * angle.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::Orbit;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// A single lap starts at the center's east and returns to it.
    #[test]
    fn a_single_lap_closes_the_circle() {
        let orbit = Orbit::new(Vec2::<()>::new(1.0, 1.0), 0.5);
        assert_eq!(orbit.value_at(Fraction::ZERO), Vec2::<()>::new(1.5, 1.0));
        let end = orbit.value_at(Fraction::ONE);
        assert!((end.x - 1.5).abs() < 1e-9);
        assert!((end.y - 1.0).abs() < 1e-9);
    }

    /// A quarter turn reaches the south point.
    #[test]
    fn the_quarter_turn_hits_the_cardinal_point() {
        let orbit = Orbit::new(Vec2::<()>::new(0.0, 0.0), 1.0);
        let point = orbit.value_at(Fraction::new(0.25));
        assert!(point.x.abs() < 1e-9);
        assert!((point.y - 1.0).abs() < 1e-9);
    }
}
