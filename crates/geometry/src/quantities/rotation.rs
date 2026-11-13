use crate::QuarterTurn;
use crate::RotationRad;
use serde::Deserialize;
use serde::Serialize;

/// A rotation specification - either a snapped [`QuarterTurn`] or a
/// free continuous [`RotationRad`].
///
/// The enum makes "snapped vs. free" visible in the type: consumers
/// that distinguish the two (rotation widgets with snap presets,
/// configuration formats allowing both forms) model it directly
/// instead of re-deriving snappedness from raw angle values.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Rotation {
    /// A snapped right-angle rotation.
    QuarterTurn(QuarterTurn),
    /// A free continuous rotation.
    Free(RotationRad),
}

impl Rotation {
    /// The identity rotation (`0` degrees, snapped).
    pub const ZERO: Self = Self::QuarterTurn(QuarterTurn::Deg0);

    /// Creates a rotation from a degree value, snapping to
    /// [`QuarterTurn`] when the value is exactly `0`, `90`, `180`
    /// or `270` and falling back to [`Rotation::Free`] otherwise.
    ///
    /// Accepts any scalar that widens losslessly to `f64` (`f32`, `i32`,
    /// `u32`, ...); narrower-than-`f64` sources like `usize` still
    /// require an explicit cast.
    pub fn from_degrees(degrees: impl Into<f64>) -> Self {
        Self::quantized(RotationRad::from_degrees(degrees.into()))
    }

    /// Creates a rotation from a [`RotationRad`], snapping to
    /// [`QuarterTurn`] when the angle is exactly `0`, `90`, `180`
    /// or `270` degrees and falling back to [`Rotation::Free`]
    /// otherwise - the quantizing counterpart of the lossless
    /// `From<RotationRad>` conversion.
    pub fn quantized(angle: RotationRad) -> Self {
        match QuarterTurn::from_rotation_rad(angle) {
            Some(turn) => Self::QuarterTurn(turn),
            None => Self::Free(angle),
        }
    }

    /// The rotation angle in degrees.
    pub fn degrees(self) -> f64 {
        match self {
            Self::QuarterTurn(turn) => turn.degrees(),
            Self::Free(angle) => angle.to_degrees(),
        }
    }

    /// The equivalent continuous [`RotationRad`].
    pub fn to_rotation_rad(self) -> RotationRad {
        match self {
            Self::QuarterTurn(turn) => turn.to_rotation_rad(),
            Self::Free(angle) => angle,
        }
    }

    /// The snapped [`QuarterTurn`] when `self` is a
    /// [`Rotation::QuarterTurn`], `None` for free rotations.
    pub fn quarter_turn(self) -> Option<QuarterTurn> {
        match self {
            Self::QuarterTurn(turn) => Some(turn),
            Self::Free(_) => None,
        }
    }

    /// Whether `self` is a snapped [`Rotation::QuarterTurn`].
    pub fn is_snapped(self) -> bool {
        matches!(self, Self::QuarterTurn(_))
    }
}

impl Default for Rotation {
    /// The identity rotation [`Rotation::ZERO`].
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<QuarterTurn> for Rotation {
    fn from(turn: QuarterTurn) -> Self {
        Self::QuarterTurn(turn)
    }
}

impl From<RotationRad> for Rotation {
    /// A free rotation - snapped values stay [`Rotation::Free`];
    /// use [`Rotation::quantized`] or [`Rotation::from_degrees`]
    /// to quantize.
    fn from(angle: RotationRad) -> Self {
        Self::Free(angle)
    }
}

#[cfg(test)]
mod tests {
    use super::QuarterTurn;
    use super::Rotation;
    use super::RotationRad;

    /// `from_degrees` snaps the four exact values to `QuarterTurn`.
    #[test]
    fn from_degrees_snaps_exact_values() {
        for (degrees, turn) in [
            (0.0, QuarterTurn::Deg0),
            (90.0, QuarterTurn::Deg90),
            (180.0, QuarterTurn::Deg180),
            (270.0, QuarterTurn::Deg270),
        ] {
            assert_eq!(Rotation::from_degrees(degrees), Rotation::QuarterTurn(turn));
        }
    }

    /// `from_degrees` keeps non-snapped values as `Free` rotations.
    #[test]
    fn from_degrees_keeps_free_values() {
        let rotation = Rotation::from_degrees(45.0);
        assert!(!rotation.is_snapped());
        assert_eq!(rotation.quarter_turn(), None);
        assert!((rotation.degrees() - 45.0).abs() < 1e-9);
    }

    /// Both variants convert to the same continuous angle.
    #[test]
    fn to_rotation_rad_unifies_variants() {
        let snapped = Rotation::QuarterTurn(QuarterTurn::Deg90);
        let free = Rotation::Free(RotationRad::from_degrees(90.0));
        assert_eq!(snapped.to_rotation_rad(), free.to_rotation_rad());
    }

    /// `quarter_turn` reflects the declared variant, not the value.
    #[test]
    fn quarter_turn_reflects_declared_variant() {
        assert_eq!(Rotation::from(RotationRad::from_degrees(90.0)).quarter_turn(), None);
        assert_eq!(Rotation::QuarterTurn(QuarterTurn::Deg90).quarter_turn(), Some(QuarterTurn::Deg90));
    }

    /// `quantized` snaps exact quarter turns and keeps every other
    /// angle `Free` - the explicit path `From` deliberately avoids.
    #[test]
    fn quantized_snaps_exact_angles() {
        assert_eq!(Rotation::quantized(RotationRad::from_degrees(90.0)), Rotation::QuarterTurn(QuarterTurn::Deg90));
        assert_eq!(Rotation::quantized(RotationRad::from_degrees(45.0)), Rotation::Free(RotationRad::from_degrees(45.0)));
    }
}
