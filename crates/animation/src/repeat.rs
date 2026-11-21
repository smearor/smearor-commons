//! A repeating trajectory - wraps an inner trajectory and plays it
//! `count` times within the animation duration.

use crate::EasingFunction;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Repeats the `inner` trajectory `count` times over the progress
/// range - the blinking/pulsing/loading-indicator shape.
///
/// `Repeat::new(inner, 3)` plays `inner` three times; the eased
/// progress of the outer animation is folded into `0..1` per
/// repetition, so the easing applies to the whole cycle sequence,
/// not per cycle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Repeat<S> {
    /// The trajectory played each cycle.
    pub inner: S,
    /// How often `inner` plays - at least once.
    pub count: u32,
    /// Easing applied to the progress inside each cycle - kept
    /// per-cycle so the outer easing only warps the cycle
    /// sequence, not the cycle shape itself.
    #[serde(default)]
    pub cycle_easing: EasingFunction,
}

impl<S> Repeat<S> {
    /// Creates a trajectory repeating `inner` `count` times.
    pub fn new(inner: S, count: u32) -> Self {
        Self {
            inner,
            count,
            cycle_easing: EasingFunction::Linear,
        }
    }

    /// Returns the repeat with an easing curve inside each cycle.
    pub fn with_easing(mut self, cycle_easing: EasingFunction) -> Self {
        self.cycle_easing = cycle_easing;
        self
    }
}

impl<S: Trajectory> Trajectory for Repeat<S> {
    type Value = S::Value;

    fn value_at(&self, t: Fraction) -> S::Value {
        let count = self.count.max(1) as f64;
        let cycle = t.as_f64() * count;
        // The last point of the last cycle is the trajectory end -
        // `fract` alone would wrap it back to the start.
        let t = if cycle >= count { 1.0 } else { cycle.fract() };
        self.inner.value_at(self.cycle_easing.apply(Fraction::new(t)))
    }
}

#[cfg(test)]
mod tests {
    use super::Repeat;
    use crate::Span;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// Two repetitions pass the inner midpoint twice.
    #[test]
    fn the_inner_cycle_repeats() {
        let repeat = Repeat::new(Span::new(0.0_f64, 10.0), 2);
        assert_eq!(repeat.value_at(Fraction::new(0.25)), 5.0);
        assert_eq!(repeat.value_at(Fraction::new(0.75)), 5.0);
        assert_eq!(repeat.value_at(Fraction::ONE), 10.0);
    }

    /// Cycle boundaries wrap back to the inner start.
    #[test]
    fn cycle_boundaries_wrap_to_the_start() {
        let repeat = Repeat::new(Span::new(0.0_f64, 10.0), 2);
        assert_eq!(repeat.value_at(Fraction::new(0.5)), 0.0);
    }

    /// `with_easing` shapes each cycle's inside while the cycle
    /// count and boundaries stay fixed.
    #[test]
    fn cycle_easing_shapes_each_cycle() {
        let repeat = Repeat::new(Span::new(0.0_f64, 10.0), 2).with_easing(crate::EasingFunction::EaseIn);
        // cycle midpoint t=0.25 -> cycle progress 0.5 eased to 0.25
        assert_eq!(repeat.value_at(Fraction::new(0.25)), 2.5);
        // boundaries are untouched - cycles still end on `to`
        assert_eq!(repeat.value_at(Fraction::new(0.5)), 0.0);
        assert_eq!(repeat.value_at(Fraction::ONE), 10.0);
    }

    /// Older payloads without the easing field still deserialize.
    #[test]
    fn missing_cycle_easing_defaults_to_linear() {
        let json = r#"{"inner":{"from":0.0,"to":10.0},"count":2}"#;
        let repeat: Repeat<Span<f64>> = serde_json::from_str(json).expect("valid repeat");
        assert_eq!(repeat.cycle_easing, crate::EasingFunction::Linear);
    }
}
