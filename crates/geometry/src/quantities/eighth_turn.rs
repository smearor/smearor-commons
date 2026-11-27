use crate::RotationDirection;
use crate::RotationRad;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// An eighth-of-a-turn rotation (`0`, `45`, `90`, `135`, `180`,
/// `225`, `270`, `315` degrees) - the snapped orientations of
/// regular octagons, compass roses and 8-item radial layouts.
///
/// The direction is **clockwise as seen by the viewer**: `Deg45`
/// rotates the native frame's top edge 45° to the viewer's right.
/// Unlike [`RotationRad`] this is a discrete enum, not a continuous
/// angle - use it where only the eight snapped orientations exist.
/// Every eighth turn maps a regular octagon onto itself, so
/// rotating an octagonal element snaps in 45° steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub enum EighthTurn {
    /// No rotation - native orientation.
    #[default]
    Deg0,
    /// One eighth of a turn clockwise (45°) as seen by the viewer.
    Deg45,
    /// One quarter of a turn (90°).
    Deg90,
    /// Three eighths of a turn clockwise (135°).
    Deg135,
    /// Half turn (180°) - upside down.
    Deg180,
    /// Five eighths of a turn clockwise (225° = 135° counter-clockwise).
    Deg225,
    /// Three quarters of a turn clockwise (270° = 90° counter-clockwise).
    Deg270,
    /// Seven eighths of a turn clockwise (315° = 45° counter-clockwise).
    Deg315,
}

impl EighthTurn {
    /// All eighth turns in clockwise order.
    pub const ALL: [Self; 8] = [
        Self::Deg0,
        Self::Deg45,
        Self::Deg90,
        Self::Deg135,
        Self::Deg180,
        Self::Deg225,
        Self::Deg270,
        Self::Deg315,
    ];

    /// The eighth turn one step clockwise from `self`.
    pub fn clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg45,
            Self::Deg45 => Self::Deg90,
            Self::Deg90 => Self::Deg135,
            Self::Deg135 => Self::Deg180,
            Self::Deg180 => Self::Deg225,
            Self::Deg225 => Self::Deg270,
            Self::Deg270 => Self::Deg315,
            Self::Deg315 => Self::Deg0,
        }
    }

    /// The eighth turn one step counter-clockwise from `self`.
    pub fn counter_clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg315,
            Self::Deg45 => Self::Deg0,
            Self::Deg90 => Self::Deg45,
            Self::Deg135 => Self::Deg90,
            Self::Deg180 => Self::Deg135,
            Self::Deg225 => Self::Deg180,
            Self::Deg270 => Self::Deg225,
            Self::Deg315 => Self::Deg270,
        }
    }

    /// The eighth turn whose snap zone contains `rotation`.
    ///
    /// Snap gestures divide the circle into zones reaching 22.5°
    /// to either side of each turn's angle: `[0°, 22.5°)` for
    /// `Deg0`, `[22.5°, 67.5°)` for `Deg45`, `[67.5°, 112.5°)` for
    /// `Deg90`, `[112.5°, 157.5°)` for `Deg135`, `[157.5°, 202.5°)`
    /// for `Deg180`, `[202.5°, 247.5°)` for `Deg225`,
    /// `[247.5°, 292.5°)` for `Deg270` - the wrap-around range
    /// `292.5°..360°` stays engaged with `Deg315` rather than
    /// `Deg0`.
    pub fn snap_zone(rotation: RotationRad) -> Self {
        match rotation.normalized() {
            r if r < RotationRad::from_degrees(22.5) => Self::Deg0,
            r if r < RotationRad::from_degrees(67.5) => Self::Deg45,
            r if r < RotationRad::from_degrees(112.5) => Self::Deg90,
            r if r < RotationRad::from_degrees(157.5) => Self::Deg135,
            r if r < RotationRad::from_degrees(202.5) => Self::Deg180,
            r if r < RotationRad::from_degrees(247.5) => Self::Deg225,
            r if r < RotationRad::from_degrees(292.5) => Self::Deg270,
            _ => Self::Deg315,
        }
    }

    /// The turn a snap gesture starting at `rotation` settles on
    /// when dragged further in `direction`: one step from the turn
    /// whose [`snap_zone`](Self::snap_zone) contains `rotation`.
    ///
    /// A gesture beginning at `Deg0` snaps to `Deg45` when dragged
    /// clockwise and to `Deg315` counter-clockwise.
    pub fn snap_target(rotation: RotationRad, direction: RotationDirection) -> Self {
        let zone = Self::snap_zone(rotation);
        match direction {
            RotationDirection::Clockwise => zone.clockwise(),
            RotationDirection::CounterClockwise => zone.counter_clockwise(),
        }
    }

    /// The turn undoing this one - `Deg0`/`Deg180` are their own
    /// inverses, `Deg45`/`Deg315`, `Deg90`/`Deg270` and
    /// `Deg135`/`Deg225` swap.
    pub fn inverse(self) -> Self {
        match self {
            Self::Deg0 | Self::Deg180 => self,
            Self::Deg45 => Self::Deg315,
            Self::Deg90 => Self::Deg270,
            Self::Deg135 => Self::Deg225,
            Self::Deg225 => Self::Deg135,
            Self::Deg270 => Self::Deg90,
            Self::Deg315 => Self::Deg45,
        }
    }

    /// The rotation angle in degrees (`0.0`, `45.0`, ..., `315.0`).
    pub fn degrees(self) -> f64 {
        match self {
            Self::Deg0 => 0.0,
            Self::Deg45 => 45.0,
            Self::Deg90 => 90.0,
            Self::Deg135 => 135.0,
            Self::Deg180 => 180.0,
            Self::Deg225 => 225.0,
            Self::Deg270 => 270.0,
            Self::Deg315 => 315.0,
        }
    }

    /// The equivalent continuous [`RotationRad`].
    pub fn to_rotation_rad(self) -> RotationRad {
        RotationRad::from_degrees(self.degrees())
    }

    /// The snapped eighth turn when `angle` is exactly a multiple
    /// of 45 degrees in `[0°, 360°)` - `None` for every other
    /// direction. The [`RotationRad`]-typed counterpart of
    /// [`Self::from_degrees`].
    pub fn from_rotation_rad(angle: RotationRad) -> Option<Self> {
        Self::from_degrees(angle.to_degrees())
    }

    /// The snapped eighth turn for a degree value, or `None` when
    /// `degrees` is not one of `0`, `45`, `90`, `135`, `180`, `225`,
    /// `270`, `315`.
    pub fn from_degrees(degrees: f64) -> Option<Self> {
        match degrees {
            0.0 => Some(Self::Deg0),
            45.0 => Some(Self::Deg45),
            90.0 => Some(Self::Deg90),
            135.0 => Some(Self::Deg135),
            180.0 => Some(Self::Deg180),
            225.0 => Some(Self::Deg225),
            270.0 => Some(Self::Deg270),
            315.0 => Some(Self::Deg315),
            _ => None,
        }
    }
}

