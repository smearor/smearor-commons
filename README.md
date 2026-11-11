# smearor-commons

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
