//! A rhodonea trajectory - the classic rose curve.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A rose curve: `r = size * cos(petals * theta)` - odd integer
/// petal counts draw `k` leaves, even counts `2k`, rational values
/// braided rosettes. `phase` rotates the whole figure.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Rose<U = ()> {
    /// The point the rose is centered on.
    pub center: Vec2<U>,
    /// The tip radius in the unit of `center`.
    pub size: f64,
    /// The petal ratio - `3.0` is the trefoil rose.
    pub petals: f64,
    /// The rotation of the rose, in radians.
    pub phase: f64,
    /// Laps over `0..=1` - `1.0` draws the full rosette.
    pub turns: f64,
}

impl<U> Rose<U> {
    /// Creates a rose of `petals` ratio around `center`.
    pub fn new(center: Vec2<U>, size: f64, petals: f64) -> Self {
        Self {
            center,
            size,
            petals,
            phase: 0.0,
            turns: 1.0,
        }
    }

    /// Returns the rose rotated by `phase` radians.
    pub fn with_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        self
    }

    /// Returns the rose with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Rose<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU + self.phase;
        let radius = self.size * (self.petals * angle).cos();
        Vec2::new(self.center.x + radius * angle.cos(), self.center.y + radius * angle.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::Rose;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The rose starts at a petal tip and closes the rosette.
    #[test]
    fn the_rose_closes() {
        let rose = Rose::new(Vec2::<()>::new(0.5, 0.5), 0.3, 3.0);
        let (start, end) = (rose.value_at(Fraction::ZERO), rose.value_at(Fraction::ONE));
        assert!((start.x - 0.8).abs() < 1e-9);
        assert!((end.x - start.x).abs() < 1e-9);
        assert!((end.y - start.y).abs() < 1e-9);
    }
}
