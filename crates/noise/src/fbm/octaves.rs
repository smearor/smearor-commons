//! `Octaves` - octave count of an [`crate::Fbm`].

use serde::Deserialize;
use serde::Serialize;

/// Octave count of an [`crate::Fbm`] - `1..=8` is the sensible range;
/// the wrapper prevents accidentally feeding a `Gain` where an octave
/// count belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Octaves(pub u32);
