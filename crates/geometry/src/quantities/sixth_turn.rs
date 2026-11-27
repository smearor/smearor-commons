use crate::RotationDirection;
use crate::RotationRad;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// A sixth-of-a-turn rotation (`0`, `60`, `120`, `180`, `240`, `300`
/// degrees) - the snapped orientations of regular hexagons and
/// hexagonal grids.
///
/// The direction is **clockwise as seen by the viewer**: `Deg60`
/// rotates the native frame's top edge 60° to the viewer's right.
/// Unlike [`RotationRad`] this is a discrete enum, not a continuous
/// angle - use it where only the six snapped orientations exist.
/// Every sixth turn maps a regular hexagon onto itself, so rotating
/// a hexagonal element snaps in 60° steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub enum SixthTurn {
    /// No rotation - native orientation.
    #[default]
    Deg0,
    /// One sixth of a turn clockwise (60°) as seen by the viewer.
    Deg60,
    /// One third of a turn (120°).
    Deg120,
    /// Half turn (180°) - upside down.
    Deg180,
    /// Two thirds of a turn clockwise (240° = 120° counter-clockwise).
    Deg240,
    /// Five sixths of a turn clockwise (300° = 60° counter-clockwise).
    Deg300,
}

impl SixthTurn {
    /// All sixth turns in clockwise order.
    pub const ALL: [Self; 6] = [Self::Deg0, Self::Deg60, Self::Deg120, Self::Deg180, Self::Deg240, Self::Deg300];

    /// The sixth turn one step clockwise from `self`.
    pub fn clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg60,
            Self::Deg60 => Self::Deg120,
            Self::Deg120 => Self::Deg180,
            Self::Deg180 => Self::Deg240,
            Self::Deg240 => Self::Deg300,
            Self::Deg300 => Self::Deg0,
        }
    }

    /// The sixth turn one step counter-clockwise from `self`.
    pub fn counter_clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg300,
            Self::Deg60 => Self::Deg0,
            Self::Deg120 => Self::Deg60,
            Self::Deg180 => Self::Deg120,
            Self::Deg240 => Self::Deg180,
            Self::Deg300 => Self::Deg240,
        }
    }

    /// The sixth turn whose snap zone contains `rotation`.
    ///
    /// Snap gestures divide the circle into zones reaching 30°
    /// to either side of each turn's angle: `[0°, 30°)` for `Deg0`,
    /// `[30°, 90°)` for `Deg60`, `[90°, 150°)` for `Deg120`,
    /// `[150°, 210°)` for `Deg180`, `[210°, 270°)` for `Deg240` -
    /// the wrap-around range `330°..360°` stays engaged with
    /// `Deg300` rather than `Deg0`.
    pub fn snap_zone(rotation: RotationRad) -> Self {
        match rotation.normalized() {
            r if r < RotationRad::from_degrees(30.0) => Self::Deg0,
            r if r < RotationRad::from_degrees(90.0) => Self::Deg60,
            r if r < RotationRad::from_degrees(150.0) => Self::Deg120,
            r if r < RotationRad::from_degrees(210.0) => Self::Deg180,
            r if r < RotationRad::from_degrees(270.0) => Self::Deg240,
            _ => Self::Deg300,
        }
    }

    /// The turn a snap gesture starting at `rotation` settles on
    /// when dragged further in `direction`: one step from the turn
    /// whose [`snap_zone`](Self::snap_zone) contains `rotation`.
    ///
    /// A gesture beginning at `Deg0` snaps to `Deg60` when dragged
    /// clockwise and to `Deg300` counter-clockwise.
    pub fn snap_target(rotation: RotationRad, direction: RotationDirection) -> Self {
        let zone = Self::snap_zone(rotation);
        match direction {
            RotationDirection::Clockwise => zone.clockwise(),
            RotationDirection::CounterClockwise => zone.counter_clockwise(),
        }
    }

    /// The turn undoing this one - `Deg0`/`Deg180` are their own
    /// inverses, `Deg60`/`Deg300` and `Deg120`/`Deg240` swap.
    pub fn inverse(self) -> Self {
        match self {
            Self::Deg0 | Self::Deg180 => self,
            Self::Deg60 => Self::Deg300,
            Self::Deg120 => Self::Deg240,
            Self::Deg240 => Self::Deg120,
            Self::Deg300 => Self::Deg60,
        }
    }

    /// The rotation angle in degrees (`0.0`, `60.0`, ..., `300.0`).
    pub fn degrees(self) -> f64 {
        match self {
            Self::Deg0 => 0.0,
            Self::Deg60 => 60.0,
            Self::Deg120 => 120.0,
            Self::Deg180 => 180.0,
            Self::Deg240 => 240.0,
            Self::Deg300 => 300.0,
        }
    }

    /// The equivalent continuous [`RotationRad`].
    pub fn to_rotation_rad(self) -> RotationRad {
        RotationRad::from_degrees(self.degrees())
    }

    /// The snapped sixth turn when `angle` is exactly a multiple
    /// of 60 degrees in `[0°, 360°)` - `None` for every other
    /// direction. The [`RotationRad`]-typed counterpart of
    /// [`Self::from_degrees`].
    pub fn from_rotation_rad(angle: RotationRad) -> Option<Self> {
        Self::from_degrees(angle.to_degrees())
    }

    /// The snapped sixth turn for a degree value, or `None` when
    /// `degrees` is not one of `0`, `60`, `120`, `180`, `240`, `300`.
    pub fn from_degrees(degrees: f64) -> Option<Self> {
        match degrees {
            0.0 => Some(Self::Deg0),
            60.0 => Some(Self::Deg60),
            120.0 => Some(Self::Deg120),
            180.0 => Some(Self::Deg180),
            240.0 => Some(Self::Deg240),
            300.0 => Some(Self::Deg300),
            _ => None,
        }
    }
}

