//! A conical helix trajectory - `Helix` with a tapering radius.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A conical helix: the value winds `turns` times around `center`
/// while the radius tapers `radius_from -> radius_to` and `z`
/// climbs `z_from -> z_to` - a spiral staircase flaring into a
/// cone. Equal radii are the plain [`crate::Helix`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ConicalHelix<U = ()> {
    /// The point the winding is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The starting radius at progress `0`.
    pub radius_from: f64,
    /// The final radius at progress `1`.
    pub radius_to: f64,
    /// The starting depth at progress `0`.
    pub z_from: f64,
    /// The final depth at progress `1`.
    pub z_to: f64,
    /// Revolutions over `0..=1`.
    pub turns: f64,
}

impl<U> ConicalHelix<U> {
    /// Creates a conical helix around `center`.
    pub fn new(center: Vec2<U>, radius_from: f64, radius_to: f64, z_from: f64, z_to: f64) -> Self {
        Self {
            center,
            radius_from,
            radius_to,
            z_from,
            z_to,
            turns: 1.0,
        }
    }

    /// Returns the helix with `turns` windings per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for ConicalHelix<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let t = t.as_f64();
        let radius = self.radius_from + (self.radius_to - self.radius_from) * t;
        let angle = t * self.turns * TAU;
        Vec3::new(
            self.center.x + radius * angle.cos(),
            self.center.y + radius * angle.sin(),
            self.z_from + (self.z_to - self.z_from) * t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ConicalHelix;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The cone starts on the inner radius and ends on the outer.
    #[test]
    fn the_radius_tapers() {
        let helix = ConicalHelix::new(Vec2::<()>::new(0.5, 0.5), 0.1, 0.4, 0.0, 1.0);
        let start = helix.value_at(Fraction::ZERO);
        assert!((start.x - 0.6).abs() < 1e-9);
        assert!(start.z.abs() < 1e-9);
        let end = helix.value_at(Fraction::ONE);
        assert!((end.x - 0.9).abs() < 1e-9);
        assert!((end.z - 1.0).abs() < 1e-9);
    }
}
