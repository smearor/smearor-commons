use crate::AnchorWeights;
use crate::LayoutError;
use crate::LayoutItem;
use crate::LayoutStrategy;
use crate::Placement;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// Line-flow arrangement: items side by side from the `anchor` corner
/// until the far edge of `bounds`, then wrapping to the next line.
///
/// Footprints arrive already rotated, so items keep their
/// `rotation` (and never overlap, whatever their angle or scale).
/// Overflow yields [`LayoutError::DoesNotFit`] - the consumer decides
/// whether to retry, shrink or leave items unarranged.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FlowLayout {
    /// Corner the flow starts from (`horizontal` picks the item
    /// direction, `vertical` the line direction).
    pub anchor: AnchorWeights,
    /// Gap between adjacent footprints and between lines - a scalar
    /// in the same coordinate space as `bounds` (pixels for
    /// `Rect<Pixels>`, millimeters for `Rect<Millimeters>`); `U` is a
    /// phantom marker, so the unit comes from the call, not the type.
    pub gap: f64,
    /// Wrap into a new line when the next item would cross the far
    /// edge. `false` means a single line - an overflowing item is
    /// the reported [`LayoutError::DoesNotFit`].
    pub wrap: bool,
}

impl Default for FlowLayout {
    fn default() -> Self {
        Self {
            anchor: AnchorWeights::TOP_LEFT,
            gap: 24.0,
            wrap: true,
        }
    }
}

/// One packed line: the item indices it holds and its extent along
/// the cross axis (`row height` / `column width`).
struct Line {
    /// Indices into `items`, in placement order.
    indices: Vec<usize>,
    /// Extent of the tallest/widest item on the line's cross axis.
    cross: f64,
}

impl FlowLayout {
    /// Packs items into lines along the main axis. Each line fits the
    /// `main` bounds extent; an item wider than that never fits and
    /// is reported immediately, regardless of `wrap`.
    fn pack_lines(&self, sizes: &[(f64, f64)], main: f64, items: usize) -> Result<Vec<Line>, LayoutError> {
        let mut lines: Vec<Line> = Vec::new();
        let mut line = Line {
            indices: Vec::new(),
            cross: 0.0,
        };
        let mut used = 0.0_f64;
        for (index, &(item_main, item_cross)) in sizes.iter().enumerate() {
            if item_main > main {
                return Err(LayoutError::DoesNotFit { index, items });
            }
            let needed = if line.indices.is_empty() { item_main } else { self.gap + item_main };
            if used + needed > main {
                if !self.wrap {
                    return Err(LayoutError::DoesNotFit { index, items });
                }
                lines.push(line);
                line = Line {
                    indices: Vec::new(),
                    cross: 0.0,
                };
                used = 0.0;
            }
            line.indices.push(index);
            line.cross = line.cross.max(item_cross);
            used = if line.indices.len() == 1 { item_main } else { used + self.gap + item_main };
        }
        if !line.indices.is_empty() {
            lines.push(line);
        }
        Ok(lines)
    }
}

impl<U> LayoutStrategy<U> for FlowLayout {
    /// Rows run along the axis `anchor.horizontal` picks (left-to-right
    /// or right-to-left), lines stack along `anchor.vertical`. Item
    /// `rotation` passes through unchanged; `seat` is ignored.
    fn arrange(&self, bounds: Rect<U>, items: &[LayoutItem<U>]) -> Result<Vec<Placement<U>>, LayoutError> {
        let count = items.len();
        let left_to_right = self.anchor.horizontal.as_f64() < 0.5;
        let top_down = self.anchor.vertical.as_f64() < 0.5;
        // (main, cross) per item along the flow axes.
        let sizes: Vec<(f64, f64)> = items.iter().map(|item| (item.size.x, item.size.y)).collect();
        let lines = self.pack_lines(&sizes, bounds.size.x, count)?;
        let total_cross = lines.iter().map(|line| line.cross).sum::<f64>() + self.gap * lines.len().saturating_sub(1) as f64;
        if total_cross > bounds.size.y {
            let index = lines.last().and_then(|line| line.indices.first()).copied().unwrap_or(0);
            return Err(LayoutError::DoesNotFit { index, items: count });
        }

        let mut placements: Vec<Option<Placement<U>>> = vec![None; count];
        let mut cursor_cross = if top_down { bounds.min.y } else { bounds.min.y + bounds.size.y };
        for line in &lines {
            let line_min_cross = if top_down { cursor_cross } else { cursor_cross - line.cross };
            let mut cursor_main = if left_to_right { bounds.min.x } else { bounds.min.x + bounds.size.x };
            for &index in &line.indices {
                let (item_main, item_cross) = sizes[index];
                let (x, y) = if left_to_right {
                    let x = cursor_main;
                    cursor_main += item_main + self.gap;
                    (x, line_min_cross)
                } else {
                    cursor_main -= item_main + self.gap;
                    (cursor_main + self.gap, line_min_cross)
                };
                placements[index] = Some(Placement::new(Vec2::new(x + item_main / 2.0, y + item_cross / 2.0), items[index].rotation, items[index].size));
            }
            cursor_cross = if top_down {
                cursor_cross + line.cross + self.gap
            } else {
                cursor_cross - line.cross - self.gap
            };
        }
        placements
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(LayoutError::DoesNotFit { index: count, items: count })
    }
}

