use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::ops::Mul;

/// A uniform scale factor (`1.0` = natural size).
///
/// Wraps the plain `f64` so factors cannot be confused with other
/// scalar values flowing through the pipeline. A technical primitive
/// without validation: negative factors legitimately express mirrored
/// transforms; domain limits (degenerate-size floors, per-object
/// bounds) belong to consumer wrappers.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScaleFactor(pub f64);

impl ScaleFactor {
    /// Identity scale - natural size, no zoom.
    pub const ONE: Self = Self(1.0);

    /// Creates a scale factor.
    pub const fn new(factor: f64) -> Self {
        Self(factor)
    }

    /// The factor (`1.0` = natural size).
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The factor as `f32` for `f32`-based framework APIs (e.g. GTK4).
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }

    /// Whether the factor is `1.0` - scaling by it changes nothing.
    pub fn is_identity(self) -> bool {
        self.0 == 1.0
    }

    /// The reciprocal `1/factor` - the scale that undoes this one
    /// (`a * a.inverse()` composes to identity for nonzero `a`).
    pub fn inverse(self) -> Self {
        Self(1.0 / self.0)
    }

    /// Linear interpolation `from + t * (to - from)` - plain
    /// lerp for animations; unlike [`crate::RotationRad`] there is
    /// no wrap-around to take the shortest way across.
    pub fn lerp(from: ScaleFactor, to: ScaleFactor, t: f64) -> Self {
        Self(from.0 + t * (to.0 - from.0))
    }
}

impl Default for ScaleFactor {
    fn default() -> Self {
        Self::ONE
    }
}

impl Mul for ScaleFactor {
    type Output = ScaleFactor;

    /// Composes two scales - zooming by `a` then `b` yields `a * b`.
    fn mul(self, other: ScaleFactor) -> ScaleFactor {
        ScaleFactor(self.0 * other.0)
    }
}

impl<U> Mul<ScaleFactor> for Vec2<U> {
    type Output = Vec2<U>;

    /// Scales the vector uniformly (`v * factor`).
    fn mul(self, factor: ScaleFactor) -> Vec2<U> {
        self * factor.0
    }
}

impl From<f64> for ScaleFactor {
    fn from(factor: f64) -> Self {
        Self::new(factor)
    }
}

impl From<ScaleFactor> for f64 {
    fn from(factor: ScaleFactor) -> Self {
        factor.0
    }
}

impl Display for ScaleFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for ScaleFactor {
    const SIGNATURE: &'static zvariant::Signature = <f64 as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(ScaleFactor);

#[cfg(test)]
mod tests {
    use super::ScaleFactor;
    use crate::Pixels;
    use crate::Vec2;

    #[test]
    fn default_is_identity() {
        assert_eq!(ScaleFactor::default().as_f64(), 1.0);
        assert!(ScaleFactor::ONE.is_identity());
        assert!(!ScaleFactor::new(2.0).is_identity());
    }

    /// Composition multiplies: `2x` then `3x` is `6x`.
    #[test]
    fn mul_composes() {
        let zoom = ScaleFactor::new(2.0) * ScaleFactor::new(3.0);
        assert_eq!(zoom.as_f64(), 6.0);
    }

    /// `a * a.inverse()` lands back at identity.
    #[test]
    fn inverse_undoes_scale() {
        let scale = ScaleFactor::new(4.0);
        assert!((scale * scale.inverse()).as_f64() - 1.0 < 1e-9);
        assert_eq!(scale.inverse().as_f64(), 0.25);
    }

    /// Lerp hits the endpoints and the midpoint.
    #[test]
    fn lerp_interpolates() {
        let (a, b) = (ScaleFactor::new(1.0), ScaleFactor::new(2.0));
        assert_eq!(ScaleFactor::lerp(a, b, 0.0), a);
        assert_eq!(ScaleFactor::lerp(a, b, 1.0), b);
        assert_eq!(ScaleFactor::lerp(a, b, 0.5).as_f64(), 1.5);
    }

    /// A vector scales uniformly in both components.
    #[test]
    fn vec_mul_scales_both_axes() {
        let v = Vec2::<Pixels>::new(4.0, -2.0) * ScaleFactor::new(0.5);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, -1.0);
    }
}
