use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::ops::Mul;
use std::time::Duration;

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

    /// Frame-rate independent smoothing factor for one rendered frame.
    ///
    /// Computes `alpha = 1 - exp(-frame_time / tau)` so the same `tau`
    /// time constant closes the same fraction of the remaining gap per
    /// second at any frame rate. A zero `tau` yields `alpha = 1.0`
    /// (instant tracking).
    pub fn from_frame_time(frame_time: Duration, tau: Duration) -> Self {
        if tau.is_zero() {
            return Self(1.0);
        }
        Self(1.0 - (-frame_time.as_secs_f64() / tau.as_secs_f64()).exp())
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

#[cfg(test)]
mod tests {
    use super::SmoothingAlpha;
    use std::time::Duration;

    #[test]
    fn frame_time_scales_with_tau() {
        // One 16 ms frame at tau = 80 ms closes ~18% of the gap.
        let alpha = SmoothingAlpha::from_frame_time(Duration::from_millis(16), Duration::from_millis(80));
        assert!((alpha.as_f64() - 0.181).abs() < 0.001);
        // Zero frame time blends nothing.
        assert_eq!(SmoothingAlpha::from_frame_time(Duration::ZERO, Duration::from_millis(80)).as_f64(), 0.0);
        // Zero tau tracks instantly.
        assert_eq!(SmoothingAlpha::from_frame_time(Duration::from_millis(16), Duration::ZERO).as_f64(), 1.0);
    }
}
