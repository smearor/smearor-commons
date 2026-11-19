use crate::AnchorWeights;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;
use std::fmt;
use std::str::FromStr;

/// One of the four edges of a rectangular surface - the technical
/// seat vocabulary of tabletop layouts.
///
/// A side says *where* a user sits (or where an item is readable from),
/// not *who* sits there - player and owner semantics stay in consumer
/// crates. The declaration order is clockwise as seen from above:
/// `Bottom -> Left -> Top -> Right` walks the edges the way a clock
/// hand passes 6 -> 9 -> 12 -> 3, and each step corresponds to a
/// `+90` degree (clockwise) reading rotation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
#[serde(rename_all = "lowercase")]
pub enum TableSide {
    /// Bottom edge - items rotated `Deg0` read correctly here.
    #[default]
    Bottom,
    /// Left edge - items rotated `Deg90` clockwise read correctly here.
    Left,
    /// Top edge - items rotated `Deg180` read correctly here.
    Top,
    /// Right edge - items rotated `Deg270` clockwise read correctly here.
    Right,
}

impl TableSide {
    /// All sides in clockwise order - the iteration list for
    /// round-robin distribution. Stepping logic lives in the
    /// exhaustive `match` of [`Self::clockwise`], not in this array.
    pub const ALL: [Self; 4] = [Self::Bottom, Self::Left, Self::Top, Self::Right];

    /// The side one step clockwise from `self` (`Bottom -> Left ->
    /// Top -> Right -> Bottom`). Exhaustive `match`: a new variant
    /// becomes a compile error instead of silently wrong indexing.
    pub fn clockwise(&self) -> Self {
        match self {
            Self::Bottom => Self::Left,
            Self::Left => Self::Top,
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
        }
    }

    /// The side one step counter-clockwise from `self`.
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Self::Bottom => Self::Right,
            Self::Right => Self::Top,
            Self::Top => Self::Left,
            Self::Left => Self::Bottom,
        }
    }

    /// The reading orientation of this side as a [`QuarterTurn`] -
    /// an item rotated this way has its bottom edge facing the side
    /// and reads correctly for a user sitting there.
    pub fn to_quarter_turn(self) -> QuarterTurn {
        match self {
            Self::Bottom => QuarterTurn::Deg0,
            Self::Left => QuarterTurn::Deg90,
            Self::Top => QuarterTurn::Deg180,
            Self::Right => QuarterTurn::Deg270,
        }
    }

    /// Unit direction pointing to the seated user's right along this
    /// edge, in screen coordinates (x right, y down).
    ///
    /// A user at the bottom edge faces up the board, so their right is
    /// +x; a user at the top edge faces down, so their right is -x;
    /// users at the left/right edges face right/left, so their right
    /// is +y/-y. The vector orients "place to my right" fanning of
    /// items queueing at a seat.
    pub fn right_direction<U>(&self) -> Vec2<U> {
        match self {
            Self::Bottom => Vec2::new(1.0, 0.0),
            Self::Top => Vec2::new(-1.0, 0.0),
            Self::Left => Vec2::new(0.0, 1.0),
            Self::Right => Vec2::new(0.0, -1.0),
        }
    }

    /// The nearest side to a (free) rotation - the quarter turn the
    /// angle snaps to, read as a side. Each side owns a 90-degree
    /// sector centered on its own rotation; on exact sector
    /// boundaries (45/135/225/315 degrees) the clockwise-next side
    /// wins.
    pub fn nearest(rotation: RotationRad) -> Self {
        let degrees = rotation.normalized().to_degrees();
        if !(45.0..315.0).contains(&degrees) {
            Self::Bottom
        } else if degrees < 135.0 {
            Self::Left
        } else if degrees < 225.0 {
            Self::Top
        } else {
            Self::Right
        }
    }

    /// The side whose edge is nearest to `position` inside `bounds`,
    /// by perpendicular distance to each of the four edges. Positions
    /// outside `bounds` yield negative distances, so the closest edge
    /// still wins; exact ties resolve to the first side in
    /// [`Self::ALL`] order (`Bottom` first).
    ///
    /// The positional counterpart of [`Self::nearest`], which snaps a
    /// rotation to a side: here a *location* picks its edge - e.g.
    /// deriving a [`crate::Seat`] from a marker's physical position.
    pub fn nearest_edge<U>(position: Vec2<U>, bounds: Rect<U>) -> Self {
        let mut best = Self::Bottom;
        let mut best_distance = f64::MAX;
        for (side, distance) in [
            (Self::Bottom, bounds.min.y + bounds.size.y - position.y),
            (Self::Left, position.x - bounds.min.x),
            (Self::Top, position.y - bounds.min.y),
            (Self::Right, bounds.min.x + bounds.size.x - position.x),
        ] {
            if distance < best_distance {
                best = side;
                best_distance = distance;
            }
        }
        best
    }

    /// The side's edge-center anchor inside bounds: `Bottom`
    /// anchors `BOTTOM_CENTER` - bottom edge, horizontally centered.
    pub fn anchor_weights(&self) -> AnchorWeights {
        match self {
            Self::Bottom => AnchorWeights::BOTTOM_CENTER,
            Self::Left => AnchorWeights::CENTER_LEFT,
            Self::Top => AnchorWeights::TOP_CENTER,
            Self::Right => AnchorWeights::CENTER_RIGHT,
        }
    }
}

