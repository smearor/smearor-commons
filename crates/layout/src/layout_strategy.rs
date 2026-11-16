use crate::LayoutError;
use crate::LayoutItem;
use crate::Placement;
use smearor_commons_geometry::Rect;

/// An arrangement strategy: maps items onto target placements inside
/// `bounds`.
///
/// Strategies are pure functions over technical types - they see
/// footprints, not widgets or domain objects, and stay consumer-
/// extensible: any crate can implement this trait and hand the
/// strategy to a shared caller (e.g. `FloatingBoard::apply_layout`).
///
/// Input and output are index-stable: `placements[i]` answers
/// `items[i]`, so the caller maps back onto its own item list by
/// position. Strategies decide per item which input fields they
/// honor (`rotation`, `seat`) and which they override.
pub trait LayoutStrategy<U> {
    /// Computes the target [`Placement`] for each item.
    ///
    /// Returns [`LayoutError::DoesNotFit`] when the requested options
    /// cannot fit every item inside `bounds`; the consumer decides
    /// how to react.
    fn arrange(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<Placement<U>>, LayoutError>;
}
