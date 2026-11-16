//! Tabletop layout primitives - arrangement vocabulary without
//! domain semantics.
//!
//! [`TableSide`] names the four edges of a rectangular surface; a
//! [`Placement`] is the technical target of an arrangement
//! (center + rotation + footprint). A [`LayoutStrategy`] maps a list
//! of [`LayoutItem`]s onto `Placement`s inside bounds - the built-ins
//! are [`FlowLayout`] (side-by-side lines, non-overlapping) and
//! [`SeatLayout`] (the four table seats). Consumer crates keep their
//! semantic types ("player", "card", "note") and map them onto these
//! primitives - which side a user sits at, or what a placed item
//! *is*, stays downstream.
//!
//! Pure `std` with unconditional `serde`; all geometry is unit-generic
//! over the `U` marker of `Vec2`/`Rect` from
//! `smearor-commons-geometry`, so pixels, millimeters and erased
//! coordinates share the same algorithms.

mod anchor_weights;
mod error;
mod flow_layout;
mod layout_item;
mod layout_strategy;
mod placement;
mod seat;
mod seat_assignment;
mod seat_layout;
mod table_side;

pub use anchor_weights::AnchorWeights;
pub use error::LayoutError;
pub use flow_layout::FlowLayout;
pub use layout_item::LayoutItem;
pub use layout_strategy::LayoutStrategy;
pub use placement::Placement;
pub use placement::footprint_for;
pub use seat::Seat;
pub use seat_assignment::SeatAssignment;
pub use seat_layout::SeatLayout;
pub use table_side::TableSide;
