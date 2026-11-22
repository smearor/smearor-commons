//! A spherical Lissajous trajectory - woven coverage of a ball.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A sphere Lissajous: longitude and latitude oscillate in a
/// `lon_turns : lat_turns` frequency ratio - the value weaves a
/// basket pattern over a sphere of `radius`. Integer ratios close
/// the weave.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SphereLissajous<U = ()> {
    /// The point the sphere is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The sphere radius in the unit of `center`.
    pub radius: f64,
    /// Longitude frequency in revolutions over `0..=1`.
    pub lon_turns: f64,
    /// Latitude frequency in revolutions over `0..=1`.
    pub lat_turns: f64,
}

impl<U> SphereLissajous<U> {
    /// Creates a weave of `lon_turns`:`lat_turns` on a sphere.
    pub fn new(center: Vec2<U>, radius: f64, lon_turns: f64, lat_turns: f64) -> Self {
        Self {
            center,
            radius,
            lon_turns,
            lat_turns,
        }
    }
}

impl<U> Trajectory for SphereLissajous<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let t = t.as_f64();
        let longitude = t * self.lon_turns * TAU;
        let latitude = t * self.lat_turns * TAU;
        Vec3::new(
            self.center.x + self.radius * latitude.cos() * longitude.cos(),
            self.center.y + self.radius * latitude.cos() * longitude.sin(),
            self.radius * latitude.sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::SphereLissajous;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// Every sample sits exactly on the sphere.
    #[test]
    fn samples_stay_on_the_sphere() {
        let weave = SphereLissajous::new(Vec2::<()>::new(0.5, 0.5), 0.4, 5.0, 3.0);
        for i in 0..=16 {
            let point = weave.value_at(Fraction::new(i as f64 / 16.0));
            let radius = ((point.x - 0.5).powi(2) + (point.y - 0.5).powi(2) + point.z.powi(2)).sqrt();
            assert!((radius - 0.4).abs() < 1e-9);
        }
    }
}
