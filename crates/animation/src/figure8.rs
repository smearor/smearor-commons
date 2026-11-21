//! A figure eight in space - a lemniscate in the `x`/`y` plane
//! with a depth swing.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A spatial figure eight: the value traces a Gerono lemniscate
/// in the `x`/`y` plane while `z` swings with `depth` - a sideways
/// infinity loop that also dives forward.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Figure8<U = ()> {
    /// The point the eight is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The half width of the eight in the unit of `center`.
    pub size: f64,
    /// Laps over `0..=1` - `1.0` is a single eight.
    pub turns: f64,
    /// The `z` swing amplitude - `0` keeps the eight flat.
    pub depth: f64,
}

impl<U> Figure8<U> {
    /// Creates a flat figure eight around `center`.
    pub fn new(center: Vec2<U>, size: f64, depth: f64) -> Self {
        Self {
            center,
            size,
            turns: 1.0,
            depth,
        }
    }

    /// Returns the eight with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Figure8<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let (sin, cos) = (angle.sin(), angle.cos());
        Vec3::new(self.center.x + self.size * cos, self.center.y + self.size * sin * cos, self.depth * sin)
    }
}

#[cfg(test)]
mod tests {
    use super::Figure8;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The eight closes and crosses its center at the quarter lap.
    #[test]
    fn the_eight_crosses_its_center() {
        let eight = Figure8::new(Vec2::<()>::new(0.5, 0.5), 0.3, 0.0);
        let point = eight.value_at(Fraction::new(0.25));
        assert!((point.x - 0.5).abs() < 1e-9);
        assert!((point.y - 0.5).abs() < 1e-9);
        let end = eight.value_at(Fraction::ONE);
        assert!((end.x - 0.8).abs() < 1e-9);
    }
}
