# Context

`FxContext` is the uniform block one effect sees for one frame - every
field is the deliberate analogue of a Burn-My-Windows uniform:

| Field | BMW uniform | Notes |
|---|---|---|
| `progress: Fraction` | `uProgress` | linear `0..=1`, pre-eased by the consumer's `TimedAnimation` |
| `duration: Duration` | `uDuration` | wall-clock run length (shaders scale speed by it) |
| `seed: Seed` | `Math.random()` | fix to replay bitwise |
| `direction: Direction` | `uForOpening` | `Open`/`Close` - progress always runs `0 -> 1` |
| `layer: Layer` | `uLayer` | `Surface` or `Window` per pass |
| `window: BufferView<Rgba>` | `sampler2D` | the animated content; `window.size()` is `uSize` |
| `surface: Option<BufferView>` | `uLayer = 0` source | only for `Layers::SurfaceAndWindow` |
| `pointer: Option<Vec2>` | `uEpicenter` | uv-space pointer (broken-glass) |
| `fullscreen: bool` | `uIsFullscreen` | effects clamp edge fades differently |
| `padding: Pixels<f64>` | `uPadding` | shadow margin around the content |
| `window_rect: Rect<Pixels, f64>` | `uFrame`/`uCenter` | the content rect inside the layer |

`Direction` keeps BMW's semantics: the clock never flips - an
out-flying particle flies *out* on close, not backward in time. For
plain fades, `Direction::weight(progress)` gives the visibility weight
(`progress` for `Open`, `1 - progress` for `Close`).

`Layers::SurfaceAndWindow` makes the consumer run two passes and
supply `ctx.surface`; `WindowEffect::check` pre-flights it and returns
`FxError::MissingSurface` when the surface is absent.
