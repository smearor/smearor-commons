//! A hypotrochoid trajectory - the spirograph curve.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A spirograph: a circle of radius `inner` rolls inside a circle
/// of radius `outer` while a pen at distance `pen` from its center
/// traces the curve - stars, blossoms and hypocycloids from three
/// numbers. Rational `inner : outer` ratios close the figure;
/// `pen == inner` pins the pen on the rolling circle (stars with
/// cusps, `outer == 4 * inner` gives the astroid).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Spiro<U = ()> {
    /// The point the fixed circle is centered on.
    pub center: Vec2<U>,
    /// The radius of the fixed circle in the unit of `center`.
    pub outer: f64,
    /// The radius of the rolling circle.
    pub inner: f64,
    /// The pen's distance from the rolling circle's center.
    pub pen: f64,
    /// The rolling multiplier over `0..=1` - the full pattern needs
    /// `outer / inner` revolutions when that ratio is an integer.
    pub turns: f64,
}

impl<U> Spiro<U> {
    /// Creates a spirograph with the classic wheel and pen.
    pub fn new(center: Vec2<U>, outer: f64, inner: f64, pen: f64) -> Self {
        Self {
            center,
            outer,
            inner,
            pen,
            turns: 1.0,
        }
    }

    /// Returns the spirograph with a `turns` rolling multiplier.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Spiro<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let gap = self.outer - self.inner;
        let ratio = gap / self.inner.max(f64::EPSILON);
        Vec2::new(
            self.center.x + gap * angle.cos() + self.pen * (ratio * angle).cos(),
            self.center.y + gap * angle.sin() - self.pen * (ratio * angle).sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Spiro;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// An integer ratio closes: the 4:1 astroid returns home.
    #[test]
    fn an_integer_wheel_ratio_closes() {
        let spiro = Spiro::new(Vec2::<()>::new(0.5, 0.5), 0.4, 0.1, 0.1).with_turns(4.0);
        let (start, end) = (spiro.value_at(Fraction::ZERO), spiro.value_at(Fraction::ONE));
        assert!((end.x - start.x).abs() < 1e-9);
        assert!((end.y - start.y).abs() < 1e-9);
    }
}
