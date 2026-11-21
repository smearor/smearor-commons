//! A helical 3D trajectory - circular motion with a climbing `z`.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A helix: the value winds `turns` revolutions around `center`
/// while `z` climbs linearly from `z_from` to `z_to` - corkscrews,
/// spirals, ascending paths.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Helix<U = ()> {
    /// The point the value circles, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The helix radius in the unit of `center`.
    pub radius: f64,
    /// Revolutions over `0..=1` - `1.0` is a single winding.
    pub turns: f64,
    /// The `z` at progress `0`.
    pub z_from: f64,
    /// The `z` at progress `1`.
    pub z_to: f64,
}

impl<U> Helix<U> {
    /// Creates a single-winding helix around `center`, climbing
    /// from `z_from` to `z_to`.
    pub fn new(center: Vec2<U>, radius: f64, z_from: f64, z_to: f64) -> Self {
        Self {
            center,
            radius,
            turns: 1.0,
            z_from,
            z_to,
        }
    }

    /// Returns the helix with `turns` windings per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Helix<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let t = t.as_f64();
        let angle = t * self.turns * TAU;
        Vec3::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
            self.z_from + (self.z_to - self.z_from) * t,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Helix;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;
    use smearor_commons_geometry::Vec3;

    /// The endpoints pin the circle's east at `z_from`/`z_to`.
    #[test]
    fn the_endpoints_pin_the_winding() {
        let helix = Helix::new(Vec2::<()>::new(1.0, 1.0), 0.5, 0.0, 2.0);
        assert_eq!(helix.value_at(Fraction::ZERO), Vec3::<()>::new(1.5, 1.0, 0.0));
        let end = helix.value_at(Fraction::ONE);
        assert!((end.x - 1.5).abs() < 1e-9);
        assert!((end.z - 2.0).abs() < 1e-9);
    }

    /// Half a turn crosses the circle at half the `z` climb.
    #[test]
    fn half_a_turn_crosses_at_half_the_climb() {
        let helix = Helix::new(Vec2::<()>::new(0.0, 0.0), 1.0, 0.0, 2.0);
        let point = helix.value_at(Fraction::HALF);
        assert!((point.x + 1.0).abs() < 1e-9);
        assert!(point.y.abs() < 1e-9);
        assert!((point.z - 1.0).abs() < 1e-9);
    }
}
