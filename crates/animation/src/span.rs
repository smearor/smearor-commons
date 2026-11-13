use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// A two-endpoint trajectory: interpolates `from -> to` along the
/// progress `t`.
///
/// `Span<RotationRad>` animated on a clock is the rotation
/// animation; `Span<ScaleFactor>` the zoom animation; any other
/// [`Interpolate`] value type works the same way.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Span<T> {
    /// The value at progress `0`.
    pub from: T,
    /// The value at progress `1`.
    pub to: T,
}

impl<T> Span<T> {
    /// Creates a trajectory between two endpoints.
    pub const fn new(from: T, to: T) -> Self {
        Self { from, to }
    }
}

impl<T: Interpolate + Copy> Trajectory for Span<T> {
    type Value = T;

    fn value_at(&self, t: Fraction) -> T {
        self.from.interpolate(self.to, t)
    }
}

#[cfg(test)]
mod tests {
    use super::Span;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// A span lerps linearly across its endpoints.
    #[test]
    fn span_interpolates_between_endpoints() {
        let span = Span::new(10.0_f64, 20.0);
        assert_eq!(span.value_at(Fraction::new(0.0)), 10.0);
        assert_eq!(span.value_at(Fraction::new(0.25)), 12.5);
        assert_eq!(span.value_at(Fraction::new(1.0)), 20.0);
    }
}
