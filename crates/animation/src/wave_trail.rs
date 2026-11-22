//! A wave trail trajectory - travel from A to B wobbling
//! perpendicular.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// A wave trail: the value moves `from -> to` while oscillating
/// `amplitude` to either side of the route `waves` times - a
/// slalom, a wobbling fly-by.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct WaveTrail<U = ()> {
    /// The departure point.
    pub from: Vec2<U>,
    /// The arrival point.
    pub to: Vec2<U>,
    /// The sideways swing in the unit of `from`/`to`.
    pub amplitude: f64,
    /// The number of full wiggles over `0..=1`.
    pub waves: f64,
}

impl<U> WaveTrail<U> {
    /// Creates a wave trail from `from` to `to`.
    pub fn new(from: Vec2<U>, to: Vec2<U>, amplitude: f64, waves: f64) -> Self {
        Self { from, to, amplitude, waves }
    }
}

impl<U> Trajectory for WaveTrail<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let position = self.from.interpolate(self.to, t);
        let direction = self.to - self.from;
        let length = (direction.x * direction.x + direction.y * direction.y).sqrt().max(f64::EPSILON);
        let sway = self.amplitude * (t.as_f64() * self.waves * std::f64::consts::TAU).sin();
        Vec2::new(position.x - direction.y / length * sway, position.y + direction.x / length * sway)
    }
}

#[cfg(test)]
mod tests {
    use super::WaveTrail;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// Integer waves pin the endpoints exactly on the route.
    #[test]
    fn the_trail_anchors_at_its_ends() {
        let trail = WaveTrail::new(Vec2::<()>::new(0.0, 0.5), Vec2::<()>::new(1.0, 0.5), 0.2, 3.0);
        assert_eq!(trail.value_at(Fraction::ZERO), Vec2::<()>::new(0.0, 0.5));
        let end = trail.value_at(Fraction::ONE);
        assert!((end.x - 1.0).abs() < 1e-9);
        assert!((end.y - 0.5).abs() < 1e-9);
    }
}
