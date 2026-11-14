//! A regular grid partitioning of a rectangular extent.
//!
//! ```
//! use smearor_commons_geometry::{Cell, Col, Grid, Rect, Row, Vec2};
//!
//! let grid = Grid::for_items(9, 4); // 4 cols, 3 rows
//! let cell = grid.cell(5).expect("in range");
//! assert_eq!(cell, Cell::new(Col(1), Row(1)));
//!
//! let extent = Vec2::<(), u32>::new(800, 600);
//! assert_eq!(grid.rect(cell, extent), Rect::new(Vec2::new(200, 200), Vec2::new(200, 200)));
//! ```

use crate::Cell;
use crate::Col;
use crate::QuarterTurn;
use crate::Rect;
use crate::Row;
use crate::Vec2;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

/// A regular grid of `cols` × `rows` equal-sized cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Grid {
    cols: u32,
    rows: u32,
}

impl Grid {
    /// A grid of `cols` columns and `rows` rows.
    ///
    /// `0` clamps to `1`: an empty grid partitions nothing and would only
    /// surface as division by zero downstream.
    pub fn new(cols: u32, rows: u32) -> Self {
        Self {
            cols: cols.max(1),
            rows: rows.max(1),
        }
    }

    /// A grid of `cols` columns with just enough rows to hold
    /// `item_count` items in row-major order.
    ///
    /// `0` items still yields one row: the grid describes a partition,
    /// not the items.
    pub fn for_items(item_count: u32, cols: u32) -> Self {
        let cols = cols.max(1);
        Self::new(cols, item_count.div_ceil(cols).max(1))
    }

    /// Number of columns.
    pub fn cols(&self) -> u32 {
        self.cols
    }

    /// Number of rows.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Total number of cells.
    pub fn capacity(&self) -> u32 {
        self.cols * self.rows
    }

    /// The [`Cell`] at linear `index` in row-major order, or `None`
    /// when `index` exceeds [`capacity`](Self::capacity).
    pub fn cell(&self, index: u32) -> Option<Cell> {
        (index < self.capacity()).then(|| Cell::new(Col(index % self.cols), Row(index / self.cols)))
    }

    /// The linear row-major index of `cell`, or `None` when `cell`
    /// addresses outside this grid.
    pub fn index(&self, cell: Cell) -> Option<u32> {
        (cell.col.0 < self.cols && cell.row.0 < self.rows).then(|| cell.row.0 * self.cols + cell.col.0)
    }

    /// The grid rotated by `turn`. Quarter turns transpose the
    /// partition: `Grid::new(4, 3).rotated(Deg90)` is 3 × 4.
    pub fn rotated(&self, turn: QuarterTurn) -> Self {
        match turn {
            QuarterTurn::Deg0 | QuarterTurn::Deg180 => *self,
            QuarterTurn::Deg90 | QuarterTurn::Deg270 => Self::new(self.rows, self.cols),
        }
    }

    /// The cell rotated by `turn` — shorthand for
    /// `cell.rotated(turn, self)`.
    pub fn rotate_cell(&self, turn: QuarterTurn, cell: Cell) -> Cell {
        cell.rotated(turn, self)
    }

    /// The extent of a single cell when `extent` is partitioned by this
    /// grid. Integer extents keep the remainder unassigned at the
    /// right/bottom edge.
    pub fn cell_size<U, T>(&self, extent: Vec2<U, T>) -> Vec2<U, T>
    where
        T: Copy + Div<Output = T> + From<u32>,
    {
        Vec2::new(extent.x / T::from(self.cols), extent.y / T::from(self.rows))
    }

    /// The [`Rect`] of `cell` inside `extent`, with the partition
    /// anchored at the origin — shorthand for
    /// `rect_within(cell, Rect::new(origin, extent))`.
    pub fn rect<U, T>(&self, cell: Cell, extent: Vec2<U, T>) -> Rect<U, T>
    where
        T: Copy + Default + Add<Output = T> + Div<Output = T> + Mul<Output = T> + From<u32>,
    {
        self.rect_within(cell, Rect::new(Vec2::default(), extent))
    }

