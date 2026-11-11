# Introduction

`smearor-commons` is a mono-workspace of small, framework-agnostic Rust
libraries shared across the Smearor ecosystem (table-top smart desk
frameworks, applications and games).

- `smearor-commons-geometry` — unit-tagged 2D vectors, rotations,
  rectangles and shapes.
- `smearor-commons-color` — channel-generic `Color<T>`, hex parsing and
  color operations.

Both crates are published to crates.io. Optional integrations
(async-graphql, schemars, zbus, evdev, gtk4, cairo, smithay) live behind
feature gates so headless consumers pull no GUI dependencies.
