use crate::Degrees;
use crate::QuarterTurn;
use serde::Deserialize;
use serde::Serialize;

/// The direction of an angular change - which way a rotation
/// progresses around the circle.
///
/// Positive angle changes are [`Clockwise`](Self::Clockwise) in
/// Smearor's coordinate convention, matching the sign of GTK's
/// `GestureRotate::angle_delta` and of [`crate::RotationRad`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::Enum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub enum RotationDirection {
    /// Positive angle change - clockwise as seen by the viewer.
    Clockwise,
    /// Negative angle change - counter-clockwise.
    CounterClockwise,
}

impl RotationDirection {
    /// The direction of a signed angular `delta`: positive is
    /// clockwise, negative counter-clockwise, zero has no direction.
    pub fn of(delta: Degrees) -> Option<Self> {
        if delta > Degrees::ZERO {
            Some(Self::Clockwise)
        } else if delta < Degrees::ZERO {
            Some(Self::CounterClockwise)
        } else {
            None
        }
    }

    /// The quarter turn one step in this direction from `turn`.
    pub fn step(self, turn: QuarterTurn) -> QuarterTurn {
        match self {
            Self::Clockwise => turn.clockwise(),
            Self::CounterClockwise => turn.counter_clockwise(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RotationDirection;
    use crate::Degrees;
    use crate::QuarterTurn;

    /// Positive deltas are clockwise, negative counter-clockwise,
    /// zero has no direction.
    #[test]
    fn direction_of_signed_delta() {
        assert_eq!(RotationDirection::of(Degrees::new(12.5)), Some(RotationDirection::Clockwise));
        assert_eq!(RotationDirection::of(Degrees::new(-12.5)), Some(RotationDirection::CounterClockwise));
        assert_eq!(RotationDirection::of(Degrees::ZERO), None);
    }

    /// Stepping walks the quarter turns in the given direction.
    #[test]
    fn step_follows_direction() {
        assert_eq!(RotationDirection::Clockwise.step(QuarterTurn::Deg270), QuarterTurn::Deg0);
        assert_eq!(RotationDirection::CounterClockwise.step(QuarterTurn::Deg0), QuarterTurn::Deg270);
    }
}
