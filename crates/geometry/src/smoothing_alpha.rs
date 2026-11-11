use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::ops::Mul;

/// Exponential-moving-average weight of the newest sample
/// (`pos = alpha*raw + (1-alpha)*prev`).
///
/// Averages out the differential-latency distortion sensors report during
/// motion. `0.0` disables smoothing, `1.0` passes raw values through.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SmoothingAlpha(f64);

impl SmoothingAlpha {
    /// Creates a smoothing factor.
    pub const fn new(alpha: f64) -> Self {
        Self(alpha)
    }

    /// The EMA weight of the newest sample.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Whether running the smoothing algorithm changes anything -
    /// `0.0` freezes the value (disabled), `1.0` passes samples
    /// through (identity); only `0 < alpha < 1` blends.
    pub fn is_effective(&self) -> bool {
        self.0 > 0.0 && self.0 < 1.0
    }

    /// The complementary weight `1 − alpha` - the previous value's
    /// share of an EMA blend (`pos * alpha + prev * alpha.complement()`).
    pub fn complement(self) -> Self {
        Self(1.0 - self.0)
    }
}

impl<U> Mul<SmoothingAlpha> for Vec2<U> {
    type Output = Vec2<U>;

    /// Scales the vector by the weight (`v * alpha`).
    fn mul(self, alpha: SmoothingAlpha) -> Vec2<U> {
        self * alpha.as_f64()
    }
}

impl Default for SmoothingAlpha {
    fn default() -> Self {
        Self(0.6)
    }
}

impl From<f64> for SmoothingAlpha {
    fn from(alpha: f64) -> Self {
        Self::new(alpha)
    }
}

impl Display for SmoothingAlpha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for SmoothingAlpha {
    const SIGNATURE: &'static zvariant::Signature = <f64 as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(SmoothingAlpha);
