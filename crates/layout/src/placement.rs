use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::QuarterTurn;
use smearor_commons_geometry::Vec2;

/// The technical target of an arrangement: where an item's center
/// lands, how it is rotated and how large its footprint is.
///
/// Consumers map their domain items onto `Placement`s and back -
/// the type carries no semantics about what the item is. Generic
/// over the coordinate unit `U` (and, like [`Vec2`], the numeric
/// component type `T`, default `f64`) so pixel, millimeter and
/// erased coordinate spaces share the same algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Placement<U = (), T = f64> {
    /// Target center point inside the arranged bounds.
    pub center: Vec2<U, T>,
    /// Reading orientation, snapped to a quarter turn.
    pub rotation: QuarterTurn,
    /// Footprint extent (rotated bounding box of the item),
    /// passed through unchanged by the arrangement.
    pub size: Vec2<U, T>,
}

impl<U, T: Copy> Placement<U, T> {
    /// Creates a placement at `center` facing `rotation` with
    /// footprint `size`.
    pub fn new(center: Vec2<U, T>, rotation: QuarterTurn, size: Vec2<U, T>) -> Self {
        Self { center, rotation, size }
    }
}

#[cfg(test)]
mod tests {
    use super::Placement;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::Vec2;

    /// A placement serializes and deserializes field-for-field.
    #[test]
    fn serde_round_trip() {
        let placement = Placement::<Pixels>::new(Vec2::new(120.0, 80.0), QuarterTurn::Deg90, Vec2::new(200.0, 100.0));
        let json = serde_json::to_string(&placement).expect("serialize");
        assert_eq!(serde_json::from_str::<Placement<Pixels>>(&json).expect("deserialize"), placement);
    }
}
