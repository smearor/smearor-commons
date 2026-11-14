//! A cell position within a [`Grid`].

use crate::Col;
use crate::Grid;
use crate::QuarterTurn;
use crate::Row;

/// A position within a [`Grid`]: the intersection of a [`Col`] and a
/// [`Row`]. Resolve to a [`Rect`](crate::Rect) via
/// [`Grid::rect`](crate::Grid::rect).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cell {
    /// Column of the cell.
    pub col: Col,
    /// Row of the cell.
    pub row: Row,
}

impl Cell {
    /// A cell at `col`/`row`.
    pub fn new(col: Col, row: Row) -> Self {
        Self { col, row }
    }

    /// The cell rotated by `turn` inside `grid`.
    ///
    /// Quarter turns transpose the grid — pair the result with
    /// [`Grid::rotated`] for geometry in the rotated frame:
    /// `cell.rotated(Deg90, grid)` addresses into `grid.rotated(Deg90)`.
    pub fn rotated(self, turn: QuarterTurn, grid: &Grid) -> Self {
        let (cols, rows) = (grid.cols(), grid.rows());
        match turn {
            QuarterTurn::Deg0 => self,
            QuarterTurn::Deg90 => Self::new(Col(rows - 1 - self.row.0), Row(self.col.0)),
            QuarterTurn::Deg270 => Self::new(Col(self.row.0), Row(cols - 1 - self.col.0)),
            QuarterTurn::Deg180 => Self::new(Col(cols - 1 - self.col.0), Row(rows - 1 - self.row.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use crate::Col;
    use crate::Grid;
    use crate::QuarterTurn;
    use crate::Row;

    #[test]
    fn rotated_matches_row_major_index_rotation() {
        // On a square grid a clockwise quarter turn maps linear index
        // (c + r*8) to c*8 + (7 - r) — the chess FieldIndex::rot_cw.
        let grid = Grid::new(8, 8);
        for index in 0..grid.capacity() {
            let cell = grid.cell(index).expect("in range");
            let rotated = cell.rotated(QuarterTurn::Deg90, &grid);
            let expected = (index % 8) * 8 + (7 - index / 8);
            assert_eq!(grid.index(rotated), Some(expected));
        }
    }

    #[test]
    fn rotated_cells_land_back_via_four_turns() {
        let grid = Grid::new(4, 3);
        let cell = Cell::new(Col(1), Row(2));
        let mut g = grid;
        let mut c = cell;
        for _ in 0..4 {
            c = c.rotated(QuarterTurn::Deg90, &g);
            g = g.rotated(QuarterTurn::Deg90);
        }
        assert_eq!(g, grid);
        assert_eq!(c, cell);
    }
}
