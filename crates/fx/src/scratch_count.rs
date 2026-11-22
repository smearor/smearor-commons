use serde::Deserialize;
use serde::Serialize;

/// Number of claw strikes on the window (Trex'
/// `trex-scratch_count`, BMW `uScratchCount`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct ScratchCount(pub u32);

impl ScratchCount {
    /// Creates a scratch count.
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
