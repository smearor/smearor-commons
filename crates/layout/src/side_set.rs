use crate::LayoutError;
use crate::TableSide;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use std::fmt;

/// A non-empty set of [`TableSide`]s - one to four edges of a
/// rectangular surface.
///
/// Multi-edge arrangements (edge strips docking to several sides of a
/// host rectangle) take their edge selection as a `SideSet` so "at
/// least one side" is a type-level invariant, not a convention.
/// Membership and iteration follow the clockwise [`TableSide::ALL`]
/// order, making `iter()` output deterministic regardless of
/// construction order.
///
/// Serde represents the set as a list of side names
/// (`["bottom","left"]`) reusing `TableSide`'s lowercase wire format.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SideSet {
    /// Membership bitmask - bit `i` corresponds to `TableSide::ALL[i]`.
    bits: u8,
}

impl SideSet {
    /// All four sides.
    pub const ALL: Self = Self { bits: 0b1111 };

    /// A set holding only `side`.
    pub const fn single(side: TableSide) -> Self {
        Self { bits: side_bit(side) }
    }

    /// Builds a set from the given sides - duplicates collapse.
    /// An empty iterator yields [`LayoutError::EmptySides`]: the
    /// non-empty invariant is enforced here, not assumed.
    pub fn try_from_sides(sides: impl IntoIterator<Item = TableSide>) -> Result<Self, LayoutError> {
        let mut set = Self { bits: 0 };
        for side in sides {
            set.bits |= side_bit(side);
        }
        if set.bits == 0 { Err(LayoutError::EmptySides) } else { Ok(set) }
    }

    /// Whether `side` is part of the set.
    pub const fn contains(&self, side: TableSide) -> bool {
        self.bits & side_bit(side) != 0
    }

    /// The contained sides in clockwise [`TableSide::ALL`] order.
    pub fn iter(&self) -> impl Iterator<Item = TableSide> + '_ {
        TableSide::ALL.into_iter().filter(|side| self.contains(*side))
    }

    /// The number of contained sides - always `1..=4`.
    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    /// Always `false` - the non-empty invariant holds by construction.
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Whether all four sides are contained.
    pub fn is_full(&self) -> bool {
        self.bits == Self::ALL.bits
    }

    /// Adds `side` to the set - the non-empty invariant is unaffected.
    pub fn insert(&mut self, side: TableSide) {
        self.bits |= side_bit(side);
    }
}

/// The membership bit of a side - its index in [`TableSide::ALL`].
const fn side_bit(side: TableSide) -> u8 {
    match side {
        TableSide::Bottom => 0b0001,
        TableSide::Left => 0b0010,
        TableSide::Top => 0b0100,
        TableSide::Right => 0b1000,
    }
}

/// A lone side is a one-element set.
impl From<TableSide> for SideSet {
    fn from(side: TableSide) -> Self {
        Self::single(side)
    }
}

/// `SideSet::default()` is the set holding only the default side
/// ([`TableSide::Bottom`]) - never an empty set.
impl Default for SideSet {
    fn default() -> Self {
        Self::single(TableSide::Bottom)
    }
}

/// The sides as a readable list (`{bottom, left}`).
impl fmt::Display for SideSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// `Debug` shows the sides, not the bitmask (`{Bottom, Left}`).
impl fmt::Debug for SideSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// Serialized as the list of contained side names.
impl Serialize for SideSet {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.iter().collect::<Vec<_>>().serialize(serializer)
    }
}

/// Deserialized from a list of side names; an empty list fails the
/// non-empty invariant with [`LayoutError::EmptySides`].
impl<'de> Deserialize<'de> for SideSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from_sides(Vec::<TableSide>::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::SideSet;
    use crate::LayoutError;
    use crate::TableSide;

    /// Membership answers the constructed sides only.
    #[test]
    fn contains_reports_membership() {
        let set = SideSet::try_from_sides([TableSide::Bottom, TableSide::Left]).expect("non-empty");
        assert!(set.contains(TableSide::Bottom));
        assert!(set.contains(TableSide::Left));
        assert!(!set.contains(TableSide::Top));
        assert!(!set.contains(TableSide::Right));
    }

    /// Iteration follows `TableSide::ALL` order, not insertion order.
    #[test]
    fn iter_walks_clockwise_all_order() {
        let set = SideSet::try_from_sides([TableSide::Right, TableSide::Bottom]).expect("non-empty");
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![TableSide::Bottom, TableSide::Right]);
    }

    /// `len`/`is_full` reflect the membership count.
    #[test]
    fn len_and_is_full() {
        assert_eq!(SideSet::single(TableSide::Top).len(), 1);
        assert_eq!(SideSet::ALL.len(), 4);
        assert!(SideSet::ALL.is_full());
        assert!(!SideSet::single(TableSide::Top).is_full());
        assert!(!SideSet::ALL.is_empty());
    }

    /// Duplicates collapse; empty input is an error, not an empty set.
    #[test]
    fn try_from_sides_enforces_non_empty() {
        let duplicated = SideSet::try_from_sides([TableSide::Bottom, TableSide::Bottom]).expect("non-empty");
        assert_eq!(duplicated.len(), 1);
        assert_eq!(SideSet::try_from_sides([]), Err(LayoutError::EmptySides));
    }

    /// `insert` grows the set; `From<TableSide>` builds a singleton.
    #[test]
    fn insert_and_from() {
        let mut set = SideSet::from(TableSide::Top);
        set.insert(TableSide::Right);
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![TableSide::Top, TableSide::Right]);
    }

    /// The set serializes as a side-name list and rejects `[]`.
    #[test]
    fn serde_round_trip() {
        let set = SideSet::try_from_sides([TableSide::Bottom, TableSide::Left]).expect("non-empty");
        let json = serde_json::to_string(&set).expect("serialize");
        assert_eq!(json, "[\"bottom\",\"left\"]");
        assert_eq!(serde_json::from_str::<SideSet>(&json).expect("deserialize"), set);
        assert!(serde_json::from_str::<SideSet>("[]").is_err());
    }
}
