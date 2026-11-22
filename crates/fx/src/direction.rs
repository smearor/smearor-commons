use serde::Deserialize;
use serde::Serialize;

/// Which way the transition runs - Burn-My-Windows' `uForOpening`.
///
/// Progress always runs `0 -> 1`; effects branch on the direction,
/// they do not get a flipped clock (BMW semantics, kept deliberately:
/// particles that fly *in* on open fly *out* on close, not backward).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// The content appears (window open, card created).
    #[default]
    Open,
    /// The content disappears (window close, card deleted).
    Close,
}

impl Direction {
    /// The progress at which the content is fully visible for this
    /// direction - `1.0` for `Open`, `0.0` for `Close`. Effects that
    /// fade alpha can weight it by `progress` directly via
    /// [`Direction::weight`].
    pub const fn visible_progress(self) -> f64 {
        match self {
            Self::Open => 1.0,
            Self::Close => 0.0,
        }
    }

    /// The visibility weight of a frame: `progress` for `Open`,
    /// `1 - progress` for `Close` - `1.0` is fully visible.
    pub fn weight(self, progress: f64) -> f64 {
        match self {
            Self::Open => progress,
            Self::Close => 1.0 - progress,
        }
    }
}
