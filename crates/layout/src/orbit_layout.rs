use crate::OrbitItem;
use crate::Placement;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Distance;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Vec2;
use std::cmp::Ordering;
use std::f64::consts::TAU;

/// Orbit arrangement: items placed on an arc (full circle or limited
/// span) around the center of the host `bounds`.
///
/// Follows the [`crate::LayoutStrategy`] pattern - pure, index-stable,
/// over technical types - but takes [`OrbitItem`]s instead of
/// [`crate::LayoutItem`]s so each item can carry an optional anchor
/// direction: anchored items keep their angle verbatim (even outside
/// `span`), flexible items distribute evenly when no anchor exists or
/// fill the anchor gaps' intersection with `span` proportionally to
/// segment width (largest-remainder rounding, the algorithm extracted
/// from the pie menu's `redistribute_angles`).
///
/// The strategy never fails: degenerate inputs (empty item list,
/// zero-width span, all gaps clipped away) yield deterministic
/// placements - items may stack, consumers pick sensible parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrbitLayout<U = ()> {
    /// The angular span items distribute over - [`Arc::FULL`] for a
    /// full circle. Anchored items ignore it; flexible items only
    /// land inside it.
    pub span: Arc,
    /// Distance from the host `bounds`' center to each item center, in
    /// the `U` coordinate space of `bounds`. [`orbit_radius_for`]
    /// derives a radius that clears a rectangular host.
    pub radius: Distance<U>,
}

impl<U> OrbitLayout<U> {
    /// An orbit with the given radius and span - `f64` radii read as
    /// `U` by context.
    pub fn new(radius: impl Into<Distance<U>>, span: Arc) -> Self {
        Self { span, radius: radius.into() }
    }

    /// A full-circle orbit with the given radius.
    pub fn full(radius: impl Into<Distance<U>>) -> Self {
        Self::new(radius, Arc::FULL)
    }

    /// Whether `span` covers the whole turn - endpoint-inclusive and
    /// endpoint-exclusive distributions differ there.
    fn is_full(&self) -> bool {
        self.span.width >= RotationRad::FULL_TURN
    }

    /// The target placement of every item: `center` on the orbit at
    /// the item's angle, `rotation` and `size` passed through.
    /// Index-stable: `placements[i]` answers `items[i]`.
    pub fn arrange(&self, bounds: Rect<U>, items: &[OrbitItem<U>]) -> Vec<Placement<U>> {
        let center = bounds.center();
        self.directions(items)
            .into_iter()
            .zip(items)
            .map(|(angle, item)| Placement::new(center + Vec2::<U>::from_polar(self.radius.as_f64(), angle), item.rotation, item.size))
            .collect()
    }

    /// The direction of every item on the orbit, input order kept:
    /// anchored items verbatim, flexible items distributed - the
    /// angle-only half of [`Self::arrange`] for consumers that place
    /// items themselves (e.g. a radial menu on its own canvas).
    pub fn directions(&self, items: &[OrbitItem<U>]) -> Vec<RotationRad> {
        let mut anchors: Vec<RotationRad> = items.iter().filter_map(|item| item.anchor.map(|anchor| anchor.normalized())).collect();
        anchors.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let flexible = items.iter().filter(|item| item.anchor.is_none()).count();
        let mut distributed = (if anchors.is_empty() {
            self.even_angles(flexible)
        } else {
            self.distribute_between_anchors(&anchors, flexible)
        })
        .into_iter();
        items
            .iter()
            .map(|item| {
                item.anchor
                    .map_or_else(|| distributed.next().unwrap_or(RotationRad::ZERO), |anchor| anchor.normalized())
            })
            .collect()
    }

    /// `count` evenly spread directions inside `span` - the no-anchor
    /// case. A full turn spaces `i/n` (endpoint equals start), a
    /// limited span spaces `i/(n-1)` so items cover both endpoints; a
    /// single item sits at the span's midpoint.
    fn even_angles(&self, count: usize) -> Vec<RotationRad> {
        match count {
            0 => Vec::new(),
            1 => vec![self.span.midpoint()],
            count if self.is_full() => (0..count).map(|index| self.span.at(Fraction::new(index as f64 / count as f64))).collect(),
            count => (0..count).map(|index| self.span.at(Fraction::new(index as f64 / (count - 1) as f64))).collect(),
        }
    }

