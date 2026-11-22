use serde::Deserialize;
use serde::Serialize;

/// Shredded strip count (Shredder's `shredder-strips`, BMW `uCount`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct StripCount(pub u32);

impl StripCount {
    /// Creates a strip count.
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
