use crate::LayoutError;
use crate::LayoutItem;
use crate::LayoutStrategy;
use crate::Placement;
use crate::SeatAssignment;
use crate::TableSide;
use crate::footprint_for;
use crate::seat;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// Four-seat arrangement: every item lands in front of a table side,
/// rotated for the user sitting there.
///
/// Items sharing a side spread along its edge with `gap` spacing
/// (centered as a group); an explicit [`LayoutItem::seat`] wins over
/// the `assignment` fallback.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SeatLayout {
    /// Assignment for items without an explicit `seat`.
    pub assignment: SeatAssignment,
    /// Footprint distance to the table edge - scalar in the
    /// coordinate space of `bounds` (see [`crate::FlowLayout::gap`]).
    pub edge_margin: f64,
    /// Spacing between items sharing the same side - same space
    /// as `bounds`.
    pub gap: f64,
}

impl Default for SeatLayout {
    fn default() -> Self {
        Self {
            assignment: SeatAssignment::RoundRobin,
            edge_margin: 24.0,
            gap: 24.0,
        }
    }
}

impl SeatLayout {
    /// Whether the side's edge runs horizontally (`Bottom`/`Top`) or
    /// vertically (`Left`/`Right`).
    fn horizontal_edge(side: TableSide) -> bool {
        matches!(side, TableSide::Bottom | TableSide::Top)
    }
}

impl<U> LayoutStrategy<U> for SeatLayout {
    /// Per side: items spread along the edge centered as a group,
    /// each `edge_margin` off the edge, rotated to the side's
    /// orientation. Extents along the edge use the footprint at the
    /// target rotation ([`footprint_for`]). A group wider than its
    /// edge reports [`LayoutError::DoesNotFit`].
    fn arrange(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<Placement<U>>, LayoutError> {
        let count = items.len();
        let sides: Vec<TableSide> = items
            .iter()
            .enumerate()
            .map(|(index, item)| item.seat.unwrap_or_else(|| TableSide::ALL[index % TableSide::ALL.len()]))
            .collect();

        let mut placements: Vec<Option<Placement<U>>> = vec![None; count];
        for side in TableSide::ALL {
            let group: Vec<usize> = (0..count).filter(|&index| sides[index] == side).collect();
            if group.is_empty() {
                continue;
            }
            // Footprints at the target rotation drive every extent.
            let target: Vec<Vec2<U>> = group
                .iter()
                .map(|&index| footprint_for(items[index].size, items[index].rotation, side.to_quarter_turn()))
                .collect();
            let edge_extents: Vec<f64> = target
                .iter()
                .map(|footprint| if Self::horizontal_edge(side) { footprint.x } else { footprint.y })
                .collect();
            let span = edge_extents.iter().sum::<f64>() + self.gap * group.len().saturating_sub(1) as f64;
            let edge_length = if Self::horizontal_edge(side) { bounds.size.x } else { bounds.size.y };
            if span > edge_length {
                return Err(LayoutError::DoesNotFit { index: group[0], items: count });
            }

            // Walk the edge from the group's centered start position.
            let mut cursor = {
                let center = if Self::horizontal_edge(side) {
                    bounds.min.x + bounds.size.x / 2.0
                } else {
                    bounds.min.y + bounds.size.y / 2.0
                };
                center - span / 2.0
            };
            for (&index, footprint) in group.iter().zip(&target) {
                let edge_extent = if Self::horizontal_edge(side) { footprint.x } else { footprint.y };
                let mut placement = seat(side, bounds, *footprint, self.edge_margin);
                placement.center = if Self::horizontal_edge(side) {
                    Vec2::new(cursor + edge_extent / 2.0, placement.center.y)
                } else {
                    Vec2::new(placement.center.x, cursor + edge_extent / 2.0)
                };
                placements[index] = Some(placement);
                cursor += edge_extent + self.gap;
            }
        }
        placements
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(LayoutError::DoesNotFit { index: count, items: count })
    }
}

#[cfg(test)]
mod tests {
    use super::SeatLayout;
    use crate::LayoutItem;
    use crate::LayoutStrategy;
    use crate::TableSide;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::Vec2;

    fn bounds() -> Rect<Pixels> {
        Rect {
            min: Vec2::new(0.0, 0.0),
            size: Vec2::new(400.0, 300.0),
        }
    }

    fn item(width: f64, height: f64) -> LayoutItem<Pixels> {
        LayoutItem::new(Vec2::new(width, height), Rotation::ZERO)
    }

    /// Round-robin deals items over `TableSide::ALL` in input order.
    #[test]
    fn round_robin_deals_to_all_sides() {
        let items = [item(50.0, 50.0); 5];
        let placements = SeatLayout::default().arrange(bounds(), &items).expect("fits");
        for (index, placement) in placements.iter().enumerate() {
            let expected = Rotation::QuarterTurn(TableSide::ALL[index % 4].to_quarter_turn());
            assert_eq!(placement.rotation, expected, "item {index}");
        }
    }

    /// An explicit seat wins over the round-robin order.
    #[test]
    fn explicit_seat_wins() {
        let items = [item(100.0, 50.0).with_seat(TableSide::Right)];
        let placements = SeatLayout::default().arrange(bounds(), &items).expect("fits");
        assert_eq!(placements[0].rotation, Rotation::QuarterTurn(QuarterTurn::Deg270));
        // Target footprint is 50x100 after the Deg270 axis swap.
        assert!((placements[0].center.x - (400.0 - 24.0 - 25.0)).abs() < 1e-9);
    }

    /// Items sharing a side spread along its edge without overlap.
    #[test]
    fn same_side_items_spread() {
        let items = [item(100.0, 50.0).with_seat(TableSide::Bottom), item(100.0, 50.0).with_seat(TableSide::Bottom)];
        let placements = SeatLayout::default().arrange(bounds(), &items).expect("fits");
        // Two 100-wide items + 24 gap centered on the 400-wide edge.
        assert!((placements[0].center.x - (200.0 - 12.0 - 50.0)).abs() < 1e-9);
        assert!((placements[1].center.x - (200.0 + 12.0 + 50.0)).abs() < 1e-9);
    }

    /// A group wider than its edge reports `DoesNotFit`.
    #[test]
    fn overflowing_side_errors() {
        let items = [item(300.0, 50.0).with_seat(TableSide::Top), item(300.0, 50.0).with_seat(TableSide::Top)];
        assert!(SeatLayout::default().arrange(bounds(), &items).is_err());
    }

    /// Rotated items occupy their *target* footprint on the edge:
    /// a 200-wide item at Left (Deg90) needs 200 of vertical edge.
    #[test]
    fn target_footprint_drives_edge_extents() {
        let items = [item(200.0, 50.0).with_seat(TableSide::Left), item(200.0, 50.0).with_seat(TableSide::Left)];
        // Two 50x200 footprints + 24 gap = 424 > 300 edge length.
        assert!(SeatLayout::default().arrange(bounds(), &items).is_err());
        let ok = [item(100.0, 50.0).with_seat(TableSide::Left), item(100.0, 50.0).with_seat(TableSide::Left)];
        assert!(SeatLayout::default().arrange(bounds(), &ok).is_ok());
    }
}
