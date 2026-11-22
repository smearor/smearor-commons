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