#[cfg(test)]
mod tests {
    use super::FlowLayout;
    use crate::AnchorWeights;
    use crate::LayoutItem;
    use crate::LayoutStrategy;
    use smearor_commons_geometry::Pixels;
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

    fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
        a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
    }

    /// Every pair of target footprints is disjoint.
    #[test]
    fn placements_never_overlap() {
        let items = [item(100.0, 80.0), item(150.0, 60.0), item(80.0, 100.0), item(120.0, 50.0)];
        let placements = FlowLayout::default().arrange(bounds(), &items).expect("fits");
        let rects: Vec<(f64, f64, f64, f64)> = placements
            .iter()
            .map(|p| (p.center.x - p.size.x / 2.0, p.center.y - p.size.y / 2.0, p.size.x, p.size.y))
            .collect();
        for i in 0..rects.len() {
            for j in (i + 1)..rects.len() {
                assert!(!rects_overlap(rects[i], rects[j]), "{i} overlaps {j}");
            }
        }
    }

    /// A line exceeding the width wraps into the next row.
    #[test]
    fn wraps_when_line_overflows() {
        let items = [item(300.0, 50.0), item(300.0, 50.0)];
        let placements = FlowLayout::default().arrange(bounds(), &items).expect("wraps");
        assert!(placements[0].center.y < placements[1].center.y, "second item on a later line");
    }

    /// `wrap = false` reports the first overflowing item.
    #[test]
    fn single_line_overflow_errors() {
        let items = [item(300.0, 50.0), item(300.0, 50.0)];
        let error = FlowLayout {
            wrap: false,
            ..FlowLayout::default()
        }
        .arrange(bounds(), &items)
        .unwrap_err();
        assert_eq!(error.to_string(), "item 1 of 2 does not fit inside the bounds");
    }

    /// Wrapped lines taller than the bounds height overflow too.
    #[test]
    fn vertical_overflow_errors() {
        // Two 250-wide items force one line each; 2x200 + gap = 424 > 300.
        let items = [item(250.0, 200.0), item(250.0, 200.0)];
        assert!(FlowLayout::default().arrange(bounds(), &items).is_err());
    }

    /// A bottom-right anchor flows right-to-left, stacking upward.
    #[test]
    fn anchor_flips_directions() {
        let items = [item(100.0, 50.0), item(100.0, 50.0)];
        let placements = FlowLayout {
            anchor: AnchorWeights::BOTTOM_RIGHT,
            gap: 10.0,
            wrap: true,
        }
        .arrange(bounds(), &items)
        .expect("fits");
        // First item hugs the bottom-right corner.
        assert!((placements[0].center.x - 350.0).abs() < 1e-9);
        assert!((placements[0].center.y - 275.0).abs() < 1e-9);
        // Second item sits further left on the same line.
        assert!((placements[1].center.x - 240.0).abs() < 1e-9);
    }

    /// The item rotation reaches the placement unchanged.
    #[test]
    fn rotation_passes_through() {
        let items = [LayoutItem::new(Vec2::new(50.0, 50.0), Rotation::from_degrees(42.0))];
        let placements = FlowLayout::default().arrange(bounds(), &items).expect("fits");
        assert_eq!(placements[0].rotation, Rotation::from_degrees(42.0));
    }

    /// Input order is index-stable in the output.
    #[test]
    fn output_is_index_stable() {
        let items = [item(50.0, 50.0), item(300.0, 50.0), item(50.0, 50.0)];
        let placements = FlowLayout::default().arrange(bounds(), &items).expect("fits");
        assert_eq!(placements.len(), 3);
        // Item 2 (wide) wrapped, so it sits below item 0.
        assert!(placements[2].center.y > placements[1].center.y || placements[1].center.y > placements[0].center.y);
    }
}
