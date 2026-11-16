use serde::Deserialize;
use serde::Serialize;
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
}
