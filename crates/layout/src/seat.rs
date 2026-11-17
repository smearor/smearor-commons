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

    /// Places the item in the best free spot nearest this seat: like
    /// [`Self::placement`], but `taken` items are treated as physical
    /// obstacles and the item slips into the closest free lateral gap
    /// instead of stacking on them.
    ///
    /// `taken` holds each occupying item as `(center, footprint)` -
    /// its live center and displayed extent; *every* item near the
    /// seat counts, regardless of any seat assignment. Spots are
    /// evaluated as lanes perpendicular to the edge: lane 0 is the
    /// edge lane at `margin` inset (the preferred position), deeper
    /// lanes stack behind it. Within a lane the free center with the
    /// smallest lateral distance wins, so slots freed by removed items
    /// are reused and a bigger item only takes a middle slot when it
    /// actually fits between its neighbors with `gap` clearance.
    /// Candidates are scored by seat-space distance
    /// `hypot(offset, ROW_JUMP_WEIGHT * depth)`, so a card may leave
    /// the edge lane when the queue there reaches notably farther
    /// from the seat than a spot behind the first row.
    pub fn fanned_placement<U>(&self, bounds: Rect<U>, footprint: Vec2<U>, margin: f64, taken: &[(Vec2<U>, Vec2<U>)], gap: f64) -> Placement<U> {
        /// Perpendicular depth counts this much when scoring lanes -
        /// staying on the edge is preferred unless the queue there is
        /// notably farther from the seat.
        const ROW_JUMP_WEIGHT: f64 = 2.0;

        let mut placement = self.placement(bounds, footprint, margin);
        let right: Vec2<U> = self.side.right_direction();
        let inward = Vec2::new(right.y, -right.x);
        let anchor = placement.center;
        let own_t = self.tangential_extent(footprint) / 2.0;
        let own_p = self.perpendicular_extent(footprint) / 2.0;
        let anchor_inset = margin + own_p;

        let (mut best_offset, mut best_depth, mut best_score) = (0.0, 0.0, f64::MAX);
        let mut lane_inset = anchor_inset;
        loop {
            let mut blocked = Vec::new();
            let mut inner_edge = lane_inset;
            for &(center, item_footprint) in taken {
                let item_inset = self.inset_of(center, bounds);
                let item_p = self.perpendicular_extent(item_footprint) / 2.0;
                if (item_inset - lane_inset).abs() >= item_p + own_p + gap {
                    continue;
                }
                inner_edge = inner_edge.max(item_inset + item_p);
                let delta = center - anchor;
                let offset = delta.x * right.x + delta.y * right.y;
                let item_t = self.tangential_extent(item_footprint) / 2.0;
                blocked.push((offset - item_t - gap - own_t, offset + item_t + gap + own_t));
            }
            let depth = lane_inset - anchor_inset;
            let offset = nearest_free_slot(&mut blocked);
            let weighted = ROW_JUMP_WEIGHT * depth;
            let score = (offset * offset + weighted * weighted).sqrt();
            if score < best_score {
                (best_offset, best_depth, best_score) = (offset, depth, score);
            }
            // An empty lane has no inner edge to stack the next lane
            // behind; deeper lanes only score worse once their depth
            // alone already beats the best candidate.
            if blocked.is_empty() || weighted >= best_score {
                break;
            }
            lane_inset = inner_edge + gap + own_p;
        }
        placement.center += right * best_offset + inward * best_depth;
        placement
    }

    /// The item's extent along this seat's edge axis - x for the
    /// bottom/top edges, y for the left/right edges.
    fn tangential_extent<U>(&self, footprint: Vec2<U>) -> f64 {
        match self.side {
            TableSide::Bottom | TableSide::Top => footprint.x,
            TableSide::Left | TableSide::Right => footprint.y,
        }
    }

    /// The item's extent perpendicular to this seat's edge - y for the
    /// bottom/top edges, x for the left/right edges.
    fn perpendicular_extent<U>(&self, footprint: Vec2<U>) -> f64 {
        match self.side {
            TableSide::Bottom | TableSide::Top => footprint.y,
            TableSide::Left | TableSide::Right => footprint.x,
        }
    }

    /// The distance of a center point inward from this seat's edge.
    fn inset_of<U>(&self, center: Vec2<U>, bounds: Rect<U>) -> f64 {
        match self.side {
            TableSide::Bottom => bounds.min.y + bounds.size.y - center.y,
            TableSide::Top => center.y - bounds.min.y,
            TableSide::Left => center.x - bounds.min.x,
            TableSide::Right => bounds.min.x + bounds.size.x - center.x,
        }
    }
}

