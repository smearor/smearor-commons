use serde::Deserialize;
use serde::Serialize;

/// How [`crate::SeatLayout`] assigns items without an explicit
/// [`crate::LayoutItem::seat`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeatAssignment {
    /// Round-robin over [`crate::TableSide::ALL`] in input order:
    /// item `i` lands on `ALL[i % 4]`.
    #[default]
    RoundRobin,
}
