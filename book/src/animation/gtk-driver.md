# GTK Driver

The `gtk4` feature adds `drive` — the glue that samples a
`TimedAnimation` on a widget's frame clock. The core stays `std`-pure;
only this one function depends on GTK.

```rust
use smearor_commons_animation::{RotationAnimation, drive};
use smearor_commons_geometry::{RotationRad, Span};
use gtk4::glib::ControlFlow;
use std::time::Duration;

let animation = RotationAnimation::new(
    Span::new(RotationRad::ZERO, RotationRad::QUARTER_TURN),
    Duration::from_millis(250),
    EasingFunction::EaseOutCubic,
);

drive(&widget, animation, |widget, rotation| {
    widget.imp().rotation.set(rotation);
    widget.queue_allocate();
    ControlFlow::Continue
});
```

## How it works

`drive` installs `add_tick_callback` on the widget: every frame it
samples `animation.value()` and calls `apply(&widget, value)`.

- `value()` returns `None` — the tick removes itself
  (`ControlFlow::Break`); that covers both "not started" and
  "completed"
- `apply` returns a `ControlFlow` — `Continue` keeps sampling,
  `Break` detaches early (consumer-driven cancellation)
- `is_complete()` forces `Break` regardless of `apply`'s answer, so a
  finished animation never keeps the tick alive
- `S::Value: Copy` — values move into the closure by copy per frame

The widget owns the animation's lifetime: dropping the widget detaches
the tick, and the animation needs no `Rc`/`RefCell` — `drive` moves it
into the callback.

The `schemars` feature separately adds `JsonSchema` derives for
`EasingFunction`, `Span`, `ThreePhaseSpan` and friends — animation
configs can travel through serialized state.
