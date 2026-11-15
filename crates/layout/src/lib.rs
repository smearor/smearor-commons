//! Tabletop layout primitives - arrangement vocabulary without
//! domain semantics.
//!
//! [`TableSide`] names the four edges of a rectangular surface; a
//! [`Placement`] is the technical target of an arrangement
//! (center + quarter-turn rotation + footprint). Consumer crates keep
//! their semantic types ("player", "card", "note") and map them onto
//! these primitives - which side a user sits at, or what a placed
//! item *is*, stays downstream.
//!
//! Pure `std` with unconditional `serde`; all geometry is unit-generic
//! over the `U` marker of `Vec2`/`Rect` from
//! `smearor-commons-geometry`, so pixels, millimeters and erased
//! coordinates share the same algorithms.

mod error;
mod placement;
mod table_side;

pub use error::LayoutError;
pub use placement::Placement;
pub use table_side::TableSide;
