//! A Lorenz attractor trajectory - the strange attractor baked
//! into a path.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;

/// A Lorenz attractor: `new` integrates the classic system
/// (sigma = 10, rho = 28, beta = 8/3) with RK4 and stores the
/// normalized path - `value_at` then walks the baked polyline.
/// The attractor does not close; pair with `Mirror` or a
/// `Repeat`-friendly easing for seamless loops.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Lorenz<U = ()> {
    /// The baked path, normalized around the center.
    pub points: Vec<Vec3<U>>,
}

/// One RK4 step of the Lorenz system.
fn rk4(state: (f64, f64, f64), dt: f64) -> (f64, f64, f64) {
    const SIGMA: f64 = 10.0;
    const RHO: f64 = 28.0;
    const BETA: f64 = 8.0 / 3.0;
    let f = |(x, y, z): (f64, f64, f64)| (SIGMA * (y - x), x * (RHO - z) - y, x * y - BETA * z);
    let k1 = f(state);
    let k2 = f((state.0 + dt * k1.0 / 2.0, state.1 + dt * k1.1 / 2.0, state.2 + dt * k1.2 / 2.0));
    let k3 = f((state.0 + dt * k2.0 / 2.0, state.1 + dt * k2.1 / 2.0, state.2 + dt * k2.2 / 2.0));
    let k4 = f((state.0 + dt * k3.0, state.1 + dt * k3.1, state.2 + dt * k3.2));
    (
        state.0 + dt * (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0,
        state.1 + dt * (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0,
        state.2 + dt * (k1.2 + 2.0 * k2.2 + 2.0 * k3.2 + k4.2) / 6.0,
    )
}

impl<U> Lorenz<U> {
    /// Integrates the attractor and bakes `points` scaled to
    /// `size` around `center` (in `x`/`y`, `z` centered on `0`).
    pub fn new(center: Vec2<U>, size: f64) -> Self {
        const DT: f64 = 0.005;
        const WARMUP: usize = 800;
        const SAMPLES: usize = 512;
        const STRIDE: usize = 12;
        let mut state = (0.1_f64, 0.0, 0.0);
        for _ in 0..WARMUP {
            state = rk4(state, DT);
        }
        let mut raw = Vec::with_capacity(SAMPLES);
        for _ in 0..SAMPLES {
            for _ in 0..STRIDE {
                state = rk4(state, DT);
            }
            raw.push(state);
        }
        // uniform scale around the bounding-box center keeps the
        // butterfly's proportions
        let (mut lo, mut hi) = (raw[0], raw[0]);
        for point in &raw {
            lo = (lo.0.min(point.0), lo.1.min(point.1), lo.2.min(point.2));
            hi = (hi.0.max(point.0), hi.1.max(point.1), hi.2.max(point.2));
        }
        let middle = ((lo.0 + hi.0) / 2.0, (lo.1 + hi.1) / 2.0, (lo.2 + hi.2) / 2.0);
        let extent = ((hi.0 - lo.0) / 2.0).max((hi.1 - lo.1) / 2.0).max((hi.2 - lo.2) / 2.0).max(f64::EPSILON);
        let scale = size / extent;
        let points = raw
            .into_iter()
            .map(|(x, y, z)| Vec3::new(center.x + (x - middle.0) * scale, center.y + (y - middle.1) * scale, (z - middle.2) * scale))
            .collect();
        Self { points }
    }
}

impl<U> Trajectory for Lorenz<U> {
    type Value = Vec3<U>;

    fn value_at(&self, t: Fraction) -> Vec3<U> {
        let last = (self.points.len() - 1).max(1);
        let position = t.as_f64().clamp(0.0, 1.0) * last as f64;
        let index = (position as usize).min(last - 1);
        self.points[index].interpolate(self.points[index + 1], Fraction::new(position - index as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::Lorenz;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The baked path fills its declared size and never runs far
    /// off - the attractor stays bounded.
    #[test]
    fn the_attractor_fills_its_bounds() {
        let lorenz = Lorenz::new(Vec2::<()>::new(0.5, 0.5), 0.4);
        assert_eq!(lorenz.points.len(), 512);
        for i in 0..=16 {
            let point = lorenz.value_at(Fraction::new(i as f64 / 16.0));
            assert!((point.x - 0.5).abs() <= 0.4 + 1e-9);
            assert!((point.y - 0.5).abs() <= 0.4 + 1e-9);
            assert!(point.z.abs() <= 0.4 + 1e-9);
        }
    }
}
