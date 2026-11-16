use crate::Placement;
use crate::TableSide;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;

/// A seat on a tabletop surface: which [`TableSide`] it faces plus
/// where along that edge it sits.
///
/// A seat is more than a side - users can sit anywhere along an edge
/// (two on one long side, off-center), so the offset carries the
/// position along the edge axis. `offset` `None` means "assigned to
/// the side, a strategy may distribute the item along it";
/// `Some(fraction)` pins the item to that edge position
/// (`HALF` = centered). Which *user* sits there stays consumer domain.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Seat {
    /// The table edge the seat faces.
    pub side: TableSide,
    /// Position along the edge axis (`0.0` = edge start, `1.0` = edge
    /// end, `0.5` = centered). `None` leaves distribution along the
    /// side to the strategy.
    pub offset: Option<Fraction>,
}

impl Seat {
    /// A seat assigned to `side` without a pinned position - a
    /// strategy distributes the item along the edge.
    pub const fn new(side: TableSide) -> Self {
        Self { side, offset: None }
    }

    /// A seat pinned to `offset` along the edge of `side`.
    pub const fn at(side: TableSide, offset: Fraction) -> Self {
        Self { side, offset: Some(offset) }
    }

    /// A seat centered on `side`'s edge.
    pub const fn center(side: TableSide) -> Self {
        Self::at(side, Fraction::HALF)
    }

    /// The seat one step clockwise (`side` steps, `offset` keeps).
    pub fn clockwise(&self) -> Self {
        Self {
            side: self.side.clockwise(),
            ..*self
        }
    }

    /// The seat one step counter-clockwise.
    pub fn counter_clockwise(&self) -> Self {
        Self {
            side: self.side.counter_clockwise(),
            ..*self
        }
    }

    /// The centered seat nearest a (free) rotation.
    pub fn nearest(rotation: RotationRad) -> Self {
        Self::center(TableSide::nearest(rotation))
    }

    /// Places `footprint` in front of this seat: `margin` (scalar in
    /// the space of `bounds`) off the edge, positioned along the
    /// edge at `offset` (unpinned offsets resolve to `HALF`),
    /// rotated so a user sitting there reads it correctly.
    ///
    /// `footprint` is the item's extent *as displayed at the seat's
    /// rotation* - compute it with [`crate::footprint_for`] when the
    /// placement changes orientation.
    pub fn placement<U>(&self, bounds: Rect<U>, footprint: Vec2<U>, margin: f64) -> Placement<U> {
        let offset = self.offset.unwrap_or(Fraction::HALF).as_f64();
        let center = match self.side {
            TableSide::Bottom => Vec2::new(bounds.min.x + bounds.size.x * offset, bounds.min.y + bounds.size.y - margin - footprint.y / 2.0),
            TableSide::Left => Vec2::new(bounds.min.x + margin + footprint.x / 2.0, bounds.min.y + bounds.size.y * offset),
            TableSide::Top => Vec2::new(bounds.min.x + bounds.size.x * offset, bounds.min.y + margin + footprint.y / 2.0),
            TableSide::Right => Vec2::new(bounds.min.x + bounds.size.x - margin - footprint.x / 2.0, bounds.min.y + bounds.size.y * offset),
        };
        Placement::new(center, Rotation::QuarterTurn(self.side.to_quarter_turn()), footprint)
    }
}

#[cfg(test)]
mod tests {
    use super::Seat;
    use crate::TableSide;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::RotationRad;
    use smearor_commons_geometry::Vec2;

    fn bounds() -> Rect<Pixels> {
        Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(400.0, 300.0),
        }
    }

    /// `placement` puts the footprint against the correct edge,
    /// `margin` off it, centered on the edge axis, rotated for
    /// the side.
    #[test]
    fn placement_attaches_to_each_edge() {
        let footprint = Vec2::new(100.0, 50.0);
        let cases = [
            (TableSide::Bottom, 200.0, 300.0 - 24.0 - 25.0, QuarterTurn::Deg0),
            (TableSide::Left, 24.0 + 50.0, 150.0, QuarterTurn::Deg90),
            (TableSide::Top, 200.0, 24.0 + 25.0, QuarterTurn::Deg180),
            (TableSide::Right, 400.0 - 24.0 - 50.0, 150.0, QuarterTurn::Deg270),
        ];
        for (side, x, y, turn) in cases {
            let placement = Seat::center(side).placement(bounds(), footprint, 24.0);
            assert!((placement.center.x - x).abs() < 1e-9, "{side:?} x");
            assert!((placement.center.y - y).abs() < 1e-9, "{side:?} y");
            assert_eq!(placement.rotation, Rotation::QuarterTurn(turn));
        }
    }

    /// A pinned offset positions the footprint along the edge axis.
    #[test]
    fn offset_moves_along_the_edge() {
        let footprint = Vec2::new(50.0, 50.0);
        let quarter = Seat::at(TableSide::Bottom, Fraction::new(0.25)).placement(bounds(), footprint, 0.0);
        assert!((quarter.center.x - 100.0).abs() < 1e-9);
        let top = Seat::at(TableSide::Left, Fraction::new(0.75)).placement(bounds(), footprint, 0.0);
        assert!((top.center.y - 225.0).abs() < 1e-9);
    }

    /// Stepping keeps the offset while the side moves.
    #[test]
    fn clockwise_keeps_offset() {
        let seat = Seat::at(TableSide::Bottom, Fraction::new(0.25));
        assert_eq!(seat.clockwise(), Seat::at(TableSide::Left, Fraction::new(0.25)));
        assert_eq!(seat.counter_clockwise(), Seat::at(TableSide::Right, Fraction::new(0.25)));
    }

    /// `nearest` yields the centered seat of the nearest side.
    #[test]
    fn nearest_is_centered() {
        assert_eq!(Seat::nearest(RotationRad::from_degrees(100.0)), Seat::center(TableSide::Left));
    }

    /// Serde round-trips a seat with and without a pinned offset.
    #[test]
    fn serde_round_trip() {
        for seat in [
            Seat::new(TableSide::Top),
            Seat::center(TableSide::Bottom),
            Seat::at(TableSide::Left, Fraction::new(0.3)),
        ] {
            let json = serde_json::to_string(&seat).expect("serialize");
            assert_eq!(serde_json::from_str::<Seat>(&json).expect("deserialize"), seat);
        }
    }
}
