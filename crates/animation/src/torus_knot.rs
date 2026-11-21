//! A (p,q) torus knot trajectory - the value winds around a torus.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::f64::consts::TAU;

/// A torus knot: the value winds `p` times around the torus axis
/// while threading `q` times through the hole - prime pairs draw
/// the classic interlaced knots. `radius` is the overall extent;
/// the knot is normalized to fit it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TorusKnot<U = ()> {
    /// The point the knot is centered on, in the `x`/`y` plane.
    pub center: Vec2<U>,
    /// The overall extent of the knot in the unit of `center`.
    pub radius: f64,
    /// Winds around the torus axis per run.
    pub p: u32,
    /// Threads through the torus hole per run.
    pub q: u32,
}

impl<U> TorusKnot<U> {
    /// Creates a `(p, q)` torus knot around `center`.
    pub fn new(center: Vec2<U>, radius: f64, p: u32, q: u32) -> Self {
        Self { center, radius, p, q }
    }
}

impl<U> Trajectory for TorusKnot<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let angle = t.as_f64() * TAU;
        // normalized (2 + cos q) keeps the knot inside `radius`
        let scale = self.radius / 3.0;
        let ring = 2.0 + (angle * self.q as f64).cos();
        Vec3::new(
            self.center.x + scale * ring * (angle * self.p as f64).cos(),
            self.center.y + scale * ring * (angle * self.p as f64).sin(),
            scale * (angle * self.q as f64).sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TorusKnot;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The knot closes: integer windings return to the start.
    #[test]
    fn integer_windings_close_the_knot() {
        let knot = TorusKnot::new(Vec2::<()>::new(0.5, 0.5), 0.3, 2, 3);
        let (start, end) = (knot.value_at(Fraction::ZERO), knot.value_at(Fraction::ONE));
        assert!((end.x - start.x).abs() < 1e-9);
        assert!((end.y - start.y).abs() < 1e-9);
        assert!((end.z - start.z).abs() < 1e-9);
    }

    /// The knot stays inside its declared extent.
    #[test]
    fn the_knot_stays_within_radius() {
        let knot = TorusKnot::new(Vec2::<()>::new(0.0, 0.0), 1.0, 3, 4);
        for i in 0..=32 {
            let point = knot.value_at(Fraction::new(i as f64 / 32.0));
            assert!(point.x.abs() <= 1.0 + 1e-9);
            assert!(point.y.abs() <= 1.0 + 1e-9);
            assert!(point.z.abs() <= 1.0 + 1e-9);
        }
    }
}
