//! `Lacunarity` - frequency multiplier per [`crate::Fbm`] octave.

use serde::Deserialize;
use serde::Serialize;

/// Frequency multiplier per [`crate::Fbm`] octave - the classic fBm
/// lacunarity is `2.0` (each octave doubles in frequency).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Lacunarity(pub f64);
