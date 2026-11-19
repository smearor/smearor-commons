use crate::Placement;
use crate::Seat;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;

/// A [`Seat`] plus the free radial orientation of the user sitting
/// there.
///
/// `Seat` alone carries the *organizational* alignment - which edge,
/// where along it. `orientation` carries the direction the seated
/// user faces as a free angle, not snapped to quarter turns, so a
/// seat in a table corner can face diagonally into the surface.
/// Placements made through this type set `Rotation::Free(orientation)`
/// instead of the edge's `QuarterTurn` - that is what makes corner
/// seats usable.
///
/// Technical like `Seat`: which *user* sits there stays consumer
/// domain. No `zvariant::Type` impl - a `SeatPose` travels D-Bus as a
/// `(TableSide, f64, f64)` triple with `NaN` for an unpinned offset
/// (see [`Seat`]).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::SimpleObject, async_graphql::InputObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "SeatPoseInput"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SeatPose {
    /// Organizational alignment - the table edge plus the position
    /// along it.
    pub seat: Seat,
    /// Radial orientation the seated user faces, in radians - free
    /// angle, not snapped to [`crate::TableSide`]'s quarter turns.
    pub orientation: RotationRad,
}

impl Default for SeatPose {
    /// The bottom edge, unpinned, facing `RotationRad::ZERO`.
    fn default() -> Self {
        Self {
            seat: Seat::default(),
            orientation: RotationRad::ZERO,
        }
    }
}

impl SeatPose {
    /// A pose for `seat` facing `orientation`.
    pub const fn new(seat: Seat, orientation: RotationRad) -> Self {
        Self { seat, orientation }
    }

    /// The whole pose derived from a physical marker pose: the edge
    /// alignment from [`Seat::from_position`], `orientation` passed
    /// through - the one-step derivation for tangible seat markers.
    pub fn from_position<U>(position: Vec2<U>, bounds: Rect<U>, orientation: RotationRad) -> Self {
        Self {
            seat: Seat::from_position(position, bounds),
            orientation,
        }
    }

    /// The pose one step clockwise (`side` steps, `offset` and
    /// `orientation` keep).
    pub fn clockwise(&self) -> Self {
        Self {
            seat: self.seat.clockwise(),
            ..*self
        }
    }

    /// The pose one step counter-clockwise.
    pub fn counter_clockwise(&self) -> Self {
        Self {
            seat: self.seat.counter_clockwise(),
            ..*self
        }
    }

    /// The seat's position along the table perimeter - delegates to
    /// [`Seat::perimeter_position`].
    pub fn perimeter_position(&self) -> f64 {
        self.seat.perimeter_position()
    }

    /// The occupied seat one step clockwise along the table perimeter -
    /// delegates to [`Seat::next_in`]. Roster walking works on
    /// `&[Seat]`; consumers holding `&[SeatPose]` map `pose.seat`.
    pub fn next_in(&self, roster: &[Seat]) -> Option<Seat> {
        self.seat.next_in(roster)
    }

    /// The occupied seat one step counter-clockwise - delegates to
    /// [`Seat::previous_in`].
    pub fn previous_in(&self, roster: &[Seat]) -> Option<Seat> {
        self.seat.previous_in(roster)
    }

    /// Places `footprint` in front of the seat like [`Seat::placement`],
    /// but rotated to `Rotation::Free(self.orientation)` instead of
    /// the edge's quarter turn - a corner seat's content reads
    /// diagonally to its user.
    pub fn placement<U>(&self, bounds: Rect<U>, footprint: Vec2<U>, margin: f64) -> Placement<U> {
        let mut placement = self.seat.placement(bounds, footprint, margin);
        placement.rotation = Rotation::Free(self.orientation);
        placement
    }

    /// Places `footprint` in the best free spot nearest the seat like
    /// [`Seat::fanned_placement`], with the same `Rotation::Free`
    /// override as [`Self::placement`].
    pub fn fanned_placement<U>(&self, bounds: Rect<U>, footprint: Vec2<U>, margin: f64, taken: &[(Vec2<U>, Vec2<U>)], gap: f64) -> Placement<U> {
        let mut placement = self.seat.fanned_placement(bounds, footprint, margin, taken, gap);
        placement.rotation = Rotation::Free(self.orientation);
        placement
    }
}

#[cfg(test)]
mod tests {
    use super::SeatPose;
    use crate::Seat;
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

