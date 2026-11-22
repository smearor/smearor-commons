use serde::Deserialize;
use serde::Serialize;

/// Angular segment count for wheel-style masks (PixelWheel's spokes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct SpokeCount(pub u32);

impl SpokeCount {
    /// Creates a spoke count.
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
