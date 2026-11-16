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

    /// The seat's position along the table perimeter, clockwise in
    /// edge-length units `0.0..4.0`: the bottom edge spans `0..1`
    /// (right-to-left, `offset` decreases), the left edge `1..2`
    /// (bottom-to-top), the top edge `2..3` (left-to-right), the right
    /// edge `3..4` (top-to-bottom) - continuous at the corners, so
    /// seats on neighboring sides compare directly. An unpinned
    /// `offset` resolves to `HALF`, like [`Self::placement`].
    ///
    /// The coordinate orders seats for "pass to the next occupied
    /// seat" and lets consumers position seat indicators around the
    /// table.
    pub fn perimeter_position(&self) -> f64 {
        let offset = self.offset.unwrap_or(Fraction::HALF).as_f64();
        match self.side {
            TableSide::Bottom => 1.0 - offset,
            TableSide::Left => 2.0 - offset,
            TableSide::Top => 2.0 + offset,
            TableSide::Right => 3.0 + offset,
        }
    }

    /// The occupied seat one step clockwise along the table perimeter:
    /// the roster seat with the smallest positive distance in
    /// [`Self::perimeter_position`]. Empty sides are skipped, several
    /// seats on one side are visited in order, and a seat at this
    /// seat's own position does not count.
    ///
    /// Returns `None` when `roster` is empty or holds no position
    /// other than this seat's.
    pub fn next_in(&self, roster: &[Seat]) -> Option<Seat> {
        self.step_in(roster, true)
    }

    /// The occupied seat one step counter-clockwise - the mirror of
    /// [`Self::next_in`].
    pub fn previous_in(&self, roster: &[Seat]) -> Option<Seat> {
        self.step_in(roster, false)
    }

    /// Shared walk of `next_in`/`previous_in`: picks the roster seat
    /// with the smallest strictly-positive perimeter distance in the
    /// chosen direction.
    fn step_in(&self, roster: &[Seat], clockwise: bool) -> Option<Seat> {
        let here = self.perimeter_position();
        let mut best: Option<(Seat, f64)> = None;
        for seat in roster {
            let position = seat.perimeter_position();
            let distance = if clockwise { position - here } else { here - position }.rem_euclid(4.0);
            if distance <= f64::EPSILON {
                continue;
            }
            if best.is_none_or(|(_, shortest)| distance < shortest) {
                best = Some((*seat, distance));
            }
        }
        best.map(|(seat, _)| seat)
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

    /// `perimeter_position` is continuous at the corners: a seat pinned
    /// to a shared corner agrees with its neighbor side's value.
    #[test]
    fn perimeter_is_continuous_at_corners() {
        assert_eq!(Seat::at(TableSide::Bottom, Fraction::new(0.0)).perimeter_position(), 1.0);
        assert_eq!(Seat::at(TableSide::Left, Fraction::new(1.0)).perimeter_position(), 1.0);
        assert_eq!(Seat::at(TableSide::Left, Fraction::new(0.0)).perimeter_position(), 2.0);
        assert_eq!(Seat::at(TableSide::Top, Fraction::new(0.0)).perimeter_position(), 2.0);
        assert_eq!(Seat::at(TableSide::Top, Fraction::new(1.0)).perimeter_position(), 3.0);
        assert_eq!(Seat::at(TableSide::Right, Fraction::new(0.0)).perimeter_position(), 3.0);
    }

    /// `next_in` skips empty sides and visits two seats on one side in
    /// order; `previous_in` is its exact inverse.
    #[test]
    fn next_in_visits_occupied_seats_only() {
        let roster = [
            Seat::center(TableSide::Bottom),
            Seat::center(TableSide::Right),
            Seat::at(TableSide::Top, Fraction::new(0.25)),
            Seat::at(TableSide::Top, Fraction::new(0.75)),
        ];
        let [a, b, c, d] = roster;
        // Clockwise A (bottom) -> C (top-left) -> D (top-right) -> B (right) -> A.
        assert_eq!(a.next_in(&roster), Some(c));
        assert_eq!(c.next_in(&roster), Some(d));
        assert_eq!(d.next_in(&roster), Some(b));
        assert_eq!(b.next_in(&roster), Some(a));
        // Counter-clockwise runs the tour backwards.
        assert_eq!(a.previous_in(&roster), Some(b));
        assert_eq!(c.previous_in(&roster), Some(a));
    }

    /// An unpinned seat resolves to `HALF` for ordering, like
    /// `placement` does for positioning.
    #[test]
    fn next_in_treats_unpinned_as_centered() {
        let roster = [Seat::new(TableSide::Bottom), Seat::new(TableSide::Top)];
        assert_eq!(Seat::center(TableSide::Bottom).next_in(&roster), Some(roster[1]));
    }

    /// No roster or only the own position means there is no next seat.
    #[test]
    fn next_in_without_other_seats_is_none() {
        let me = Seat::center(TableSide::Bottom);
        assert_eq!(me.next_in(&[]), None);
        assert_eq!(me.next_in(&[me]), None);
    }

    /// A seat outside the roster still finds the next occupied seat
    /// after its own perimeter position.
    #[test]
    fn next_in_from_unlisted_position() {
        let roster = [Seat::center(TableSide::Right), Seat::center(TableSide::Bottom)];
        assert_eq!(Seat::center(TableSide::Top).next_in(&roster), Some(roster[0]));
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
