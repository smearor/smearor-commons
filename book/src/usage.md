# Usage

How consumer crates and applications use `smearor-commons`: pick the
crates you need, opt integrations in via feature gates, and wrap the
technical types in your own semantic newtypes.

## Adding the crates

```toml
[dependencies]
smearor-commons-geometry = "0.1"
smearor-commons-color    = "0.1"
smearor-commons-layout   = "0.1"
smearor-commons-noise    = "0.1"
```

Optional integrations stay off until you name the feature — features
are always named after the dependency they enable:

```toml
[dependencies]
smearor-commons-buffer   = { version = "0.1", features = ["gtk4"] }
smearor-commons-drawing  = { version = "0.1", features = ["cairo"] }
smearor-commons-layer    = { version = "0.1", features = ["gtk4-layer-shell"] }
smearor-commons-geometry = { version = "0.1", features = ["schemars"] }
```

`serde` needs no feature — it is unconditional, so every type
serializes out of the box.

## A quick tour

### Units and orientation — geometry

```rust
use smearor_commons_geometry::{Fraction, Pixels, QuarterTurn, Rect, Size, Vec2};

let surface: Size<u32> = Size::new(1920, 1080);
let bounds: Rect<Pixels> = Rect::from_xywh(0.0, 0.0, 1920.0, 1080.0);
let center: Vec2<Pixels> = Vec2::new(960.0, 540.0);
let half = Fraction::HALF;

// Four snapped orientations for a multi-user tabletop:
let facing_south = QuarterTurn::Deg0.clockwise().clockwise(); // Deg180
```

Units are type-level markers — a `Vec2<Pixels>` and a
`Vec2<Millimeters>` cannot be mixed up by accident.

### Colors — color

```rust
use std::str::FromStr;
use smearor_commons_color::Color;

let brand = Color::<u8>::from_str("#00a1e4")?; // celestial blue
let dimmed = brand.fade(0.4);
```

### Seats on the table — layout

```rust
use smearor_commons_geometry::{Pixels, QuarterTurn, Rect, Vec2};
use smearor_commons_layout::{LayoutItem, LayoutStrategy, Seat, SeatLayout, TableSide};

// A user on the far edge gets content rotated 180°:
let seat = Seat::center(TableSide::Top);
let facing = seat.side.to_quarter_turn(); // QuarterTurn::Deg180

// A layout strategy assigns items to seats — headless and testable:
let bounds: Rect<Pixels> = Rect::from_xywh(0.0, 0.0, 1920.0, 1080.0);
let items = [
    LayoutItem::new(Vec2::new(420.0, 300.0), QuarterTurn::Deg0.into())
        .with_seat(Seat::center(TableSide::Bottom)),
    LayoutItem::new(Vec2::new(420.0, 300.0), QuarterTurn::Deg180.into())
        .with_seat(Seat::center(TableSide::Top)),
];
let placements = SeatLayout::default().arrange(bounds, &items)?;
```

### Rendering — buffer + drawing

```rust
use std::str::FromStr;
use smearor_commons_buffer::{PixelBuffer, Rgba};
use smearor_commons_color::Color;
use smearor_commons_drawing::RenderContext;
use smearor_commons_geometry::{Fraction, Pixels, Size};

let mut buffer = PixelBuffer::<Rgba>::new(Size::new(800, 480));
let mut context = RenderContext::new(&mut buffer)
    .background(Color::<f64>::from_str("#0a0e16")?);

context.fill_background()?;
context.draw_label("Hello, desk")?;
context.draw_progress_bar(Fraction::new(0.7), Pixels::new(4.0f32))?;
```

The software-raster target is infallible; cairo targets report
`DrawingError` through the same call signatures.

### Animating — animation

```rust
use std::time::{Duration, Instant};
use smearor_commons_animation::{EasingFunction, Span, TimedAnimation, Trajectory};

let mut animation = TimedAnimation::new(
    Span::new(0.0, 180.0),
    Duration::from_millis(350),
    EasingFunction::EaseOutCubic,
);
animation.start();
let degrees = animation.value_at(Instant::now()); // Option<f64>
```

### Layer-shell config — layer

```rust
use std::str::FromStr;
use smearor_commons_layer::{LayerConfig, SmearorLayer};

// e.g. `smearor-app --layer overlay`:
let layer = SmearorLayer::from_str("overlay")?;
let config = LayerConfig {
    layer: Some(layer),
    namespace: Some("swipe-launcher".to_string()),
    ..Default::default()
};
let zone = config.effective_exclusive_zone();
```

## Semantic newtypes — the intended pattern

The crates ship technical, domain-free types. Consumers give them
domain meaning as thin newtype wrappers:

```rust
use serde::{Deserialize, Serialize};
use smearor_commons_color::Color;
use smearor_commons_geometry::QuarterTurn;

/// A game figure's color — serializes exactly like `Color<u8>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FigureColor(pub Color<u8>);

/// Which way the game board faces.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BoardOrientation(pub QuarterTurn);
```

`#[serde(transparent)]` keeps the wire format identical to the inner
type — a `FigureColor` is a `"#dc0073"` string, a `BoardOrientation`
is `"Deg180"`. Standard trait impls (`FromStr`, `Display`, `From`)
delegate straight to the wrapped type.

## Error handling

Every crate defines its own `thiserror` enum — `ParseLayerError`,
`BufferError`, `LayoutError`, `DrawingError` — instead of panicking.
Consumers forward them via `?` inside their own error types:

```rust
use std::str::FromStr;
use smearor_commons_layer::SmearorLayer;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid --layer value")]
    Layer(#[from] smearor_commons_layer::ParseLayerError),
}

fn layer_arg(argument: &str) -> Result<SmearorLayer, ConfigError> {
    Ok(SmearorLayer::from_str(argument)?)
}
```
