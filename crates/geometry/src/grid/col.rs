//! A column index within a [`Grid`](crate::Grid).

/// Column index within a [`Grid`](crate::Grid) — distinct from
/// [`Row`](crate::Row) so the two cannot be swapped without a type error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Col(pub u32);
