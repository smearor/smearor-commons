//! A piecewise-linear trajectory through named progress points -
//! the general case that [`crate::Span`] (two keys) and
//! [`crate::ThreePhaseSpan`] (four keys) specialize.

use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// A trajectory through `keys` - `(progress, value)` pairs sorted
/// ascending by progress. `value_at` lerps between the surrounding
/// keys; before the first and after the last key the boundary
/// value holds.
///
/// The first key is a separate constructor argument, so a
/// `Keyframes` can never be empty. `Keyframes::new((0.0, a), [(0.5,
/// b), (1.0, c)])` is the generalization every other shape is a
/// special case of: a `Span` is two keys, a `ThreePhaseSpan` is
/// four `(0, lead, trail, 1)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Keyframes<T> {
    /// `(progress, value)` pairs - sorted ascending on
    /// construction, never empty.
    pub keys: Vec<(Fraction, T)>,
}

impl<T> Keyframes<T> {
    /// Creates a keyframe trajectory from its `first` key and the
    /// `rest`. Keys are sorted ascending by progress.
    pub fn new(first: (Fraction, T), rest: impl IntoIterator<Item = (Fraction, T)>) -> Self {
        let mut keys: Vec<(Fraction, T)> = rest.into_iter().collect();
        keys.push(first);
        keys.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Self { keys }
    }
}

impl<T: Interpolate + Copy> Trajectory for Keyframes<T> {
    type Value = T;

    fn value_at(&self, t: Fraction) -> T {
        // The constructor guarantees a first key - index 0 is the
        // boundary fallback before and after the key range.
        let first = self.keys[0].1;
        for window in self.keys.windows(2) {
            let [(t0, v0), (t1, v1)] = window else { continue };
            if t >= *t0 && t <= *t1 {
                let span = t1.as_f64() - t0.as_f64();
                if span <= 0.0 {
                    return *v1;
                }
                return v0.interpolate(*v1, Fraction::new((t.as_f64() - t0.as_f64()) / span));
            }
        }
        let last = self.keys[self.keys.len() - 1].1;
        if t <= self.keys[0].0 { first } else { last }
    }
}

#[cfg(test)]
mod tests {
    use super::Keyframes;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// Outside the key range the boundary values hold.
    #[test]
    fn boundary_values_hold_outside_the_keys() {
        let keys = Keyframes::new((Fraction::new(0.25), 10.0_f64), [(Fraction::new(0.75), 20.0)]);
        assert_eq!(keys.value_at(Fraction::ZERO), 10.0);
        assert_eq!(keys.value_at(Fraction::ONE), 20.0);
    }

    /// Between two keys the value lerps linearly.
    #[test]
    fn values_lerp_between_neighbouring_keys() {
        let keys = Keyframes::new((Fraction::ZERO, 0.0_f64), [(Fraction::HALF, 10.0), (Fraction::ONE, 20.0)]);
        assert_eq!(keys.value_at(Fraction::new(0.25)), 5.0);
        assert_eq!(keys.value_at(Fraction::new(0.75)), 15.0);
    }

    /// Unsorted keys land in progress order.
    #[test]
    fn keys_are_sorted_on_construction() {
        let keys = Keyframes::new((Fraction::ONE, 20.0_f64), [(Fraction::ZERO, 0.0), (Fraction::HALF, 10.0)]);
        assert_eq!(keys.keys[0].0, Fraction::ZERO);
        assert_eq!(keys.keys[2].0, Fraction::ONE);
    }
}
