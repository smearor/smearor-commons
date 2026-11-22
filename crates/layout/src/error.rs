use thiserror::Error;

/// Errors from arrangement strategies and layout configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LayoutError {
    /// The items do not fit inside the bounds with the requested
    /// options. The consumer decides how to react - retry with
    /// different options, shrink items or leave them unarranged.
    #[error("item {index} of {items} does not fit inside the bounds")]
    DoesNotFit {
        /// Index of the first item that could not be placed.
        index: usize,
        /// Total number of items handed to the strategy.
        items: usize,
    },
    /// A [`crate::SideSet`] was built from an empty side list - the
    /// set needs at least one [`crate::TableSide`] by invariant.
    #[error("a side set needs at least one table side")]
    EmptySides,
}