    /// `from_position` derives the seat from the position and carries
    /// the free orientation through.
    #[test]
    fn from_position_derives_pose() {
        let pose = SeatPose::from_position(Vec2::new(100.0, 290.0), bounds(), RotationRad::from_degrees(30.0));
        assert_eq!(pose.seat, Seat::at(TableSide::Bottom, Fraction::new(0.25)));
        assert_eq!(pose.orientation, RotationRad::from_degrees(30.0));
    }

    /// A corner marker resolves to the nearer edge while the
    /// orientation keeps the diagonal angle.
    #[test]
    fn from_position_corner_keeps_diagonal() {
        let pose = SeatPose::from_position(Vec2::new(395.0, 290.0), bounds(), RotationRad::from_degrees(135.0));
        assert_eq!(pose.seat.side, TableSide::Right);
        assert_eq!(pose.orientation, RotationRad::from_degrees(135.0));
    }

    /// `placement` matches the seat's anchor but rotates freely.
    #[test]
    fn placement_rotates_freely() {
        let pose = SeatPose::new(Seat::center(TableSide::Bottom), RotationRad::from_degrees(45.0));
        let placement = pose.placement(bounds(), Vec2::new(100.0, 50.0), 24.0);
        assert_eq!(placement.rotation, Rotation::Free(RotationRad::from_degrees(45.0)));
        assert_eq!(placement.center, Seat::center(TableSide::Bottom).placement(bounds(), Vec2::new(100.0, 50.0), 24.0).center);
    }

    /// `fanned_placement` applies the same free-rotation override.
    #[test]
    fn fanned_placement_rotates_freely() {
        let pose = SeatPose::new(Seat::center(TableSide::Top), RotationRad::from_degrees(10.0));
        let placement = pose.fanned_placement(bounds(), Vec2::new(50.0, 50.0), 10.0, &[], 10.0);
        assert_eq!(placement.rotation, Rotation::Free(RotationRad::from_degrees(10.0)));
    }

    /// Stepping keeps offset and orientation while the side moves.
    #[test]
    fn clockwise_keeps_offset_and_orientation() {
        let pose = SeatPose::new(Seat::at(TableSide::Bottom, Fraction::new(0.25)), RotationRad::from_degrees(70.0));
        assert_eq!(pose.clockwise().seat, Seat::at(TableSide::Left, Fraction::new(0.25)));
        assert_eq!(pose.clockwise().orientation, RotationRad::from_degrees(70.0));
        assert_eq!(pose.counter_clockwise().seat, Seat::at(TableSide::Right, Fraction::new(0.25)));
    }

    /// Roster walking delegates to the inner seat.
    #[test]
    fn roster_walk_delegates() {
        let pose = SeatPose::new(Seat::center(TableSide::Bottom), RotationRad::ZERO);
        let roster = [Seat::center(TableSide::Top)];
        assert_eq!(pose.next_in(&roster), Some(Seat::center(TableSide::Top)));
        assert_eq!(pose.previous_in(&roster), Some(Seat::center(TableSide::Top)));
    }

    /// The default pose sits unpinned at the bottom edge facing zero.
    #[test]
    fn default_is_bottom_zero() {
        let pose = SeatPose::default();
        assert_eq!(pose.seat, Seat::default());
        assert_eq!(pose.orientation, RotationRad::ZERO);
        assert_eq!(pose.seat.side.to_quarter_turn(), QuarterTurn::Deg0);
    }

    /// Serde round-trips a pose.
    #[test]
    fn serde_round_trip() {
        let pose = SeatPose::new(Seat::at(TableSide::Left, Fraction::new(0.3)), RotationRad::from_degrees(42.5));
        let json = serde_json::to_string(&pose).expect("serialize");
        assert_eq!(serde_json::from_str::<SeatPose>(&json).expect("deserialize"), pose);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn json_schema_has_seat_and_orientation() {
        let schema = serde_json::to_value(schemars::schema_for!(SeatPose)).unwrap();
        let text = schema.to_string();
        assert!(text.contains("\"seat\"") && text.contains("\"orientation\""));
    }

    #[cfg(feature = "async-graphql")]
    #[test]
    fn seat_pose_in_schema_sdl() {
        struct Query;

        #[async_graphql::Object]
        impl Query {
            async fn seat_pose(&self) -> SeatPose {
                SeatPose::default()
            }
        }

        struct Mutation;

        #[async_graphql::Object]
        impl Mutation {
            async fn set_seat_pose(&self, _pose: SeatPose) -> bool {
                true
            }
        }

        let sdl = async_graphql::Schema::build(Query, Mutation, async_graphql::EmptySubscription).finish().sdl();
        assert!(sdl.contains("type SeatPose"));
        assert!(sdl.contains("input SeatPoseInput"));
    }
}
