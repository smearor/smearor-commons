//! A Bernoulli lemniscate trajectory - the classic figure eight.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A Bernoulli lemniscate: the value traces `x = cos / (1 + sin²)`,
/// `y = sin·cos / (1 + sin²)` - the canonical infinity shape,
/// smoother through the crossing than the Gerono `Figure8`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Lemniskate<U = ()> {
    /// The point the lemniscate is centered on.
    pub center: Vec2<U>,
    /// The half width of the figure in the unit of `center`.
    pub size: f64,
    /// Laps over `0..=1` - `1.0` is a single eight.
    pub turns: f64,
}

impl<U> Lemniskate<U> {
    /// Creates a single-lap lemniscate around `center`.
    pub fn new(center: Vec2<U>, size: f64) -> Self {
        Self { center, size, turns: 1.0 }
    }

    /// Returns the lemniscate with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Lemniskate<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let (sin, cos) = (angle.sin(), angle.cos());
        let denominator = 1.0 + sin * sin;
        Vec2::new(self.center.x + self.size * cos / denominator, self.center.y + self.size * sin * cos / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::Lemniskate;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The lemniscate closes and crosses its center mid-figure.
    #[test]
    fn the_lemniscate_crosses_its_center() {
        let eight = Lemniskate::new(Vec2::<()>::new(0.5, 0.5), 0.3);
        let point = eight.value_at(Fraction::new(0.25));
        assert!((point.x - 0.5).abs() < 1e-9);
        assert!((point.y - 0.5).abs() < 1e-9);
        let end = eight.value_at(Fraction::ONE);
        assert!((end.x - 0.8).abs() < 1e-9);
    }
}