/// The side as its reading [`QuarterTurn`].
impl From<TableSide> for QuarterTurn {
    fn from(side: TableSide) -> Self {
        side.to_quarter_turn()
    }
}

/// A [`QuarterTurn`] read as the side it faces.
impl From<QuarterTurn> for TableSide {
    fn from(turn: QuarterTurn) -> Self {
        match turn {
            QuarterTurn::Deg0 => Self::Bottom,
            QuarterTurn::Deg90 => Self::Left,
            QuarterTurn::Deg180 => Self::Top,
            QuarterTurn::Deg270 => Self::Right,
        }
    }
}

impl fmt::Display for TableSide {
    /// The lowercase spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Top => "top",
            Self::Right => "right",
        })
    }
}

impl FromStr for TableSide {
    type Err = String;

    /// Accepts `bottom`, `left`, `top` or `right` (case-insensitive).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bottom" => Ok(Self::Bottom),
            "left" => Ok(Self::Left),
            "top" => Ok(Self::Top),
            "right" => Ok(Self::Right),
            _ => Err(format!("invalid table side '{s}' - use bottom, left, top or right")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableSide;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::RotationRad;
    use smearor_commons_geometry::Vec2;

    /// Four clockwise steps return to the start and visit every side.
    #[test]
    fn clockwise_cycles_through_all_sides() {
        let mut side = TableSide::Bottom;
        let mut seen = Vec::new();
        for _ in 0..4 {
            seen.push(side);
            side = side.clockwise();
        }
        assert_eq!(side, TableSide::Bottom);
        assert_eq!(seen, TableSide::ALL.to_vec());
    }

    /// `clockwise`/`counter_clockwise` are mutual inverses.
    #[test]
    fn counter_clockwise_inverts_clockwise() {
        for side in TableSide::ALL {
            assert_eq!(side.clockwise().counter_clockwise(), side);
            assert_eq!(side.counter_clockwise().clockwise(), side);
        }
    }

    /// `to_quarter_turn` maps each side to its reading rotation, and
    /// `nearest` snaps that same rotation back to the side.
    #[test]
    fn quarter_turn_round_trips_through_nearest() {
        for side in TableSide::ALL {
            let turn = side.to_quarter_turn();
            assert_eq!(TableSide::from(turn), side);
            assert_eq!(TableSide::nearest(turn.to_rotation_rad()), side);
        }
    }

    /// `nearest` owns 90-degree sectors centered on each side's
    /// rotation; boundary angles go to the clockwise-next side.
    #[test]
    fn nearest_snaps_sectors() {
        let cases = [
            (0.0, TableSide::Bottom),
            (44.9, TableSide::Bottom),
            (45.0, TableSide::Left),
            (90.0, TableSide::Left),
            (134.9, TableSide::Left),
            (135.0, TableSide::Top),
            (180.0, TableSide::Top),
            (224.9, TableSide::Top),
            (225.0, TableSide::Right),
            (270.0, TableSide::Right),
            (314.9, TableSide::Right),
            (315.0, TableSide::Bottom),
            (359.9, TableSide::Bottom),
        ];
        for (degrees, expected) in cases {
            assert_eq!(TableSide::nearest(RotationRad::from_degrees(degrees)), expected, "{degrees} degrees");
        }
    }

    /// Negative and over-turned angles normalize before snapping.
    #[test]
    fn nearest_normalizes_free_angles() {
        assert_eq!(TableSide::nearest(RotationRad::from_degrees(-90.0)), TableSide::Right);
        assert_eq!(TableSide::nearest(RotationRad::from_degrees(450.0)), TableSide::Left);
    }

    /// `nearest_edge` picks the edge with the smallest perpendicular
    /// distance, for points on each side and for off-center points.
    #[test]
    fn nearest_edge_picks_closest_edge() {
        let bounds: Rect<Pixels> = Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(400.0, 300.0),
        };
        let cases = [
            (Vec2::<Pixels>::new(200.0, 280.0), TableSide::Bottom),
            (Vec2::<Pixels>::new(10.0, 150.0), TableSide::Left),
            (Vec2::<Pixels>::new(200.0, 15.0), TableSide::Top),
            (Vec2::<Pixels>::new(390.0, 150.0), TableSide::Right),
            // Off-center but still nearer the bottom edge than the sides.
            (Vec2::<Pixels>::new(390.0, 295.0), TableSide::Bottom),
            // Corner: closer to the right edge than to the bottom.
            (Vec2::<Pixels>::new(395.0, 290.0), TableSide::Right),
        ];
        for (position, expected) in cases {
            assert_eq!(TableSide::nearest_edge(position, bounds), expected, "{position:?}");
        }
    }

