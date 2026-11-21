//! A Lissajous trajectory - two sinusoidal axes in frequency ratio.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// A Lissajous figure: `x` and `y` oscillate sinusoidally around
/// `center` with frequencies `x_turns`/`y_turns` - figure eights,
/// bows and braided curves from the ratio alone.
///
/// Integer ratios close the curve, so `value_at(0) == value_at(1)`
/// - seamless loops under `EasingFunction::Linear`. `phase` shifts
///   the `x` wave and changes the figure's handedness.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Lissajous<U = ()> {
    /// The point the figure is centered on.
    pub center: Vec2<U>,
    /// The half extent of the figure on both axes.
    pub size: f64,
    /// The `x` frequency in revolutions over `0..=1`.
    pub x_turns: f64,
    /// The `y` frequency in revolutions over `0..=1`.
    pub y_turns: f64,
    /// The phase offset of the `x` wave, in radians.
    pub phase: f64,
}

impl<U> Lissajous<U> {
    /// Creates a Lissajous figure of `x_turns`:`y_turns` ratio.
    pub fn new(center: Vec2<U>, size: f64, x_turns: f64, y_turns: f64) -> Self {
        Self {
            center,
            size,
            x_turns,
            y_turns,
            phase: 0.0,
        }
    }

    /// Returns the figure with the `x` wave shifted by `phase` radians.
    pub fn with_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        self
    }
}

impl<U> Trajectory for Lissajous<U> {
    type Value = Vec2<U>;

    fn value_at(&self, t: Fraction) -> Vec2<U> {
        let t = t.as_f64();
        Vec2::new(
            self.center.x + self.size * (t * self.x_turns * TAU + self.phase).sin(),
            self.center.y + self.size * (t * self.y_turns * TAU).sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Lissajous;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Vec2;
    use std::f64::consts::FRAC_PI_2;

    /// A 2:1 figure eight at zero phase starts and ends centered.
    #[test]
    fn the_eight_is_centered_at_its_endpoints() {
        let curve = Lissajous::new(Vec2::<()>::new(0.5, 0.5), 0.3, 2.0, 1.0);
        for t in [Fraction::ZERO, Fraction::ONE] {
            let point = curve.value_at(t);
            assert!((point.x - 0.5).abs() < 1e-9);
            assert!((point.y - 0.5).abs() < 1e-9);
        }
    }

    /// A quarter phase turns a circle into a diagonal line.
    #[test]
    fn the_phase_bends_the_figure() {
        let circle = Lissajous::new(Vec2::<()>::new(0.0, 0.0), 1.0, 1.0, 1.0).with_phase(FRAC_PI_2);
        let point = circle.value_at(Fraction::ZERO);
        assert!((point.x - 1.0).abs() < 1e-9);
        assert!(point.y.abs() < 1e-9);
    }
}
