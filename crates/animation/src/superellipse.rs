//! A superellipse trajectory - one exponent morphs circle,
//! astroid and squircle.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A superellipse: `x = rx * sgn(cos)|cos|^(2/n)`, `y = ry *
/// sgn(sin)|sin|^(2/n)`. The `exponent` `n` morphs the shape -
/// `2` is the ellipse, `2/3` the four-cusped astroid, large values
/// approach a rounded square.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Superellipse<U = ()> {
    /// The point the curve is centered on.
    pub center: Vec2<U>,
    /// The `x` semi-axis in the unit of `center`.
    pub rx: f64,
    /// The `y` semi-axis in the unit of `center`.
    pub ry: f64,
    /// The shape exponent - `2.0` is the ellipse, must be positive.
    pub exponent: f64,
    /// Laps over `0..=1` - `1.0` is a single loop.
    pub turns: f64,
}

impl<U> Superellipse<U> {
    /// Creates a superellipse of exponent `exponent`.
    pub fn new(center: Vec2<U>, rx: f64, ry: f64, exponent: f64) -> Self {
        Self {
            center,
            rx,
            ry,
            exponent,
            turns: 1.0,
        }
    }

    /// Returns the curve with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Superellipse<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let power = 2.0 / self.exponent.max(f64::EPSILON);
        let signed = |v: f64| v.signum() * v.abs().powf(power);
        Vec2::new(self.center.x + self.rx * signed(angle.cos()), self.center.y + self.ry * signed(angle.sin()))
    }
}

#[cfg(test)]
mod tests {
    use super::Superellipse;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// Exponent 2 is the plain ellipse - closed, axes aligned.
    #[test]
    fn exponent_two_is_the_ellipse() {
        let curve = Superellipse::new(Vec2::<()>::new(0.5, 0.5), 0.3, 0.2, 2.0);
        assert_eq!(curve.value_at(Fraction::ZERO), Vec2::<()>::new(0.8, 0.5));
        let quarter = curve.value_at(Fraction::new(0.25));
        assert!((quarter.x - 0.5).abs() < 1e-9);
        assert!((quarter.y - 0.7).abs() < 1e-9);
    }
}
