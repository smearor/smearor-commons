# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- CI: install `libgtk-4-dev` and `libxkbcommon-dev` on runners so `--all-features` builds (gtk4, cairo, smithay) find the required system libraries; removed unused workspace dependencies (`evdev`, `smearor-commons-color`, `smearor-commons-font`).

### Added

- Initial workspace scaffold: `smearor-commons-geometry` and `smearor-commons-color` crate skeletons in a mono-workspace, prepared for publication to crates.io.
- `smearor-commons-geometry`: `Circle::bounding_box` returning the circumscribed square as `Rect<U, f64>`.
- `smearor-commons-layout`: headless, unit-generic tabletop layouting — `TableSide` (exhaustive clockwise/counter-clockwise stepping, `nearest(RotationRad)`, `anchor_weights()`), `Seat` (side plus optional edge `Fraction` offset; `perimeter_position()` orders seats along the table perimeter, `next_in`/`previous_in` find the next occupied seat in a roster, `placement()` positions a footprint in front of a seat), the `LayoutStrategy` trait with `FlowLayout` (non-overlapping anchored rows with wrapping) and `SeatLayout` (explicit seats win, `SeatAssignment::RoundRobin` distributes the rest over `TableSide::ALL`), `Placement<U>`/`LayoutItem<U>` and `LayoutError::DoesNotFit`.
`Seat::fanned_placement` places an item in the free spot nearest a
seat — filling freed gaps along the edge lane, checking `gap`
clearance against every physically occupying item, and allowing a
lane behind the edge row when the queue reaches farther from the seat
(`TableSide::right_direction`, `nearest_free_slot`).
- `smearor-commons-animation`: `Interpolate` trait (scalars, `Vec2`, `RotationRad` shortest-arc, `ScaleFactor`), `Span`/`ThreePhaseSpan` trajectories, `TimedAnimation` with `EasingFunction`s, and a `gtk4`-gated `drive` helper running animations on a widget's frame clock.
