//! A yo-yo trajectory - plays the inner trajectory forward, then
//! backward, over the same progress range.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Mirrors the `inner` trajectory: `0 -> 0.5` plays `inner`
/// forward, `0.5 -> 1` plays it backward - `A -> B -> A` on one
/// clock, the yo-yo shape.
///
/// `Mirror<Span<ScaleFactor>>` is a breathing animation;
/// `Mirror<Span<RotationRad>>` swings out and back. Chaining
/// `Mirror` in a [`crate::Repeat`] gives a continuous back-and-forth
/// without a seam.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Mirror<S> {
    /// The trajectory played forward, then backward.
    pub inner: S,
}

impl<S> Mirror<S> {
    /// Creates a mirrored trajectory.
    pub const fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S: Trajectory> Trajectory for Mirror<S> {
    type Value = S::Value;

    fn value_at(&self, t: Fraction) -> S::Value {
        let t = t.as_f64();
        self.inner.value_at(Fraction::new(if t <= 0.5 { 2.0 * t } else { 2.0 - 2.0 * t }))
    }
}

#[cfg(test)]
mod tests {
    use super::Mirror;
    use crate::Span;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// The mirror reaches the inner end at t = 0.5 and returns.
    #[test]
    fn the_value_turns_around_at_the_midpoint() {
        let mirror = Mirror::new(Span::new(0.0_f64, 10.0));
        assert_eq!(mirror.value_at(Fraction::new(0.5)), 10.0);
        assert_eq!(mirror.value_at(Fraction::new(0.25)), 5.0);
        assert_eq!(mirror.value_at(Fraction::new(0.75)), 5.0);
        assert_eq!(mirror.value_at(Fraction::ONE), 0.0);
    }
}
