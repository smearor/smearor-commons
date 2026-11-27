# Strategies

`LayoutStrategy<U>` is the contract — a pure function mapping
`LayoutItem`s onto `Placement`s inside `bounds`. Strategies see
footprints, never widgets or domain objects, so any crate can
implement one and hand it to a shared caller.

<figure class="diagram">
  <img src="img/strategies.svg" alt="FlowLayout wrapping anchored rows versus SeatLayout arranging items around table edges">
  <figcaption>flow reads like text; seat layout arranges around the perimeter</figcaption>
</figure>

```rust
pub trait LayoutStrategy<U> {
    fn arrange(&self, bounds: Rect<U>, items: &[LayoutItem<U>])
        -> Result<Vec<Placement<U>>, LayoutError>;
}
```

**Index-stable**: `placements[i]` answers `items[i]` — the caller maps
back by position. Everything is unit-generic over `U`: the same
strategy arranges a pixel canvas or a physical millimeter table.

## `FlowLayout` — anchored rows

```rust
use smearor_commons_geometry::Distance;
use smearor_commons_layout::{AnchorWeights, FlowLayout};

let flow = FlowLayout {
    anchor: AnchorWeights::TOP_LEFT,      // corner the flow starts from
    gap: Distance::new(24.0),             // between items and between lines
    wrap: true,                           // wrap to the next line at the far edge
};
// Default::default() → TOP_LEFT, gap 24, wrap on
```

`FlowLayout<U>` is unit-generic: `gap` is a `Distance<U>` in the
`bounds` coordinate space — a bare `f64` reads as `U` by context.

Non-overlapping rows starting at `anchor` — `horizontal` picks the
item direction, `vertical` the line direction. `wrap = false` means a
single line; an overflowing item reports `DoesNotFit`. The item's own
`rotation` is preserved.

## `SeatLayout` — around the table

```rust
use smearor_commons_geometry::Distance;
use smearor_commons_layout::SeatLayout;

let seats = SeatLayout {
    assignment: SeatAssignment::RoundRobin,   // for items without a seat
    edge_margin: Distance::new(24.0),         // distance off the table edge
    gap: Distance::new(24.0),                 // between items on one side
};
```

Per side: **pinned** seats (`Seat::at`) take their exact offset;
**unpinned** items spread along the edge centered as a group, each
`edge_margin` off the edge, **rotated to the side's orientation** —
footprints computed via `footprint_for` at the target quarter turn.
A spread group wider than its edge reports `DoesNotFit`.

`SeatAssignment::RoundRobin` distributes items without a seat over
`TableSide::ALL` in input order — item `i` lands on `ALL[i % 4]`.

## `AnchorWeights` — the 9-point grid

```rust
AnchorWeights::TOP_LEFT;     // horizontal 0.0, vertical 0.0
AnchorWeights::CENTER;       // 0.5 / 0.5
AnchorWeights::BOTTOM_RIGHT; // 1.0 / 1.0
```

Two `Fraction`s selecting one of nine anchors — corners, edge centers
and center. `TableSide::anchor_weights()` maps a side to its
matching anchor for consumers that blend flow and seat semantics.

## `EdgeStripLayout` — docked strips outside the host

```rust
use smearor_commons_geometry::Distance;
use smearor_commons_layout::{EdgeDistribution, EdgeStripLayout, SideSet, TableSide};

let strip = EdgeStripLayout {
    sides: SideSet::try_from_sides([TableSide::Bottom, TableSide::Left])?,
    distribution: EdgeDistribution::Balanced,
    gap: Distance::new(12.0),
};
```

Items line up as a strip *outside* `bounds`, along each edge named by
`sides` — host-attached chrome like docked tool strips or satellites,
hovering `gap` off the host outline so the host's own extent stays
untouched. Each side's group is centered on its edge and ordered along
the side's reading direction (`TableSide::right_direction`);
placements carry the side's quarter-turn orientation.
`EdgeDistribution::Sequential` fills side by side in `SideSet` order
with the overflow spilling to the next edge; `Balanced` centers an
even share on every selected edge. Strips colliding at a corner — or a
group exceeding its edge — report `DoesNotFit`.

## `OrbitLayout` — items on a ring

```rust
use smearor_commons_geometry::{Arc, Rotation, RotationRad, Vec2};
use smearor_commons_layout::{orbit_radius_for, OrbitItem, OrbitLayout};

let orbit = OrbitLayout::new(orbit_radius_for(bounds, 12.0), Arc::FULL);
let items = vec![
    OrbitItem::new(Vec2::new(20.0, 20.0), Rotation::ZERO).with_anchor(RotationRad::ZERO), // pinned at 0°
    OrbitItem::new(Vec2::new(20.0, 20.0), Rotation::ZERO),                                 // flexible
];
let placements = orbit.arrange(bounds, &items);
let directions = orbit.directions(&items);   // the angle-only half
```

Same arrange pattern, but deliberately not a `LayoutStrategy` impl: it
takes [`OrbitItem`s](items.md) — an `anchor: Option<RotationRad>` pins
an item to an absolute direction on the orbit circle — and never
fails, so `arrange` returns `Vec<Placement<U>>` directly. Anchored
items keep their angle verbatim (even outside `span`); flexible items
distribute evenly over `span` (a limited `Arc` or `Arc::FULL`) or fill
the anchor gaps clipped to `span` proportionally to segment width —
largest-remainder rounding. `orbit_radius_for(bounds, gap)` derives a
radius clearing the host's bounding circle.

## `LayoutError`

```rust
#[error("item {index} of {items} does not fit inside the bounds")]
DoesNotFit { index: usize, items: usize }
#[error("a side set needs at least one table side")]
EmptySides
```

The strategy reports the first item that failed; the consumer decides
the reaction — retry with different options, shrink items, or leave
the overflow unarranged. Layout never silently overlaps. `EmptySides`
is the construction-time sibling: `SideSet::try_from_sides` refuses an
empty edge selection, keeping "at least one side" a type-level
invariant.
