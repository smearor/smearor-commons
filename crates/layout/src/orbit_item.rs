use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;

/// One item handed to [`crate::OrbitLayout`]: footprint, desired
/// reading orientation and an optional anchor direction on the orbit.
///
/// `anchor` is the technical projection of an "anchored" item: `Some`
/// pins the item to that absolute direction on the orbit circle,
/// `None` leaves it flexible - the strategy distributes flexible
/// items into the free angular space between anchors. What the item
/// *is* stays with the consumer, same as [`crate::LayoutItem`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrbitItem<U = ()> {
    /// Footprint extent of the item.
    pub size: Vec2<U>,
    /// Desired reading orientation - passed through to the
    /// [`crate::Placement`] unchanged. Whether an item co-rotates
    /// with its host, stays upright or counter-rotates is the
    /// consumer's semantic; the strategy does not interpret it.
    pub rotation: Rotation,
    /// Fixed anchor direction on the orbit; `None` = flexible item
    /// distributed by the strategy.
    pub anchor: Option<RotationRad>,
}

impl<U> OrbitItem<U> {
    /// Creates a flexible item (no anchor) with the given footprint
    /// and rotation.
    pub fn new(size: Vec2<U>, rotation: Rotation) -> Self {
        Self { size, rotation, anchor: None }
    }

    /// The item pinned to `anchor` on the orbit.
    pub fn with_anchor(self, anchor: RotationRad) -> Self {
        Self { anchor: Some(anchor), ..self }
    }
}
