//! A regular grid: `cols` × `rows` equal cells partitioning an extent.
//!
//! [`Grid`] itself is unit-free — it maps linear indices to [`Cell`]
//! positions (row-major, like `FieldIndex` on a chess board) and resolves
//! cells to [`Rect`](crate::Rect)s inside whatever `Vec2<U, T>` extent a
//! caller supplies. [`Col`]/[`Row`] are distinct newtypes so the two
//! cannot be swapped without a type error.

mod cell;
mod col;
mod partition;
mod row;

pub use cell::Cell;
pub use col::Col;
pub use partition::Grid;
pub use row::Row;
