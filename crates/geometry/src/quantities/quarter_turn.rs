use crate::RotationRad;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// A right-angle rotation (`0`, `90`, `180`, `270` degrees) - the
/// snapped orientations of displays, boards and rotated widgets.
///
/// The direction is **clockwise as seen by the viewer**: `Deg90`
/// rotates the native frame's top edge to the viewer's right.
/// Unlike [`RotationRad`] this is a discrete enum, not a continuous
/// angle - use it where only the four snapped orientations exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
pub enum QuarterTurn {
    /// No rotation - native orientation.
    #[default]
    Deg0,
    /// Quarter turn clockwise (90°) as seen by the viewer.
    Deg90,
    /// Half turn (180°) - upside down.
    Deg180,
    /// Three-quarter turn clockwise (270° = 90° counter-clockwise).
    Deg270,
}

impl QuarterTurn {
    /// All quarter turns in clockwise order.
    pub const ALL: [Self; 4] = [Self::Deg0, Self::Deg90, Self::Deg180, Self::Deg270];

    /// The quarter turn one step clockwise from `self`.
    pub fn clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg90,
            Self::Deg90 => Self::Deg180,
            Self::Deg180 => Self::Deg270,
            Self::Deg270 => Self::Deg0,
        }
    }

    /// The quarter turn one step counter-clockwise from `self`.
    pub fn counter_clockwise(self) -> Self {
        match self {
            Self::Deg0 => Self::Deg270,
            Self::Deg90 => Self::Deg0,
            Self::Deg180 => Self::Deg90,
            Self::Deg270 => Self::Deg180,
        }
    }

    /// The turn undoing this one - `Deg0`/`Deg180` are their own
    /// inverses, the two quarter turns swap.
    pub fn inverse(self) -> Self {
        match self {
            Self::Deg0 | Self::Deg180 => self,
            Self::Deg90 => Self::Deg270,
            Self::Deg270 => Self::Deg90,
        }
    }

    /// Whether the turn swaps width and height (`Deg90`/`Deg270`).
    pub fn swaps_axes(self) -> bool {
        matches!(self, Self::Deg90 | Self::Deg270)
    }

    /// The rotation angle in degrees (`0.0`, `90.0`, `180.0`, `270.0`).
    pub fn degrees(self) -> f64 {
        match self {
            Self::Deg0 => 0.0,
            Self::Deg90 => 90.0,
            Self::Deg180 => 180.0,
            Self::Deg270 => 270.0,
        }
    }

    /// The equivalent continuous [`RotationRad`].
    pub fn to_rotation_rad(self) -> RotationRad {
        RotationRad::from_degrees(self.degrees())
    }

    /// The snapped quarter turn for a degree value, or `None` when
    /// `degrees` is not one of `0`, `90`, `180`, `270`.
    pub fn from_degrees(degrees: f64) -> Option<Self> {
        match degrees {
            0.0 => Some(Self::Deg0),
            90.0 => Some(Self::Deg90),
            180.0 => Some(Self::Deg180),
            270.0 => Some(Self::Deg270),
            _ => None,
        }
    }

    /// Rotates a position inside `bounds` into the turned orientation.
    ///
    /// The bounds swap when the turn [`Self::swaps_axes`] - the caller
    /// uses the returned position against `(bounds.y, bounds.x)` in
    /// that case.
    pub fn apply<U>(self, pos: Vec2<U>, bounds: Vec2<U>) -> Vec2<U> {
        match self {
            Self::Deg0 => pos,
            Self::Deg90 => Vec2::new(pos.y, bounds.x - pos.x),
            Self::Deg180 => Vec2::new(bounds.x - pos.x, bounds.y - pos.y),
            Self::Deg270 => Vec2::new(bounds.y - pos.y, pos.x),
        }
    }
}

impl fmt::Display for QuarterTurn {
    /// The degrees spelling accepted by [`FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees() as u16)
    }
}

impl FromStr for QuarterTurn {
    type Err = String;

    /// Accepts the quarter-turn values `0`, `90`, `180` or `270`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Deg0),
            "90" => Ok(Self::Deg90),
            "180" => Ok(Self::Deg180),
            "270" => Ok(Self::Deg270),
            _ => Err(format!("invalid quarter turn '{s}' - use 0, 90, 180 or 270")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QuarterTurn;
    use crate::Millimeters;
    use crate::Vec2;

    /// All four turns rotate a position inside its bounds correctly.
    #[test]
    fn quarter_turns_rotate_inside_bounds() {
        let bounds = Vec2::<Millimeters>::new(400.0, 300.0);
        let pos = Vec2::<Millimeters>::new(100.0, 50.0);

        let deg90 = QuarterTurn::Deg90.apply(pos, bounds);
        assert!((deg90.x - 50.0).abs() < 1e-9, "x becomes y");
        assert!((deg90.y - 300.0).abs() < 1e-9, "y becomes w - x");

        let deg180 = QuarterTurn::Deg180.apply(pos, bounds);
        assert!((deg180.x - 300.0).abs() < 1e-9);
        assert!((deg180.y - 250.0).abs() < 1e-9);

        let deg270 = QuarterTurn::Deg270.apply(pos, bounds);
        assert!((deg270.x - 250.0).abs() < 1e-9, "x becomes h - y");
        assert!((deg270.y - 100.0).abs() < 1e-9, "y becomes x");
    }

    /// `swaps_axes` is true exactly for the two odd turns.
    #[test]
    fn quarter_turns_swap_axes() {
        assert!(!QuarterTurn::Deg0.swaps_axes());
        assert!(QuarterTurn::Deg90.swaps_axes());
        assert!(!QuarterTurn::Deg180.swaps_axes());
        assert!(QuarterTurn::Deg270.swaps_axes());
    }

    /// `inverse` applied twice returns the original turn, and a turn
    /// plus its inverse always completes a full circle (or none).
    #[test]
    fn inverse_is_involution() {
        for turn in QuarterTurn::ALL {
            assert_eq!(turn.inverse().inverse(), turn);
            let sum = turn.degrees() + turn.inverse().degrees();
            assert!(sum == 0.0 || sum == 360.0);
        }
    }

    /// `clockwise`/`counter_clockwise` are mutual inverses over the cycle.
    #[test]
    fn clockwise_counter_clockwise_cycle() {
        for turn in QuarterTurn::ALL {
            assert_eq!(turn.clockwise().counter_clockwise(), turn);
            assert_eq!(turn.counter_clockwise().clockwise(), turn);
        }
    }

    /// `from_degrees` snaps only the four exact values.
    #[test]
    fn from_degrees_snaps_only_exact() {
        assert_eq!(QuarterTurn::from_degrees(0.0), Some(QuarterTurn::Deg0));
        assert_eq!(QuarterTurn::from_degrees(90.0), Some(QuarterTurn::Deg90));
        assert_eq!(QuarterTurn::from_degrees(45.0), None);
        assert_eq!(QuarterTurn::from_degrees(-90.0), None);
    }

    /// `to_rotation_rad` matches the degree value.
    #[test]
    fn to_rotation_rad_matches_degrees() {
        assert!((QuarterTurn::Deg90.to_rotation_rad().to_degrees() - 90.0).abs() < 1e-9);
    }
}
