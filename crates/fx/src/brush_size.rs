use serde::Deserialize;
use serde::Serialize;

/// Brush thickness in discrete steps (PaintBrush's
/// `paint-brush-size`, BMW `uSize`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct BrushSize(pub u32);

impl BrushSize {
    /// Creates a brush size.
    pub const fn new(size: u32) -> Self {
        Self(size)
    }

    /// The numeric value.
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