    /// `count` directions inside the anchor gaps clipped to `span`:
    /// each clipped segment receives a quota proportional to its
    /// width, rounded by largest remainder; inside a segment items
    /// space evenly (`(offset+1)/(count+1)` of the segment width) so
    /// they never land on the anchors at the edges.
    fn distribute_between_anchors(&self, anchors: &[RotationRad], count: usize) -> Vec<RotationRad> {
        let segments: Vec<Arc> = Arc::gaps_between(anchors).iter().flat_map(|gap| clip_to_span(*gap, self.span)).collect();
        let total_width: f64 = segments.iter().map(|segment| segment.width.as_f64()).sum();
        if total_width <= 0.0 {
            return self.even_angles(count);
        }
        let quota = |segment: &Arc| count as f64 * segment.width.as_f64() / total_width;
        let mut allocations: Vec<usize> = segments.iter().map(|segment| quota(segment).floor() as usize).collect();
        let remainder = count.saturating_sub(allocations.iter().sum());
        let mut remainders: Vec<(usize, f64)> = segments
            .iter()
            .enumerate()
            .map(|(index, segment)| (index, quota(segment) - quota(segment).floor()))
            .collect();
        remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        for (index, _) in remainders.iter().take(remainder) {
            allocations[*index] += 1;
        }

        let mut angles = Vec::with_capacity(count);
        for (segment, &segment_count) in segments.iter().zip(&allocations) {
            for offset in 0..segment_count {
                angles.push(segment.at(Fraction::new((offset + 1) as f64 / (segment_count + 1) as f64)));
            }
        }
        angles
    }
}

/// `gap` clipped to `span`, in `span`-relative linear space: shifting
/// `gap` by `-span.start` makes `span` the plain interval `[0, width]`,
/// so the intersection is at most two pieces (the wrap of a relative
/// gap past the full turn). Zero-width pieces are dropped.
fn clip_to_span(gap: Arc, span: Arc) -> Vec<Arc> {
    let limit = span.width.as_f64();
    if limit >= TAU {
        return vec![gap];
    }
    let rel_start = (gap.start - span.start).normalized().as_f64();
    let rel_end = rel_start + gap.width.as_f64();
    let mut pieces = Vec::new();
    let end = rel_end.min(limit);
    if end > rel_start {
        pieces.push(Arc::new(span.start + RotationRad::new(rel_start), RotationRad::new(end - rel_start)));
    }
    if rel_end > TAU {
        let wrap_end = (rel_end - TAU).min(limit);
        if wrap_end > 0.0 {
            pieces.push(Arc::new(span.start, RotationRad::new(wrap_end)));
        }
    }
    pieces
}

/// The orbit radius clearing `bounds`: half the host diagonal plus
/// `gap`, so items on a full-circle orbit never touch the host's
/// bounding circle. Consumers with non-circular hosts may pick a
/// larger radius or accept nearer passes along the short axes.
pub fn orbit_radius_for<U>(bounds: Rect<U>, gap: impl Into<Distance<U>>) -> Distance<U> {
    Distance::new(bounds.size.magnitude_f64() / 2.0 + gap.into().as_f64())
}

#[cfg(test)]
mod tests {
    use super::OrbitLayout;
    use super::orbit_radius_for;
    use crate::OrbitItem;
    use crate::Placement;
    use smearor_commons_geometry::Arc;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Rotation;
    use smearor_commons_geometry::RotationRad;
    use smearor_commons_geometry::Vec2;

    fn bounds() -> Rect<Pixels> {
        Rect::centered(Vec2::new(200.0, 150.0), Vec2::new(100.0, 60.0))
    }

    fn item() -> OrbitItem<Pixels> {
        OrbitItem::new(Vec2::new(20.0, 20.0), Rotation::ZERO)
    }

    fn direction(placement: &Placement<Pixels>) -> RotationRad {
        (placement.center - Vec2::new(200.0, 150.0)).direction().normalized()
    }

    /// No anchors on a full circle: `i/n` spacing starting at the
    /// span's start direction.
    #[test]
    fn even_distribution_on_full_circle() {
        let placements = OrbitLayout::full(100.0).arrange(bounds(), &[item(), item(), item(), item()]);
        let degrees: Vec<f64> = placements.iter().map(|p| direction(p).to_degrees()).collect();
        assert!((degrees[0] - 0.0).abs() < 1e-9);
        assert!((degrees[1] - 90.0).abs() < 1e-9);
        assert!((degrees[2] - 180.0).abs() < 1e-9);
        assert!((degrees[3] - 270.0).abs() < 1e-9);
    }

