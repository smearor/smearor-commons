//! A Moebius edge trajectory - the single edge of the band.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// The edge of a Moebius strip: after half a run the value sits
/// on the "other side" of the band - the edge is one closed curve
/// that takes two ring revolutions to complete.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Mobius<U = ()> {
    /// The point the ring is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The ring radius in the unit of `center`.
    pub radius: f64,
    /// The strip's half width - the `z` swing amplitude.
    pub width: f64,
}

impl<U> Mobius<U> {
    /// Creates a Moebius edge of `radius` and `width`.
    pub fn new(center: Vec2<U>, radius: f64, width: f64) -> Self {
        Self { center, radius, width }
    }
}

impl<U> Trajectory for Mobius<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        // the edge closes over 4 pi - one run is a full traversal
        let angle = t.as_f64() * 2.0 * TAU;
        let ring = self.radius + self.width * (angle * 0.5).cos();
        Vec3::new(self.center.x + ring * angle.cos(), self.center.y + ring * angle.sin(), self.width * (angle * 0.5).sin())
    }
}

#[cfg(test)]
mod tests {
    use super::Mobius;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// Half the run flips to the far side of the ring point.
    #[test]
    fn the_edge_flips_sides() {
        let edge = Mobius::new(Vec2::<()>::new(0.0, 0.0), 1.0, 0.2);
        let (start, end) = (edge.value_at(Fraction::ZERO), edge.value_at(Fraction::ONE));
        assert!((end.x - start.x).abs() < 1e-9);
        let mid = edge.value_at(Fraction::HALF);
        assert!((mid.x - (1.0 - 0.2)).abs() < 1e-9); // inner side of the band
    }
}
