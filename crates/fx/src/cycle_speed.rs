use serde::Deserialize;
use serde::Serialize;

/// Cycle rate of a cosine-palette style animation loop - how many
/// color cycles run over the effect's duration (AuraGlow's `uSpeed`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct CycleSpeed(pub f64);

impl CycleSpeed {
    /// Creates a cycle speed.
    pub const fn new(speed: f64) -> Self {
        Self(speed)
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}
