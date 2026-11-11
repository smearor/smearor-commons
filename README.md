# smearor-commons

[![Version](https://img.shields.io/badge/version-0.1.0-f5b700.svg)](https://github.com/smearor/smearor-commons)
[![Rust Edition](https://img.shields.io/badge/rust-2024-00a1e4.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![GTK4](https://img.shields.io/badge/GTK4-v4__20-dc0073.svg)](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)
[![License](https://img.shields.io/badge/license-MIT-04e762.svg)](LICENSE)
[![Book](https://img.shields.io/badge/book-mdBook-89fc00.svg)](https://smearor.github.io/smearor-commons/book/)
[![Docs](https://img.shields.io/badge/docs-rustdoc-04e762.svg)](https://smearor.github.io/smearor-commons/docs/)
[![crates.io](https://img.shields.io/badge/crates.io-publish-dc0073.svg)](https://crates.io/search?q=smearor-commons)

Mono-workspace of shared, framework-agnostic Rust libraries for the
Smearor ecosystem (65" table-top smart desk frameworks, applications
and games). Published to crates.io.

## Crates

| Crate | Description |
|---|---|
| `smearor-commons-geometry` | Unit-tagged 2D vectors, rotations, rectangles and shapes |
| `smearor-commons-color` | Channel-generic `Color<T>`, hex parsing and color operations |

## Design

- Pure `std` core — no GUI/compositor dependencies by default.
- Optional integrations (`async-graphql`, `schemars`, `zbus`, `evdev`,
  `gtk4`, `cairo`, `smithay`) live behind feature gates.
- `serde` is an unconditional dependency.
- Semantic newtypes (`FigureColor`, `BoardOrientation`, `CardPosition`,
  ...) stay in consumer crates as thin wrappers around these technical
  types.

The phased extraction plan lives in
`concepts/planned/SMEAROR_COMMONS.md` (see the consuming repositories).

## License

MIT