    /// The [`Rect`] of `cell` inside `frame` — like [`rect`](Self::rect),
    /// but the partition is anchored at `frame.min` (e.g. a board
    /// rectangle offset inside a surface).
    pub fn rect_within<U, T>(&self, cell: Cell, frame: Rect<U, T>) -> Rect<U, T>
    where
        T: Copy + Add<Output = T> + Div<Output = T> + Mul<Output = T> + From<u32>,
    {
        let size = self.cell_size(frame.size);
        Rect::new(frame.min + Vec2::new(T::from(cell.col.0) * size.x, T::from(cell.row.0) * size.y), size)
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::Cell;
    use crate::Col;
    use crate::QuarterTurn;
    use crate::Rect;
    use crate::Row;
    use crate::Vec2;

    #[test]
    fn for_items_rounds_rows_up() {
        let grid = Grid::for_items(9, 4);
        assert_eq!(grid.cols(), 4);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.capacity(), 12);
    }

    #[test]
    fn for_items_zero_items_keeps_one_row() {
        assert_eq!(Grid::for_items(0, 4).rows(), 1);
    }

    #[test]
    fn new_clamps_zero_dimensions() {
        let grid = Grid::new(0, 0);
        assert_eq!(grid.cols(), 1);
        assert_eq!(grid.rows(), 1);
    }

    #[test]
    fn cell_maps_row_major() {
        let grid = Grid::new(4, 3);
        assert_eq!(grid.cell(0), Some(Cell::new(Col(0), Row(0))));
        assert_eq!(grid.cell(5), Some(Cell::new(Col(1), Row(1))));
        assert_eq!(grid.cell(11), Some(Cell::new(Col(3), Row(2))));
        assert_eq!(grid.cell(12), None);
    }

    #[test]
    fn index_is_the_inverse_of_cell() {
        let grid = Grid::new(4, 3);
        for index in 0..grid.capacity() {
            let cell = grid.cell(index).expect("in range");
            assert_eq!(grid.index(cell), Some(index));
        }
        assert_eq!(grid.index(Cell::new(Col(4), Row(0))), None);
    }

    #[test]
    fn cell_size_divides_extent() {
        let grid = Grid::new(4, 3);
        assert_eq!(grid.cell_size(Vec2::<(), u32>::new(800, 600)), Vec2::new(200, 200));
        assert_eq!(Grid::new(8, 8).cell_size(Vec2::<(), f64>::new(80.0, 80.0)), Vec2::new(10.0, 10.0));
    }

    #[test]
    fn rect_places_cell_in_extent() {
        let grid = Grid::new(4, 3);
        let rect = grid.rect(Cell::new(Col(1), Row(1)), Vec2::<(), u32>::new(800, 600));
        assert_eq!(rect, Rect::new(Vec2::new(200, 200), Vec2::new(200, 200)));
    }

    #[test]
    fn rect_within_offsets_by_frame_origin() {
        let grid = Grid::new(8, 8);
        let frame = Rect::new(Vec2::<(), f64>::new(10.0, 20.0), Vec2::new(80.0, 80.0));
        let rect = grid.rect_within(Cell::new(Col(2), Row(1)), frame);
        assert_eq!(rect, Rect::new(Vec2::new(30.0, 30.0), Vec2::new(10.0, 10.0)));
    }

    #[test]
    fn rotated_transposes_on_quarter_turns() {
        let grid = Grid::new(4, 3);
        assert_eq!(grid.rotated(QuarterTurn::Deg90), Grid::new(3, 4));
        assert_eq!(grid.rotated(QuarterTurn::Deg270), Grid::new(3, 4));
        assert_eq!(grid.rotated(QuarterTurn::Deg180), grid);
        assert_eq!(grid.rotated(QuarterTurn::Deg0), grid);
    }
}