impl fmt::Display for SixthTurn {
    /// The degrees spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees() as u16)
    }
}

impl FromStr for SixthTurn {
    type Err = String;

    /// Accepts the sixth-turn values `0`, `60`, `120`, `180`,
    /// `240` or `300`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Deg0),
            "60" => Ok(Self::Deg60),
            "120" => Ok(Self::Deg120),
            "180" => Ok(Self::Deg180),
            "240" => Ok(Self::Deg240),
            "300" => Ok(Self::Deg300),
            _ => Err(format!("invalid sixth turn '{s}' - use a multiple of 60 in [0, 360)")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SixthTurn;
    use crate::RotationDirection;
    use crate::RotationRad;

    /// Clockwise and counter-clockwise steps undo each other on
    /// every turn.
    #[test]
    fn clockwise_counter_clockwise_cycle() {
        for turn in SixthTurn::ALL {
            assert_eq!(turn.clockwise().counter_clockwise(), turn);
            assert_eq!(turn.counter_clockwise().clockwise(), turn);
        }
    }

    /// `from_degrees` snaps only the six exact values.
    #[test]
    fn from_degrees_snaps_only_exact() {
        assert_eq!(SixthTurn::from_degrees(0.0), Some(SixthTurn::Deg0));
        assert_eq!(SixthTurn::from_degrees(60.0), Some(SixthTurn::Deg60));
        assert_eq!(SixthTurn::from_degrees(300.0), Some(SixthTurn::Deg300));
        assert_eq!(SixthTurn::from_degrees(30.0), None);
        assert_eq!(SixthTurn::from_degrees(-60.0), None);
    }

    /// `inverse` maps each turn to the opposite rotation.
    #[test]
    fn inverse_complements_to_a_full_turn() {
        for turn in SixthTurn::ALL {
            let sum = turn.degrees() + turn.inverse().degrees();
            assert!(sum == 0.0 || sum == 360.0, "{turn:?} inverse {sum}");
        }
    }

    /// `to_rotation_rad` matches the degree value.
    #[test]
    fn to_rotation_rad_matches_degrees() {
        assert!((SixthTurn::Deg120.to_rotation_rad().to_degrees() - 120.0).abs() < 1e-9);
    }

    /// Snap zones cover ±30° around each turn, with the wrap range
    /// `330°..360°` staying engaged with `Deg300`.
    #[test]
    fn snap_zone_covers_the_circle() {
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(0.0)), SixthTurn::Deg0);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(29.9)), SixthTurn::Deg0);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(30.0)), SixthTurn::Deg60);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(89.9)), SixthTurn::Deg60);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(90.0)), SixthTurn::Deg120);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(150.0)), SixthTurn::Deg180);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(210.0)), SixthTurn::Deg240);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(270.0)), SixthTurn::Deg300);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(330.0)), SixthTurn::Deg300);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(359.9)), SixthTurn::Deg300);
        assert_eq!(SixthTurn::snap_zone(RotationRad::from_degrees(-10.0)), SixthTurn::Deg300);
    }

    /// The snap target is one step in the drag direction from the
    /// engaged zone's turn.
    #[test]
    fn snap_target_steps_from_the_engaged_zone() {
        let cw = RotationDirection::Clockwise;
        let ccw = RotationDirection::CounterClockwise;

        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(0.0), cw), SixthTurn::Deg60);
        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(0.0), ccw), SixthTurn::Deg300);
        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(100.0), cw), SixthTurn::Deg180);
        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(100.0), ccw), SixthTurn::Deg60);
        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(330.0), cw), SixthTurn::Deg0);
        assert_eq!(SixthTurn::snap_target(RotationRad::from_degrees(360.0), cw), SixthTurn::Deg60);
    }
}