/// The center coordinate on a 1D lane nearest `0` where `blocked`
/// intervals (in newcomer-center space, already expanded by gap and
/// own half-extent) leave room. Interval boundaries count as free -
/// landing exactly on a boundary means exactly `gap` clearance.
/// Prefers the positive direction on ties.
fn nearest_free_slot(blocked: &mut Vec<(f64, f64)>) -> f64 {
    blocked.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut merged: Vec<(f64, f64)> = Vec::with_capacity(blocked.len());
    for &(left, right) in blocked.iter() {
        match merged.last_mut() {
            Some((_, merged_right)) if left <= *merged_right => *merged_right = merged_right.max(right),
            _ => merged.push((left, right)),
        }
    }
    let is_free = |candidate: f64| merged.iter().all(|&(left, right)| candidate <= left || candidate >= right);
    std::iter::once(0.0)
        .chain(merged.iter().flat_map(|&(left, right)| [left, right]))
        .filter(|&candidate| is_free(candidate))
        .min_by(|left, right| {
            left.abs()
                .partial_cmp(&right.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(right.partial_cmp(left).unwrap_or(std::cmp::Ordering::Equal))
        })
        .unwrap_or(0.0)
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

    /// Items queueing at a seat fan out beside each other: the second
    /// to the seated user's right, the third to their left, each with
    /// `gap` clearance - nothing overlaps.
    #[test]
    fn fanned_placement_alternates_right_then_left() {
        let seat = Seat::center(TableSide::Bottom);
        let footprint = Vec2::new(100.0, 50.0);
        let margin = 10.0;
        // Bottom-center anchor: (200, 300 - 10 - 25) = (200, 265).
        let first = seat.fanned_placement(bounds(), footprint, margin, &[], margin);
        assert_eq!(first.center, Vec2::new(200.0, 265.0));

        let second = seat.fanned_placement(bounds(), footprint, margin, &[(first.center, footprint)], margin);
        // Right of the seated user is +x: 50 + 10 + 50 beyond the anchor.
        assert_eq!(second.center, Vec2::new(310.0, 265.0));

        let taken = [(first.center, footprint), (second.center, footprint)];
        let third = seat.fanned_placement(bounds(), footprint, margin, &taken, margin);
        assert_eq!(third.center, Vec2::new(90.0, 265.0));
    }

    /// "Right" is relative to the seated user: at the top edge they
    /// face down the board, so their right is screen-left (-x).
    #[test]
    fn fanned_placement_right_is_user_relative() {
        let seat = Seat::at(TableSide::Top, Fraction::new(0.25));
        let footprint = Vec2::new(100.0, 50.0);
        let first = seat.fanned_placement(bounds(), footprint, 10.0, &[], 10.0);
        // Top edge at offset 0.25: (400 * 0.25, 10 + 25) = (100, 35).
        assert_eq!(first.center, Vec2::new(100.0, 35.0));

        let second = seat.fanned_placement(bounds(), footprint, 10.0, &[(first.center, footprint)], 10.0);
        // The user's right is -x: 100 - (50 + 10 + 50) = -10.
        assert_eq!(second.center, Vec2::new(-10.0, 35.0));
    }

    /// Occupancy is physical: an item merely near the seat's lateral
    /// column but off the edge lane reserves no space - the newcomer
    /// still lands at the anchor.
    #[test]
    fn fanned_placement_ignores_items_off_the_lane() {
        let seat = Seat::center(TableSide::Bottom);
        let footprint = Vec2::new(100.0, 50.0);
        // Floating mid-table (inset 150, the lane sits at 35).
        let off_lane = (Vec2::new(200.0, 150.0), footprint);
        let placement = seat.fanned_placement(bounds(), footprint, 10.0, &[off_lane], 10.0);
        assert_eq!(placement.center, Vec2::new(200.0, 265.0));
    }

    /// Freed slots are reused: a gap between two items is filled at
    /// the position nearest the seat when the item fits with `gap`
    /// clearance on both sides.
    #[test]
    fn fanned_placement_reuses_freed_slot() {
        let seat = Seat::center(TableSide::Bottom);
        let footprint = Vec2::new(100.0, 50.0);
        let taken = [(Vec2::new(200.0, 265.0), footprint), (Vec2::new(510.0, 265.0), footprint)];
        // The free lane point nearest the anchor is +110.
        let placement = seat.fanned_placement(bounds(), footprint, 10.0, &taken, 10.0);
        assert_eq!(placement.center, Vec2::new(310.0, 265.0));
    }

    /// A wider item only takes a middle slot when it fits between the
    /// neighbors with `gap` clearance.
    #[test]
    fn fanned_placement_wide_item_checks_clearance() {
        let seat = Seat::center(TableSide::Bottom);
        let slim = Vec2::new(100.0, 50.0);
        let wide = Vec2::new(300.0, 50.0);
        // The gap between the two slim items is 210 wide - too narrow
        // for the wide item (300 + 2*10). The nearest free spot is
        // behind the row (depth 60, weighted 120 < 210 lateral).
        let narrow_gap = [(Vec2::new(200.0, 265.0), slim), (Vec2::new(510.0, 265.0), slim)];
        let placement = seat.fanned_placement(bounds(), wide, 10.0, &narrow_gap, 10.0);
        assert_eq!(placement.center, Vec2::new(200.0, 205.0));

        // With the right neighbor farther out the middle gap would
        // fit, but the lane behind the anchor is closer - unless that
        // lane is occupied too: then the clearance check decides and
        // the wide item takes the middle slot.
        let occupied_behind = (Vec2::new(200.0, 205.0), slim);
        let wide_gap = [(Vec2::new(200.0, 265.0), slim), (Vec2::new(710.0, 265.0), slim), occupied_behind];
        let placement = seat.fanned_placement(bounds(), wide, 10.0, &wide_gap, 10.0);
        assert_eq!(placement.center, Vec2::new(410.0, 265.0));
    }

    /// A long edge queue lets the card leave the edge lane: a spot
    /// behind the first row is closer to the seat than the far end.
    #[test]
    fn fanned_placement_escapes_behind_the_edge_row() {
        let seat = Seat::center(TableSide::Bottom);
        let footprint = Vec2::new(100.0, 50.0);
        let taken: Vec<(Vec2<Pixels>, Vec2<Pixels>)> = [0.0, 110.0, -110.0, 220.0, 330.0]
            .iter()
            .map(|offset| (Vec2::new(200.0 + offset, 265.0), footprint))
            .collect();
        // The edge candidate sits at o = -220; the lane behind the row
        // (depth 60, weighted 120) is closer to the seat.
        let placement = seat.fanned_placement(bounds(), footprint, 10.0, &taken, 10.0);
        assert_eq!(placement.center, Vec2::new(200.0, 205.0));
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
