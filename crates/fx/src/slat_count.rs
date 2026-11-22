use serde::Deserialize;
use serde::Serialize;

/// Turning slat count (VenetianBlinds' `venetian-blinds-slats`, BMW
/// `uSlats` / shader `MAX_SLATS` 1..=15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct SlatCount(pub u32);

impl SlatCount {
    /// Creates a slat count.
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
