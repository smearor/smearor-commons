use crate::EdgeDistribution;
use crate::LayoutError;
use crate::LayoutItem;
use crate::LayoutStrategy;
use crate::Placement;
use crate::SideSet;
use crate::TableSide;
use crate::footprint_for;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Distance;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Rotation;
use smearor_commons_geometry::Vec2;

/// Edge-strip arrangement: items line up as a strip *outside* the
/// `bounds` rectangle, along each edge named by `sides`.
///
/// Meant for host-attached chrome (docked tool strips, satellites):
/// the strip hovers `gap` off the host outline, so the host's own
/// extent is untouched and the consumer can clip-union both
/// footprints if needed. The strip works in the host's local frame -
/// rotating the host is the consumer's transform to apply afterwards.
///
/// Each side's group is centered on the edge and ordered along the
/// side's reading direction ([`TableSide::right_direction`]), so item
/// order reads naturally for a user seated there. Placements carry
/// the side's quarter-turn orientation; consumers may override
/// `rotation` afterwards. Overlapping strips (long strips on
/// adjacent edges colliding at a corner) are reported
/// [`LayoutError::DoesNotFit`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EdgeStripLayout<U = ()> {
    /// The edges to dock to - one to four, non-empty by `SideSet`'s
    /// invariant.
    pub sides: SideSet,
    /// How items spread over `sides`.
    pub distribution: EdgeDistribution,
    /// Distance between the host outline and the strip, and between
    /// adjacent items, in the `U` coordinate space of `bounds`.
    pub gap: Distance<U>,
}

impl<U> Default for EdgeStripLayout<U> {
    fn default() -> Self {
        Self {
            sides: SideSet::default(),
            distribution: EdgeDistribution::default(),
            gap: Distance::new(12.0),
        }
    }
}

impl<U> EdgeStripLayout<U> {
    /// Whether `side`'s edge runs horizontally (`Bottom`/`Top`).
    fn horizontal(side: TableSide) -> bool {
        matches!(side, TableSide::Bottom | TableSide::Top)
    }

    /// The midpoint of `side`'s edge of `bounds`.
    fn edge_midpoint(bounds: Rect<U>, side: TableSide) -> Vec2<U> {
        let center = bounds.center();
        match side {
            TableSide::Bottom => Vec2::new(center.x, bounds.min.y + bounds.size.y),
            TableSide::Left => Vec2::new(bounds.min.x, center.y),
            TableSide::Top => Vec2::new(center.x, bounds.min.y),
            TableSide::Right => Vec2::new(bounds.min.x + bounds.size.x, center.y),
        }
    }

