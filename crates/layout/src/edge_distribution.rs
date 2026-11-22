use serde::Deserialize;
use serde::Serialize;

/// How an [`crate::EdgeStripLayout`] spreads its items over the
/// selected [`crate::SideSet`] edges.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeDistribution {
    /// Fill side by side in [`crate::SideSet::iter`] order: items dock
    /// to the first side until its edge is full, the overflow spills
    /// onto the next side. An item that fits no remaining side is
    /// reported [`crate::LayoutError::DoesNotFit`].
    #[default]
    Sequential,
    /// Split the items evenly over all sides (largest remainder to the
    /// earlier sides) and center each group on its edge. A group
    /// exceeding its edge is reported [`crate::LayoutError::DoesNotFit`].
    Balanced,
}
