//! A bounce path trajectory - parabolic hops that decay.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::PI;

/// A bouncing travel: the value moves `from -> to` while hopping
/// `hops` times, each hop reaching `height * decay^t` - a ball
/// bouncing itself out.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct BouncePath<U = ()> {
    /// The departure point.
    pub from: Vec2<U>,
    /// The arrival point.
    pub to: Vec2<U>,
    /// The apex of the first hop in the unit of `from`/`to`.
    pub height: f64,
    /// The number of hops over `0..=1`.
    pub hops: f64,
    /// The fraction of height left at the end - `0.2` fades the
    /// last hop to a fifth of the first.
    pub decay: f64,
}

impl<U> BouncePath<U> {
    /// Creates a bounce path from `from` to `to`.
    pub fn new(from: Vec2<U>, to: Vec2<U>, height: f64, hops: f64, decay: f64) -> Self {
        Self { from, to, height, hops, decay }
    }
}

impl<U> Trajectory for BouncePath<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let t = t.as_f64();
        let hop = self.height * self.decay.max(f64::EPSILON).powf(t) * (t * self.hops * PI).sin().abs();
        Vec2::new(self.from.x + (self.to.x - self.from.x) * t, self.from.y + (self.to.y - self.from.y) * t + hop)
    }
}

#[cfg(test)]
mod tests {
    use super::BouncePath;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// Integer hops touch down at both ends.
    #[test]
    fn the_hops_touch_down_at_the_ends() {
        let path = BouncePath::new(Vec2::<()>::new(0.0, 0.2), Vec2::<()>::new(1.0, 0.2), 0.3, 4.0, 0.5);
        assert_eq!(path.value_at(Fraction::ZERO), Vec2::<()>::new(0.0, 0.2));
        let end = path.value_at(Fraction::ONE);
        assert!((end.x - 1.0).abs() < 1e-9);
        assert!((end.y - 0.2).abs() < 1e-9);
    }
}
