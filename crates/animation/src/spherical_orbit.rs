//! An orbit on a tilted plane - a circular path in 3D.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A spherical orbit: `Orbit` tilted out of the `x`/`y` plane by
/// `inclination` radians - ring systems, gyroscope paths. `0` is a
/// flat orbit, `FRAC_PI_2` a fully vertical one.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SphericalOrbit<U = ()> {
    /// The point the orbit circles, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The orbit radius in the unit of `center`.
    pub radius: f64,
    /// Revolutions over `0..=1` - `1.0` is a single lap.
    pub turns: f64,
    /// The orbit plane's tilt around the `x` axis, in radians.
    pub inclination: f64,
}

impl<U> SphericalOrbit<U> {
    /// Creates a single-lap tilted orbit around `center`.
    pub fn new(center: Vec2<U>, radius: f64, inclination: f64) -> Self {
        Self {
            center,
            radius,
            turns: 1.0,
            inclination,
        }
    }

    /// Returns the orbit with `turns` revolutions per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for SphericalOrbit<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let (sin, cos) = (angle.sin(), angle.cos());
        Vec3::new(
            self.center.x + self.radius * cos,
            self.center.y + self.radius * sin * self.inclination.cos(),
            self.radius * sin * self.inclination.sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::SphericalOrbit;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;
    use std::f64::consts::FRAC_PI_2;

    /// A flat orbit stays in the plane like `Orbit`.
    #[test]
    fn zero_inclination_is_the_flat_orbit() {
        let orbit = SphericalOrbit::new(Vec2::<()>::new(0.5, 0.5), 0.3, 0.0);
        let point = orbit.value_at(Fraction::new(0.25));
        assert!((point.x - 0.5).abs() < 1e-9);
        assert!((point.y - 0.8).abs() < 1e-9);
        assert!(point.z.abs() < 1e-9);
    }

    /// Full tilt swings through `z` like a vertical wheel.
    #[test]
    fn full_tilt_swings_through_depth() {
        let orbit = SphericalOrbit::new(Vec2::<()>::new(0.0, 0.0), 1.0, FRAC_PI_2);
        let point = orbit.value_at(Fraction::new(0.25));
        assert!((point.z - 1.0).abs() < 1e-9);
        assert!(point.y.abs() < 1e-9);
    }
}