impl fmt::Display for EighthTurn {
    /// The degrees spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees() as u16)
    }
}

impl FromStr for EighthTurn {
    type Err = String;

    /// Accepts the eighth-turn values `0`, `45`, `90`, `135`,
    /// `180`, `225`, `270` or `315`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Deg0),
            "45" => Ok(Self::Deg45),
            "90" => Ok(Self::Deg90),
            "135" => Ok(Self::Deg135),
            "180" => Ok(Self::Deg180),
            "225" => Ok(Self::Deg225),
            "270" => Ok(Self::Deg270),
            "315" => Ok(Self::Deg315),
            _ => Err(format!("invalid eighth turn '{s}' - use a multiple of 45 in [0, 360)")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EighthTurn;
    use crate::RotationDirection;
    use crate::RotationRad;

    /// Clockwise and counter-clockwise steps undo each other on
    /// every turn.
    #[test]
    fn clockwise_counter_clockwise_cycle() {
        for turn in EighthTurn::ALL {
            assert_eq!(turn.clockwise().counter_clockwise(), turn);
            assert_eq!(turn.counter_clockwise().clockwise(), turn);
        }
    }

    /// `from_degrees` snaps only the eight exact values.
    #[test]
    fn from_degrees_snaps_only_exact() {
        assert_eq!(EighthTurn::from_degrees(0.0), Some(EighthTurn::Deg0));
        assert_eq!(EighthTurn::from_degrees(45.0), Some(EighthTurn::Deg45));
        assert_eq!(EighthTurn::from_degrees(315.0), Some(EighthTurn::Deg315));
        assert_eq!(EighthTurn::from_degrees(60.0), None);
        assert_eq!(EighthTurn::from_degrees(-45.0), None);
    }

    /// `inverse` maps each turn to the opposite rotation.
    #[test]
    fn inverse_complements_to_a_full_turn() {
        for turn in EighthTurn::ALL {
            let sum = turn.degrees() + turn.inverse().degrees();
            assert!(sum == 0.0 || sum == 360.0, "{turn:?} inverse {sum}");
        }
    }

    /// `to_rotation_rad` matches the degree value.
    #[test]
    fn to_rotation_rad_matches_degrees() {
        assert!((EighthTurn::Deg135.to_rotation_rad().to_degrees() - 135.0).abs() < 1e-9);
    }

    /// Snap zones cover ±22.5° around each turn, with the wrap
    /// range `292.5°..360°` staying engaged with `Deg315`.
    #[test]
    fn snap_zone_covers_the_circle() {
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(0.0)), EighthTurn::Deg0);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(22.4)), EighthTurn::Deg0);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(22.5)), EighthTurn::Deg45);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(67.5)), EighthTurn::Deg90);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(112.5)), EighthTurn::Deg135);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(157.5)), EighthTurn::Deg180);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(202.5)), EighthTurn::Deg225);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(247.5)), EighthTurn::Deg270);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(292.5)), EighthTurn::Deg315);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(359.9)), EighthTurn::Deg315);
        assert_eq!(EighthTurn::snap_zone(RotationRad::from_degrees(-10.0)), EighthTurn::Deg315);
    }

    /// The snap target is one step in the drag direction from the
    /// engaged zone's turn.
    #[test]
    fn snap_target_steps_from_the_engaged_zone() {
        let cw = RotationDirection::Clockwise;
        let ccw = RotationDirection::CounterClockwise;

        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(0.0), cw), EighthTurn::Deg45);
        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(0.0), ccw), EighthTurn::Deg315);
        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(100.0), cw), EighthTurn::Deg135);
        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(100.0), ccw), EighthTurn::Deg45);
        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(300.0), cw), EighthTurn::Deg0);
        assert_eq!(EighthTurn::snap_target(RotationRad::from_degrees(360.0), cw), EighthTurn::Deg45);
    }
}
