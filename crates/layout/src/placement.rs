use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::Vec2;

/// The technical target of an arrangement: where an item's center
/// lands, how it is rotated and how large its footprint is.
///
/// Consumers map their domain items onto `Placement`s and back -
/// the type carries no semantics about what the item is. Generic
/// over the coordinate unit `U` (and, like [`Vec2`], the numeric
/// component type `T`, default `f64`) so pixel, millimeter and
/// erased coordinate spaces share the same algorithms.
#[derive(Debug, Serialize, Deserialize)]
pub struct Placement<U = (), T = f64> {
    /// Target center point inside the arranged bounds.
    pub center: Vec2<U, T>,
    /// Reading orientation of the placed item. Strategies that
    /// preserve input orientation pass the item's [`Rotation`]
    /// through; seat strategies set a quarter turn.
    pub rotation: Rotation,
    /// Footprint extent of the placed item - the item's bounding
    /// box as displayed at `rotation`.
    pub size: Vec2<U, T>,
}

/// Field-wise equality - the unit marker `U` carries no value.
impl<U, T: PartialEq> PartialEq for Placement<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.rotation == other.rotation && self.size == other.size
    }
}

impl<U, T: Copy> Copy for Placement<U, T> {}

impl<U, T: Clone> Clone for Placement<U, T> {
    fn clone(&self) -> Self {
        Self {
            center: self.center.clone(),
            rotation: self.rotation,
            size: self.size.clone(),
        }
    }
}

/// The footprint `size` gains when `rotation` is replaced by `turn`:
/// quarter turns swap the axes exactly; a `Free` source rotation
/// cannot be inverted from its bounding box, so its footprint is
/// approximated as the unrotated extent.
///
/// Consumers use it to compute [`Placement::size`] when an item
/// changes orientation (e.g. [`crate::Seat::placement`]).
pub fn footprint_for<U>(size: Vec2<U>, rotation: Rotation, turn: QuarterTurn) -> Vec2<U> {
    let unswapped = match rotation {
        Rotation::QuarterTurn(turn) if turn.swaps_axes() => Vec2::new(size.y, size.x),
        _ => size,
    };
    if turn.swaps_axes() { Vec2::new(unswapped.y, unswapped.x) } else { unswapped }
}

impl<U, T: Copy> Placement<U, T> {
    /// Creates a placement at `center` facing `rotation` with
    /// footprint `size`.
    pub fn new(center: Vec2<U, T>, rotation: Rotation, size: Vec2<U, T>) -> Self {
        Self { center, rotation, size }
    }
}

#[cfg(test)]
mod tests {
    use super::Placement;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::Vec2;

    /// A placement serializes and deserializes field-for-field.
    #[test]
    fn serde_round_trip() {
        let placement = Placement::<Pixels>::new(Vec2::new(120.0, 80.0), Rotation::from_degrees(90.0), Vec2::new(200.0, 100.0));
        let json = serde_json::to_string(&placement).expect("serialize");
        assert_eq!(serde_json::from_str::<Placement<Pixels>>(&json).expect("deserialize"), placement);
    }

    /// `footprint_for` swaps axes exactly for quarter-turn changes.
    #[test]
    fn footprint_swaps_on_odd_turns() {
        use crate::footprint_for;
        use smearor_commons_geometry::QuarterTurn;
        use smearor_commons_geometry::RotationRad;
        let size = Vec2::<Pixels>::new(100.0, 50.0);
        assert_eq!(footprint_for(size, Rotation::ZERO, QuarterTurn::Deg90), Vec2::new(50.0, 100.0));
        assert_eq!(footprint_for(size, Rotation::ZERO, QuarterTurn::Deg180), Vec2::new(100.0, 50.0));
        // Deg90 source -> Deg180 target: un-swap then no swap.
        assert_eq!(
            footprint_for(Vec2::<Pixels>::new(50.0, 100.0), Rotation::QuarterTurn(QuarterTurn::Deg90), QuarterTurn::Deg180),
            Vec2::new(100.0, 50.0)
        );
        // Free rotation approximates as its own footprint.
        assert_eq!(footprint_for(size, Rotation::Free(RotationRad::from_degrees(42.0)), QuarterTurn::Deg0), size);
    }
}
