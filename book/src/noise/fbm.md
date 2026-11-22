# fBm

`Fbm` sums `octaves` copies of any `Field<P>`, `lacunarity` scaling
frequency and `gain` scaling amplitude per octave:

```rust,ignore
Fbm::FOUR_OCTAVES.at(&Simplex2D, p)
```

Fields are newtypes (`Octaves(u32)`, `Gain(f64)`, `Lacunarity(f64)`),
`serde(transparent)` — `{"octaves":4,"gain":0.5,"lacunarity":2.0}`.
`FOUR_OCTAVES` is the classic 4 / 0.5 / 2.0 configuration.

<img src="img/fbm_octaves.svg" alt="Four panels: cumulative fBm over Simplex2D with 1 to 4 octaves - detail accumulates while amplitude halves">
