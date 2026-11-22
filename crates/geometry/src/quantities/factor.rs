use serde::Deserialize;
use serde::Serialize;

/// A signed, unitless multiplier (`0.0` = neutral, negative values
/// invert).
///
/// The general-purpose sibling of [`crate::ScaleFactor`]: where
/// `ScaleFactor` describes a size relation (zoom level, `1.0` =
/// natural), `Factor` marks any plain signed coefficient - tilt,
/// squish, drift speed - so it cannot be confused with other scalar
/// quantities flowing through the pipeline. No validation; domain
/// bounds belong to consumer wrappers.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Factor(pub f64);

impl Factor {
    /// Neutral factor - no contribution.
    pub const ZERO: Self = Self(0.0);

    /// Unit factor - full strength in the positive direction.
    pub const ONE: Self = Self(1.0);

    /// Creates a factor.
    pub const fn new(factor: f64) -> Self {
        Self(factor)
    }

    /// The numeric value.
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<Factor> for f64 {
    fn from(factor: Factor) -> Self {
        factor.0
    }
}

impl Default for Factor {
    fn default() -> Self {
        Self::ONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_is_transparent() {
        let factor = Factor::new(-0.3);
        let json = serde_json::to_string(&factor).unwrap();
        assert_eq!(json, "-0.3");
        let back: Factor = serde_json::from_str(&json).unwrap();
        assert_eq!(back, factor);
    }
}
