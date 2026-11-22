use serde::Deserialize;
use serde::Serialize;

/// Quantized growth steps (Materialize's `materialize-steps`) -
/// `0` renders a smooth fade instead of stepped panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct StepCount(pub u32);

impl StepCount {
    /// Creates a step count.
    pub const fn new(count: u32) -> Self {
        Self(count)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
