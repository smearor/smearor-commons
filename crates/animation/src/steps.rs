//! A discrete trajectory - holds each of `values` for an equal
//! share of the progress, no interpolation.

use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// A stepwise trajectory: `values` played in order, each held for
/// `1 / n` of the progress - typewriter reveals, sprite-sheet frame
/// animation, anything that must *not* interpolate.
///
/// The first value is a separate constructor argument, so a
/// `Steps` can never be empty. For a stepped *easing* (progress
/// quantized before any trajectory sees it) use
/// [`crate::EasingFunction::Steps`] instead.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Steps<T> {
    /// The values held in order - never empty.
    pub values: Vec<T>,
}

impl<T> Steps<T> {
    /// Creates a stepped trajectory from its `first` value and the
    /// `rest`.
    pub fn new(first: T, rest: impl IntoIterator<Item = T>) -> Self {
        let mut values: Vec<T> = rest.into_iter().collect();
        values.insert(0, first);
        Self { values }
    }
}

impl<T: Copy> Trajectory for Steps<T> {
    type Value = T;

    fn value_at(&self, t: Fraction) -> T {
        let n = self.values.len() as f64;
        let index = (t.as_f64() * n).floor().clamp(0.0, n - 1.0) as usize;
        self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Steps;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// Each value holds for an equal share of the progress.
    #[test]
    fn each_step_holds_its_share() {
        let steps = Steps::new('a', ['b', 'c']);
        assert_eq!(steps.value_at(Fraction::new(0.2)), 'a');
        assert_eq!(steps.value_at(Fraction::new(0.5)), 'b');
        assert_eq!(steps.value_at(Fraction::new(0.9)), 'c');
        assert_eq!(steps.value_at(Fraction::ONE), 'c');
    }
}