    /// No anchors on a limited span: `i/(n-1)` spacing covering both
    /// endpoints.
    #[test]
    fn even_distribution_on_partial_span() {
        let span = Arc::new(RotationRad::ZERO, RotationRad::QUARTER_TURN);
        let placements = OrbitLayout::new(100.0, span).arrange(bounds(), &[item(), item(), item(), item()]);
        let degrees: Vec<f64> = placements.iter().map(|p| direction(p).to_degrees()).collect();
        for (index, expected) in [0.0, 30.0, 60.0, 90.0].iter().enumerate() {
            assert!((degrees[index] - expected).abs() < 1e-9, "{index}");
        }
    }

    /// A single flexible item sits at the span's midpoint.
    #[test]
    fn single_item_at_midpoint() {
        let span = Arc::new(RotationRad::ZERO, RotationRad::QUARTER_TURN);
        let placements = OrbitLayout::new(100.0, span).arrange(bounds(), &[item()]);
        assert!((direction(&placements[0]).to_degrees() - 45.0).abs() < 1e-9);
    }

    /// Anchored items keep their angle; flexible items fill the gap
    /// proportionally - the extracted pie-menu semantics.
    #[test]
    fn anchors_pin_and_gaps_distribute() {
        let items = [
            item().with_anchor(RotationRad::ZERO),
            item(),
            item(),
            item().with_anchor(RotationRad::QUARTER_TURN),
            item(),
        ];
        let placements = OrbitLayout::full(100.0).arrange(bounds(), &items);
        assert!((direction(&placements[0]).to_degrees() - 0.0).abs() < 1e-9);
        assert!((direction(&placements[3]).to_degrees() - 90.0).abs() < 1e-9);
        // Gaps 90deg and 270deg wide share three flexibles 1:2.
        let degrees: Vec<f64> = [1, 2, 4].iter().map(|&i| direction(&placements[i]).to_degrees()).collect();
        assert!((degrees[0] - 45.0).abs() < 1e-9);
        assert!((degrees[1] - 180.0).abs() < 1e-9);
        assert!((degrees[2] - 270.0).abs() < 1e-9);
    }

    /// Anchor gaps are clipped to a limited span: anchors at the span
    /// ends leave only the in-span segment for flexible items.
    #[test]
    fn anchors_clip_to_partial_span() {
        let span = Arc::new(RotationRad::ZERO, RotationRad::QUARTER_TURN);
        let items = [
            item().with_anchor(RotationRad::ZERO),
            item(),
            item(),
            item().with_anchor(RotationRad::QUARTER_TURN),
        ];
        let placements = OrbitLayout::new(100.0, span).arrange(bounds(), &items);
        assert!((direction(&placements[1]).to_degrees() - 30.0).abs() < 1e-9);
        assert!((direction(&placements[2]).to_degrees() - 60.0).abs() < 1e-9);
    }

    /// An anchor outside `span` is honored verbatim while flexible
    /// items still distribute inside the clipped gaps.
    #[test]
    fn anchor_outside_span_stays() {
        let span = Arc::new(RotationRad::ZERO, RotationRad::QUARTER_TURN);
        let items = [item().with_anchor(RotationRad::HALF_TURN), item()];
        let placements = OrbitLayout::new(100.0, span).arrange(bounds(), &items);
        assert!((direction(&placements[0]).to_degrees() - 180.0).abs() < 1e-9);
        // The only gap (full circle back to the anchor) clips to span.
        assert!((direction(&placements[1]).to_degrees() - 45.0).abs() < 1e-9);
    }

    /// The item rotation reaches the placement unchanged.
    #[test]
    fn rotation_passes_through() {
        let items = [OrbitItem::new(Vec2::new(20.0, 20.0), Rotation::from_degrees(42.0))];
        let placements = OrbitLayout::full(100.0).arrange(bounds(), &items);
        assert_eq!(placements[0].rotation, Rotation::from_degrees(42.0));
    }

    /// `orbit_radius_for` clears the host's bounding circle by `gap`.
    #[test]
    fn radius_for_clears_bounding_circle() {
        let radius = orbit_radius_for(bounds(), 12.0);
        assert!((radius.as_f64() - (116.619_037_896_06_f64 / 2.0 + 12.0)).abs() < 1e-6);
    }
}
