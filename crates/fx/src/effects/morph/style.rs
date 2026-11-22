use serde::Deserialize;
use serde::Serialize;

use crate::FxError;

/// Morph's three style modes (`uStyle`) - serializes as the BMW int
/// (`"style": 2`), not a variant name, so existing payloads keep
/// parsing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(with = "u32"))]
#[serde(try_from = "u32", into = "u32")]
pub enum MorphStyle {
    /// Liquid curl-noise flow (BMW `uStyle = 0`).
    #[default]
    LiquidFlow = 0,
    /// Edge-guided melt (`uStyle = 1`).
    EdgeMelt = 1,
    /// Pixel-block morph (`uStyle = 2`).
    Pixelate = 2,
}

impl From<MorphStyle> for u32 {
    fn from(style: MorphStyle) -> Self {
        style as u32
    }
}

impl TryFrom<u32> for MorphStyle {
    type Error = FxError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LiquidFlow),
            1 => Ok(Self::EdgeMelt),
            2 => Ok(Self::Pixelate),
            other => Err(FxError::ParamMismatch(format!("morph style {other}"))),
        }
    }
}
