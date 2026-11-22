# smearor-commons-noise

Deterministic procedural noise over `Vec2`/`Vec3`/`Vec4`: the Hoskins
hash family as a generic `Hash<Out>` trait, table-free simplex fields
(`Simplex2D`, `Simplex3D` + the fractal variants) and the generic `Fbm`
combinator — pure std, headless.

- **[Hash](noise/hash.md)** — `Hash<Out>`, the 13 `hashNN` impls,
  `fract` semantics.
- **[Simplex](noise/simplex.md)** — `Field<P>` trait, both simplex
  fields, the fixed fractal variants.
- **[fBm](noise/fbm.md)** — `Fbm` with `Octaves`/`Gain`/`Lacunarity`
  newtypes.
- **[Seeding](noise/seeding.md)** — `Seed`, coordinate-offset seeding,
  `mix` for sibling seeds.

Everything is deterministic: the hash family is sine-free and the
simplex variants take their gradients from `hash22`/`hash33`, so a
`Seed` always replays the same field.
