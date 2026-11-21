//! A delayed trajectory - holds the inner start value, then plays
//! the trajectory compressed into the remaining progress.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Holds `inner`'s start value until progress `delay`, then plays
/// `inner` remapped into the rest of the range - the choreography
/// primitive for tuple composites: `(Span<RotationRad>,
/// Delay<Span<ScaleFactor>>)` rotates first, then zooms, on one
/// clock.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Delay<S> {
    /// The trajectory played after the delay.
    pub inner: S,
    /// The progress fraction held before `inner` starts
    /// (`0.0..=1.0`).
    pub delay: Fraction,
}

impl<S> Delay<S> {
    /// Creates a trajectory holding `inner`'s start until `delay`.
    pub const fn new(inner: S, delay: Fraction) -> Self {
        Self { inner, delay }
    }
}

impl<S: Trajectory> Trajectory for Delay<S> {
    type Value = S::Value;

    fn value_at(&self, t: Fraction) -> S::Value {
        let (t, delay) = (t.as_f64(), self.delay.as_f64().clamp(0.0, 1.0));
        if t <= delay {
            return self.inner.value_at(Fraction::ZERO);
        }
        let span = 1.0 - delay;
        if span <= 0.0 {
            return self.inner.value_at(Fraction::ONE);
        }
        self.inner.value_at(Fraction::new((t - delay) / span))
    }
}

#[cfg(test)]
mod tests {
    use super::Delay;
    use crate::Span;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// Before the delay the start value holds.
    #[test]
    fn the_start_value_holds_until_the_delay() {
        let delay = Delay::new(Span::new(0.0_f64, 10.0), Fraction::new(0.5));
        assert_eq!(delay.value_at(Fraction::ZERO), 0.0);
        assert_eq!(delay.value_at(Fraction::new(0.4)), 0.0);
        assert_eq!(delay.value_at(Fraction::new(0.75)), 5.0);
        assert_eq!(delay.value_at(Fraction::ONE), 10.0);
    }
}
