use crate::Seat;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::Vec2;

/// One item handed to a [`crate::LayoutStrategy`]: footprint, current
/// reading orientation and optional seat assignment.
///
/// What the item *is* stays with the consumer - strategies see only
/// this technical triple. `size` is the rotated bounding box of the
/// item's displayed extent, so arrangements account for rotation
/// and scale without knowing either.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutItem<U, T = f64> {
    /// Footprint extent (rotated bounding box) of the item.
    pub size: Vec2<U, T>,
    /// Current reading orientation. Strategies may preserve it
    /// (e.g. [`crate::FlowLayout`]) or override it (e.g.
    /// [`crate::SeatLayout`]).
    pub rotation: Rotation,
    /// Explicit seat assignment - side and optional edge offset;
    /// `None` leaves distribution to the strategy (e.g. round-robin
    /// in [`crate::SeatLayout`]).
    pub seat: Option<Seat>,
}

/// Field-wise equality - the unit marker `U` carries no value.
impl<U, T: PartialEq> PartialEq for LayoutItem<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.rotation == other.rotation && self.seat == other.seat
    }
}

impl<U, T: Copy> Copy for LayoutItem<U, T> {}

impl<U, T: Clone> Clone for LayoutItem<U, T> {
    fn clone(&self) -> Self {
        Self {
            size: self.size.clone(),
            rotation: self.rotation,
            seat: self.seat,
        }
    }
}

impl<U, T: Copy> LayoutItem<U, T> {
    /// Creates an item with the given footprint and rotation, no
    /// explicit seat.
    pub fn new(size: Vec2<U, T>, rotation: Rotation) -> Self {
        Self { size, rotation, seat: None }
    }

    /// The item with an explicit seat assignment.
    pub fn with_seat(self, seat: Seat) -> Self {
        Self { seat: Some(seat), ..self }
    }
}
