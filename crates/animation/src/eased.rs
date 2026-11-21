//! Easing as a trajectory decorator - shapes progress before the
//! inner trajectory sees it.

use crate::EasingFunction;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Applies `easing` to the progress before delegating to `inner`:
/// `value_at(t) = inner.value_at(easing.apply(t))`.
///
/// Lets each part of a composite trajectory keep its own curve -
/// `(Eased<Span>, Eased<Span>)` eases a composite's members
/// differently while `TimedAnimation`'s outer easing stays shared.
/// Endpoints are unaffected - easing snaps `0`/`1` through.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Eased<S> {
    /// The easing applied before the inner trajectory.
    pub easing: EasingFunction,
    /// The wrapped trajectory.
    pub inner: S,
}

impl<S> Eased<S> {
    /// Wraps `inner` so progress passes `easing` first.
    pub fn new(inner: S, easing: EasingFunction) -> Self {
        Self { easing, inner }
    }
}

impl<S: Trajectory> Trajectory for Eased<S> {
    type Value = S::Value;

    fn value_at(&self, t: Fraction) -> S::Value {
        self.inner.value_at(self.easing.apply(t))
    }
}

#[cfg(test)]
mod tests {
    use super::Eased;
    use crate::EasingFunction;
    use crate::Span;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// The easing shapes the inner progress, endpoints pass through.
    #[test]
    fn the_easing_shapes_the_inner_progress() {
        let eased = Eased::new(Span::new(0.0_f64, 1.0), EasingFunction::EaseIn);
        assert_eq!(eased.value_at(Fraction::ZERO), 0.0);
        assert_eq!(eased.value_at(Fraction::HALF), 0.25); // t^2 at .5
        assert_eq!(eased.value_at(Fraction::ONE), 1.0);
    }
}
