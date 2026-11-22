# Fx

`smearor-commons-fx` is a headless, serializable per-pixel effect
library - the Rust analogue of Burn-My-Windows' shader effects,
generalized to any rectangular content a consumer can snapshot into
a `PixelBuffer<Rgba>` (widgets, cards, sprites).

Effects animate **textures, not widgets**: the consumer snapshots its
content once, hands it over as `ctx.window`, and plays the animation
as a frozen texture while the real widget stays hidden. Layout,
geometry and z-order remain the consumer's responsibility.

The crate is clock-free: `FxContext.progress` arrives pre-eased from
a `TimedAnimation` driven by the consumer.

- [Context](./fx/context.md) - the uniform block (`FxContext`,
  `Direction`, `Layer`, `Layers`)
- [Params](./fx/params.md) - the `ParamSpec`/`ParamKind`/`ParamValue`
  metadata model
- [Spec](./fx/spec.md) - `EffectSpec`, the serde-tagged dispatch root

## Shipped effects

| Nick | Effect | Mechanism |
|---|---|---|
| `crossfade` | `Crossfade` | Reference effect - plain alpha blend (Phase 1). |
| `glide` | `Glide` | UV transform (scale, squish, tilt, shift) + dissolve. |
| `pixel-wheel` | `PixelWheel` | Pixelation + angular spoke wipe. |
| `fire` | `Fire` | Fractal-noise burn front with a 5-stop flame ramp. |
| `matrix` | `Matrix` | Glyph rain + trail composite over a falling window mask; glyphs come from a `GlyphSource` (bundled `ProceduralGlyphAtlas`). |
| `aura-glow` | `AuraGlow` | Radial blur + cos-palette glow + superellipse crop. |

All six are reachable through `EffectSpec::by_nick`/`build` and survive
JSON round trips. Each rustdoc states the behavioral spec and names
its source shader in the Burn-My-Windows checkout for provenance; the
implementations are semantic re-implementations on our ops (`mix`,
`smoothstep`, `absolute_edge_mask`, `offset_hue`, `BufferView`
sampling, `Color::alpha_over`), not line ports.
