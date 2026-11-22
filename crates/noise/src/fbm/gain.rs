//! `Gain` - amplitude multiplier per [`crate::Fbm`] octave.

use serde::Deserialize;
use serde::Serialize;

/// Amplitude multiplier per [`crate::Fbm`] octave - the classic fBm
/// gain is `0.5` (each octave halves in amplitude).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Gain(pub f64);