    /// Positions outside the bounds still snap to the nearest edge.
    #[test]
    fn nearest_edge_handles_outside_positions() {
        let bounds: Rect<Pixels> = Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(400.0, 300.0),
        };
        assert_eq!(TableSide::nearest_edge(Vec2::new(-10.0, 150.0), bounds), TableSide::Left);
        assert_eq!(TableSide::nearest_edge(Vec2::new(200.0, 400.0), bounds), TableSide::Bottom);
    }

    /// Exactly centered points tie on both axes - the first side in
    /// `ALL` order (`Bottom`) wins.
    #[test]
    fn nearest_edge_tie_goes_to_first_in_order() {
        let square: Rect<Pixels> = Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        };
        assert_eq!(TableSide::nearest_edge(Vec2::new(50.0, 50.0), square), TableSide::Bottom);
    }

    /// Each side anchors the correct edge, centered on the other axis.
    #[test]
    fn anchor_weights_point_at_the_edge() {
        use crate::AnchorWeights;
        assert_eq!(TableSide::Bottom.anchor_weights(), AnchorWeights::BOTTOM_CENTER);
        assert_eq!(TableSide::Left.anchor_weights(), AnchorWeights::CENTER_LEFT);
        assert_eq!(TableSide::Top.anchor_weights(), AnchorWeights::TOP_CENTER);
        assert_eq!(TableSide::Right.anchor_weights(), AnchorWeights::CENTER_RIGHT);
    }

    /// `Display`/`FromStr` spell the lowercase side names both ways.
    #[test]
    fn display_round_trips_through_parse() {
        for side in TableSide::ALL {
            assert_eq!(side.to_string().parse::<TableSide>(), Ok(side));
        }
        assert!("front".parse::<TableSide>().is_err());
    }

    /// Serde uses the same lowercase wire names.
    #[test]
    fn serde_round_trip() {
        for side in TableSide::ALL {
            let json = serde_json::to_string(&side).expect("serialize");
            assert_eq!(json, format!("\"{side}\""));
            assert_eq!(serde_json::from_str::<TableSide>(&json).expect("deserialize"), side);
        }
    }

    #[cfg(feature = "zbus")]
    #[test]
    fn zvariant_round_trip() {
        assert_eq!(<TableSide as zvariant::Type>::SIGNATURE.to_string(), "u");
        let context = zvariant::serialized::Context::new_dbus(zvariant::LE, 0);
        for side in TableSide::ALL {
            let bytes = zvariant::to_bytes(context, &side).unwrap();
            let decoded: TableSide = bytes.deserialize().unwrap().0;
            assert_eq!(decoded, side);
        }
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn json_schema_is_string_enum() {
        let schema = serde_json::to_value(schemars::schema_for!(TableSide)).unwrap();
        let text = schema.to_string();
        assert!(text.contains("bottom") && text.contains("top"));
    }
}