    /// Groups items onto the sides per `distribution`: each group's
    /// item indices in input order.
    fn groups(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<(TableSide, Vec<usize>)>, LayoutError> {
        match self.distribution {
            EdgeDistribution::Sequential => self.sequential_groups(bounds, items),
            EdgeDistribution::Balanced => Ok(self.balanced_groups(items.len())),
        }
    }

    /// Greedy side-by-side fill: each side takes as many leading
    /// items as fit its edge, the rest spills to the next side.
    fn sequential_groups(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<(TableSide, Vec<usize>)>, LayoutError> {
        let mut pending: Vec<usize> = (0..items.len()).collect();
        let mut groups = Vec::new();
        for side in self.sides.iter() {
            if pending.is_empty() {
                break;
            }
            let edge_length = if Self::horizontal(side) { bounds.size.x } else { bounds.size.y };
            let gap = self.gap.as_f64();
            let mut used = 0.0_f64;
            let mut taken = 0_usize;
            for &index in &pending {
                let footprint = footprint_for(items[index].size, items[index].rotation, side.to_quarter_turn());
                let tangential = if Self::horizontal(side) { footprint.x } else { footprint.y };
                let needed = if taken == 0 { tangential } else { gap + tangential };
                if used + needed > edge_length {
                    break;
                }
                used += needed;
                taken += 1;
            }
            groups.push((side, pending.drain(..taken).collect()));
        }
        if let Some(&index) = pending.first() {
            return Err(LayoutError::DoesNotFit { index, items: items.len() });
        }
        Ok(groups)
    }

    /// Even split over all sides, largest remainder to earlier sides.
    fn balanced_groups(&self, count: usize) -> Vec<(TableSide, Vec<usize>)> {
        let sides: Vec<TableSide> = self.sides.iter().collect();
        let mut groups = Vec::with_capacity(sides.len());
        let mut cursor = 0_usize;
        for (position, side) in sides.iter().enumerate() {
            let group_count = count / sides.len() + usize::from(position < count % sides.len());
            groups.push((*side, (cursor..cursor + group_count).collect()));
            cursor += group_count;
        }
        groups
    }

    /// Places one side's group centered on its edge and returns the
    /// group's error when it does not fit.
    fn place_group(&self, bounds: Rect<U>, items: &[LayoutItem<U>], side: TableSide, group: &[usize]) -> Result<Vec<(usize, Placement<U>)>, LayoutError> {
        let gap = self.gap.as_f64();
        let turn = side.to_quarter_turn();
        let footprints: Vec<Vec2<U>> = group
            .iter()
            .map(|&index| footprint_for(items[index].size, items[index].rotation, turn))
            .collect();
        let edge_length = if Self::horizontal(side) { bounds.size.x } else { bounds.size.y };
        let span = footprints
            .iter()
            .map(|footprint| if Self::horizontal(side) { footprint.x } else { footprint.y })
            .sum::<f64>()
            + gap * group.len().saturating_sub(1) as f64;
        if span > edge_length {
            return Err(LayoutError::DoesNotFit {
                index: group.first().copied().unwrap_or(0),
                items: items.len(),
            });
        }

        let edge_midpoint = Self::edge_midpoint(bounds, side);
        let right: Vec2<U> = side.right_direction();
        let outward = Vec2::<U>::new(-right.y, right.x);
        let rotation = Rotation::QuarterTurn(turn);
        let mut cursor = -span / 2.0;
        let mut placed = Vec::with_capacity(group.len());
        for (&index, footprint) in group.iter().zip(&footprints) {
            let (tangential, perpendicular) = if Self::horizontal(side) {
                (footprint.x, footprint.y)
            } else {
                (footprint.y, footprint.x)
            };
            let center = edge_midpoint + right * (cursor + tangential / 2.0) + outward * (gap + perpendicular / 2.0);
            placed.push((index, Placement::new(center, rotation, *footprint)));
            cursor += tangential + gap;
        }
        Ok(placed)
    }
}

/// Whether two placement footprints overlap (strictly - touching
/// edges do not collide).
fn overlaps<U>(a: &Placement<U>, b: &Placement<U>) -> bool {
    (a.center.x - b.center.x).abs() * 2.0 < a.size.x + b.size.x && (a.center.y - b.center.y).abs() * 2.0 < a.size.y + b.size.y
}

impl<U> LayoutStrategy<U> for EdgeStripLayout<U> {
    /// Per [`EdgeDistribution`]: `Sequential` fills side by side with
    /// overflow spilling clockwise, `Balanced` centers an even share
    /// on every selected edge. Placement rotation is the side's
    /// quarter turn; `seat` is ignored. Strips overlapping at a
    /// corner - or a group exceeding its edge - yield
    /// [`LayoutError::DoesNotFit`].
    fn arrange(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<Placement<U>>, LayoutError> {
        let mut placements: Vec<Option<Placement<U>>> = vec![None; items.len()];
        for (side, group) in self.groups(bounds, items)? {
            for (index, placement) in self.place_group(bounds, items, side, &group)? {
                placements[index] = Some(placement);
            }
        }
        let placements: Vec<Placement<U>> = placements.into_iter().collect::<Option<Vec<_>>>().ok_or(LayoutError::DoesNotFit {
            index: items.len(),
            items: items.len(),
        })?;
        for (index, placement) in placements.iter().enumerate() {
            if placements[..index].iter().any(|earlier| overlaps(earlier, placement)) {
                return Err(LayoutError::DoesNotFit { index, items: items.len() });
            }
        }
        Ok(placements)
    }
}

#[cfg(test)]
mod tests {
    use super::EdgeStripLayout;
    use crate::EdgeDistribution;
    use crate::LayoutError;
    use crate::LayoutItem;
    use crate::LayoutStrategy;
    use crate::SideSet;
    use crate::TableSide;
    use smearor_commons_geometry::Distance;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::QuarterTurn;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::Vec2;

    fn bounds() -> Rect<Pixels> {
        Rect {
            min: Vec2::new(100.0, 80.0),
            size: Vec2::new(400.0, 240.0),
        }
    }

    fn item(width: f64, height: f64) -> LayoutItem<Pixels> {
        LayoutItem::new(Vec2::new(width, height), Rotation::ZERO)
    }

    /// A single side docks one centered strip below the host, items
    /// in input order along the side's reading direction.
    #[test]
    fn strip_docks_centered_below() {
        let layout = EdgeStripLayout {
            sides: SideSet::single(TableSide::Bottom),
            distribution: EdgeDistribution::Sequential,
            gap: Distance::new(10.0),
        };
        let placements = layout.arrange(bounds(), &[item(60.0, 30.0), item(40.0, 30.0)]).expect("fits");
        // span = 60 + 10 + 40 = 110, centered on x=300: first item at 245+30=...
        assert!((placements[0].center.x - (300.0 - 55.0 + 30.0)).abs() < 1e-9);
        assert!((placements[1].center.x - (300.0 - 55.0 + 60.0 + 10.0 + 20.0)).abs() < 1e-9);
        // Outside the bottom edge: 320 + gap 10 + half height 15.
        assert!((placements[0].center.y - 345.0).abs() < 1e-9);
        assert_eq!(placements[0].rotation, Rotation::QuarterTurn(QuarterTurn::Deg0));
    }

    /// Left and right strips order along the edge axis in each
    /// side's own reading direction.
    #[test]
    fn vertical_strips_follow_reading_direction() {
        let layout = EdgeStripLayout {
            sides: SideSet::try_from_sides([TableSide::Left, TableSide::Right]).expect("non-empty"),
            distribution: EdgeDistribution::Balanced,
            gap: Distance::new(10.0),
        };
        let placements = layout.arrange(bounds(), &[item(30.0, 60.0), item(30.0, 60.0)]).expect("fits");
        // Deg90/Deg270 swap the footprint axes: the 30x60 items display
        // 60x30, so the strips sit 30 (perpendicular half) + gap off
        // the edges - Left at x = 100 - 10 - 30 = 60, Right at 540.
        assert!((placements[0].center.x - 60.0).abs() < 1e-9);
        assert!((placements[1].center.x - 540.0).abs() < 1e-9);
        // Left's right_direction is +y (down), Right's is -y (up).
        assert!((placements[0].center.y - 200.0).abs() < 1e-9);
        assert!((placements[1].center.y - 200.0).abs() < 1e-9);
        assert_eq!(placements[0].rotation, Rotation::QuarterTurn(QuarterTurn::Deg90));
        assert_eq!(placements[1].rotation, Rotation::QuarterTurn(QuarterTurn::Deg270));
    }

    /// Sequential overflows to the next side only when the edge is
    /// full.
    #[test]
    fn sequential_spills_to_next_side() {
        let layout = EdgeStripLayout {
            sides: SideSet::try_from_sides([TableSide::Bottom, TableSide::Top]).expect("non-empty"),
            distribution: EdgeDistribution::Sequential,
            gap: Distance::new(0.0),
        };
        // Four 150-wide items: two fit the 400-wide bottom edge
        // (300 <= 400, 450 > 400), the rest spills to the top.
        let items = [item(150.0, 30.0), item(150.0, 30.0), item(150.0, 30.0), item(150.0, 30.0)];
        let placements = layout.arrange(bounds(), &items).expect("fits");
        assert!(placements[2].center.y < bounds().min.y, "spilled item docks above the top edge");
        assert!(placements[3].center.y < bounds().min.y, "spilled item docks above the top edge");
        assert_eq!(placements[3].rotation, Rotation::QuarterTurn(QuarterTurn::Deg180));
    }

    /// An item fitting no remaining side is an error.
    #[test]
    fn overflow_errors() {
        let layout = EdgeStripLayout {
            sides: SideSet::single(TableSide::Bottom),
            distribution: EdgeDistribution::Sequential,
            gap: Distance::new(0.0),
        };
        let items = [item(450.0, 30.0)];
        assert_eq!(layout.arrange(bounds(), &items), Err(LayoutError::DoesNotFit { index: 0, items: 1 }));
    }

    /// Balanced splits evenly, largest remainder to the earlier side.
    #[test]
    fn balanced_splits_evenly() {
        let layout = EdgeStripLayout {
            sides: SideSet::try_from_sides([TableSide::Bottom, TableSide::Top]).expect("non-empty"),
            distribution: EdgeDistribution::Balanced,
            gap: Distance::new(10.0),
        };
        let items = [item(30.0, 30.0), item(30.0, 30.0), item(30.0, 30.0)];
        let placements = layout.arrange(bounds(), &items).expect("fits");
        // 3 items over 2 sides: bottom gets 2, top gets 1.
        assert!(placements[0].center.y > bounds().max().y);
        assert!(placements[1].center.y > bounds().max().y);
        assert!(placements[2].center.y < bounds().min.y);
    }

    /// Strips overlapping - here at a corner through a negative gap -
    /// are an error.
    #[test]
    fn overlapping_strips_error() {
        let layout = EdgeStripLayout {
            sides: SideSet::try_from_sides([TableSide::Bottom, TableSide::Right]).expect("non-empty"),
            distribution: EdgeDistribution::Balanced,
            gap: Distance::new(-10.0),
        };
        // Both items fill their edge end to end (the right-side item
        // reads 240x30 before its Deg270 turn swaps the footprint);
        // the negative gap pushes each strip's center line into the
        // host, so the corner footprints overlap.
        let items = [item(400.0, 30.0), item(240.0, 30.0)];
        assert!(matches!(layout.arrange(bounds(), &items), Err(LayoutError::DoesNotFit { .. })));
    }

    /// Empty input yields empty output.
    #[test]
    fn empty_items() {
        assert!(EdgeStripLayout::default().arrange(bounds(), &[]).expect("empty").is_empty());
    }

    /// Serde round-trips the configuration.
    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&EdgeStripLayout::<Pixels>::default()).expect("serialize");
        assert_eq!(
            serde_json::from_str::<EdgeStripLayout<Pixels>>(&json).expect("deserialize"),
            EdgeStripLayout::<Pixels>::default()
        );
    }
}
