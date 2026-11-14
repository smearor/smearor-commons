//! A row index within a [`Grid`](crate::Grid).

/// Row index within a [`Grid`](crate::Grid) — distinct from
/// [`Col`](crate::Col) so the two cannot be swapped without a type error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row(pub u32);
