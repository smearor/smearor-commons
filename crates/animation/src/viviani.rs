//! Viviani's curve - a figure eight on a sphere.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// Viviani's curve: the intersection of a sphere with a cylinder
/// through its center - a lemniscate draped onto a ball. One run
/// traces the full figure and closes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Viviani<U = ()> {
    /// The point the curve is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The extent of the figure in the unit of `center`.
    pub radius: f64,
    /// Laps over `0..=1` - `1.0` traces the whole curve once.
    pub turns: f64,
}

impl<U> Viviani<U> {
    /// Creates Viviani's curve of `radius` around `center`.
    pub fn new(center: Vec2<U>, radius: f64) -> Self {
        Self { center, radius, turns: 1.0 }
    }

    /// Returns the curve with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Viviani<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        // the canonical parametrization needs 4 pi for one full
        // figure - `turns` counts complete curves
        let angle = t.as_f64() * self.turns * 2.0 * TAU;
        Vec3::new(
            self.center.x + self.radius * 0.5 * angle.cos(),
            self.center.y + self.radius * 0.5 * angle.sin(),
            self.radius * (angle * 0.5).sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Viviani;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The 4 pi parametrization closes after one lap.
    #[test]
    fn viviani_closes() {
        let curve = Viviani::new(Vec2::<()>::new(0.5, 0.5), 0.4);
        let (start, end) = (curve.value_at(Fraction::ZERO), curve.value_at(Fraction::ONE));
        assert!((end.x - start.x).abs() < 1e-9);
        assert!((end.y - start.y).abs() < 1e-9);
        assert!((end.z - start.z).abs() < 1e-9);
    }
}
