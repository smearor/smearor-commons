//! An ellipse trajectory - the oval sibling of `Orbit`.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// An ellipse: `Orbit` with independent `x`/`y` radii and an
/// optional `rotation` of the whole shape - `rx == ry` and
/// `rotation == 0` is the plain circle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Ellipse<U = ()> {
    /// The point the ellipse is centered on.
    pub center: Vec2<U>,
    /// The `x` semi-axis in the unit of `center`.
    pub rx: f64,
    /// The `y` semi-axis in the unit of `center`.
    pub ry: f64,
    /// The rotation of the ellipse's axes, in radians.
    pub rotation: f64,
    /// Laps over `0..=1` - `1.0` is a single loop.
    pub turns: f64,
}

impl<U> Ellipse<U> {
    /// Creates an axis-aligned ellipse around `center`.
    pub fn new(center: Vec2<U>, rx: f64, ry: f64) -> Self {
        Self {
            center,
            rx,
            ry,
            rotation: 0.0,
            turns: 1.0,
        }
    }

    /// Returns the ellipse rotated by `rotation` radians.
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    /// Returns the ellipse with `turns` laps per run.
    pub fn with_turns(mut self, turns: f64) -> Self {
        self.turns = turns;
        self
    }
}

impl<U> Trajectory for Ellipse<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let angle = t.as_f64() * self.turns * TAU;
        let (px, py) = (self.rx * angle.cos(), self.ry * angle.sin());
        let (sin, cos) = (self.rotation.sin(), self.rotation.cos());
        Vec2::new(self.center.x + px * cos - py * sin, self.center.y + px * sin + py * cos)
    }
}

#[cfg(test)]
mod tests {
    use super::Ellipse;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;

    /// The ellipse closes and hits its semi-axes.
    #[test]
    fn the_ellipse_reaches_its_axes() {
        let ellipse = Ellipse::new(Vec2::<()>::new(0.5, 0.5), 0.3, 0.1);
        assert_eq!(ellipse.value_at(Fraction::ZERO), Vec2::<()>::new(0.8, 0.5));
        assert_eq!(ellipse.value_at(Fraction::ONE), Vec2::<()>::new(0.8, 0.5));
        let top = ellipse.value_at(Fraction::new(0.25));
        assert!((top.x - 0.5).abs() < 1e-9);
        assert!((top.y - 0.6).abs() < 1e-9);
    }
}
