use serde::Deserialize;
use serde::Serialize;

/// Taps per ray of a blur cone (Focus' `uBlurQuality`) - the number
/// of samples taken along each of the cone's directions. Higher
/// values smooth the blur at linear sampling cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct BlurQuality(pub u32);

impl BlurQuality {
    /// Creates a blur quality - taps per ray.
    pub const fn new(quality: u32) -> Self {
        Self(quality)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
