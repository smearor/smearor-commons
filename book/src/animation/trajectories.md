# Trajectories

A `Trajectory` maps eased progress `t` to a value — **the shape of an
animation's output over time**. Time lives in `TimedAnimation`; the
trajectory only knows how to answer `value_at(t)`.

## `Span<T>` — two endpoints

```rust
use smearor_commons_animation::{Span, Trajectory};
use smearor_commons_geometry::{Fraction, Pixels, Vec2};

let span = Span::new(0.0_f64, 100.0);
span.value_at(Fraction::HALF);          // 50.0

// any Interpolate type — positions move both axes on one clock
let slide = Span::new(Vec2::<Pixels>::new(0.0, 0.0), Vec2::<Pixels>::new(320.0, 0.0));
```

## `ThreePhaseSpan<T>` — out, hold, back

`new(from, mid, to, lead, trail)`: interpolates `from → mid` over
`[0, lead)`, holds `mid` over `[lead, trail]`, interpolates `mid → to`
over the rest.

<figure class="diagram">
  <img src="img/three-phase.svg" alt="ThreePhaseSpan value curve: descend to mid at lead, hold until trail, return to the endpoint">
  <figcaption>the pulse shape — zoom out, hold, zoom back in</figcaption>
</figure>

`ThreePhaseSpan<ScaleFactor>` with `from = to = 1.0` is the
zoom-during-rotation pattern; `PulseAnimation` is exactly that.

## Tuples — composite values on one clock

`(A, B)` implements `Trajectory` when both members do — a
rotate-while-zooming animation needs no dedicated struct:

```rust
use smearor_commons_animation::{Span, ThreePhaseSpan};
use smearor_commons_geometry::{RotationRad, ScaleFactor};

let trajectory = (
    Span::new(RotationRad::ZERO, RotationRad::QUARTER_TURN),
    ThreePhaseSpan::new(
        ScaleFactor::new(1.0), ScaleFactor::new(0.85), ScaleFactor::new(1.0),
        Fraction::new(0.3), Fraction::new(0.7),
    ),
);
// TimedAnimation<…>::value_at(t) → (RotationRad, ScaleFactor)
```

## Ready-made aliases

| Alias | Type | For |
|---|---|---|
| `RotationAnimation` | `TimedAnimation<Span<RotationRad>>` | tile/pose rotation |
| `ZoomAnimation` | `TimedAnimation<Span<ScaleFactor>>` | plain zoom |
| `PulseAnimation` | `TimedAnimation<ThreePhaseSpan<ScaleFactor>>` | out-hold-back |
| `Vec2Animation<U>` | `TimedAnimation<Span<Vec2<U>>>` | position/size |
| `RotationZoomAnimation` | `TimedAnimation<(Span<RotationRad>, ThreePhaseSpan<ScaleFactor>)>` | the rotate gesture's full effect |

`Fraction::clamped` inside `ThreePhaseSpan::new` keeps
`lead ≤ trail` — phase order can not be inverted by accident.
