use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Relative anchor inside bounds: `horizontal` `0.0` = left edge,
/// `0.5` = centered, `1.0` = right edge; `vertical` `0.0` = top,
/// `1.0` = bottom. Strategies use it to pick start corners and
/// edge attachment without naming a widget type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AnchorWeights {
    /// `0.0` = left, `0.5` = centered, `1.0` = right.
    pub horizontal: Fraction,
    /// `0.0` = top, `0.5` = centered, `1.0` = bottom.
    pub vertical: Fraction,
}

impl AnchorWeights {
    /// Top-left corner.
    pub const TOP_LEFT: Self = Self {
        horizontal: Fraction::ZERO,
        vertical: Fraction::ZERO,
    };
    /// Top edge, centered.
    pub const TOP_CENTER: Self = Self {
        horizontal: Fraction::HALF,
        vertical: Fraction::ZERO,
    };
    /// Top-right corner.
    pub const TOP_RIGHT: Self = Self {
        horizontal: Fraction::ONE,
        vertical: Fraction::ZERO,
    };
    /// Left edge, centered.
    pub const CENTER_LEFT: Self = Self {
        horizontal: Fraction::ZERO,
        vertical: Fraction::HALF,
    };
    /// Dead center.
    pub const CENTER: Self = Self {
        horizontal: Fraction::HALF,
        vertical: Fraction::HALF,
    };
    /// Right edge, centered.
    pub const CENTER_RIGHT: Self = Self {
        horizontal: Fraction::ONE,
        vertical: Fraction::HALF,
    };
    /// Bottom-left corner.
    pub const BOTTOM_LEFT: Self = Self {
        horizontal: Fraction::ZERO,
        vertical: Fraction::ONE,
    };
    /// Bottom edge, centered.
    pub const BOTTOM_CENTER: Self = Self {
        horizontal: Fraction::HALF,
        vertical: Fraction::ONE,
    };
    /// Bottom-right corner.
    pub const BOTTOM_RIGHT: Self = Self {
        horizontal: Fraction::ONE,
        vertical: Fraction::ONE,
    };

    /// Creates an anchor from horizontal and vertical weights.
    pub const fn new(horizontal: Fraction, vertical: Fraction) -> Self {
        Self { horizontal, vertical }
    }
}

/// Top-left - the natural start of a left-to-right, top-to-bottom flow.
impl Default for AnchorWeights {
    fn default() -> Self {
        Self::TOP_LEFT
    }
}
