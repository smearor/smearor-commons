# Easing

`EasingFunction` maps linear time progress to eased progress —
`apply(t)` turns a constant-speed clock into acceleration curves. All
endpoints are fixed at `(0, 0)` and `(1, 1)`; what happens between is
the curve's personality.

<figure class="diagram">
  <img src="img/easing-curves.svg" alt="The eight easing curves: Linear, quadratic and cubic ease-in/out/in-out, and Overshoot exceeding 1.0">
  <figcaption>the exact formulas from easing.rs — overshoot deliberately exceeds 1.0</figcaption>
</figure>

## The eight variants

```rust
use smearor_commons_animation::EasingFunction;
use smearor_commons_geometry::Fraction;

EasingFunction::Linear.apply(Fraction::HALF);          // 0.5 — constant speed
EasingFunction::EaseOutCubic.apply(Fraction::HALF);    // 0.875 — the arrive curve
EasingFunction::overshoot().apply(Fraction::HALF);     // 0.5 — below 0 before, past 1 after
```

- **Quadratic**: `EaseIn` (`t²`), `EaseOut` (`1−(1−t)²`), `EaseInOut`
- **Cubic**: `EaseInCubic` (`t³`), `EaseOutCubic` (`1−(1−t)³`) — the
  classic "arrive" curve for camera and pose transitions —
  `EaseInOutCubic`
- **`Overshoot { overshoot_amount }`**: Penner's back-easing —
  `EasingFunction::overshoot()` gives the canonical `s = 1.70158`,
  `DEFAULT_OVERSHOOT`. Eased progress dips below 0, rises past 1.0 and
  settles — the trajectory on the other end decides what extrapolated
  `t` means.

Eased `t` exceeding `[0, 1]` is a feature, not a bug: see
[Interpolation](./interpolation.md) for how value types handle it.
