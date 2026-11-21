# RenderContext

`RenderContext<'a, T: DrawTarget>` is the builder facade over a render
target — it bundles fonts, colors and the active flag once, then
exposes convenience methods with the tile layout baked in.

## Configuration

```rust
use smearor_commons_color::Color;
use smearor_commons_drawing::RenderContext;

let mut ctx = RenderContext::new(target)
    .icon_font(Some(&icon_font))     // icons + text fallback
    .label_font(Some(&label_font))   // text; falls back to icon_font, then bitmap
    .background(Color::new_rgba(0.04, 0.05, 0.08, 1.0))
    .icon_color(Color::new_rgba(0.0, 0.63, 0.89, 1.0))
    .text_color(Color::new_rgba(0.94, 0.94, 0.94, 1.0))
    .active(true);
```

Anything left unset resolves through the target's
`default_text_color(is_active)` — and `fill_background()` simply does
nothing without a configured `background`.

## Drawing

```rust
use smearor_commons_geometry::{Fraction, Grid, Pixels, Vec2};

ctx.fill_background()?;

// glyph centered at 0.4·height, size 0.6·min(w,h) — the tile look
ctx.draw_icon('\u{f07b}')?;
ctx.draw_icon_by_name("folder", |name| resolve(name))?;

// bottom-anchored label; centered and free-positioned variants
ctx.draw_label("Launcher")?;
ctx.draw_text("42", Pixels::new(80.0))?;                    // centered on baseline
ctx.draw_text_sized("42", Pixels::new(80.0), Pixels::new(28.0))?;
ctx.draw_text_at(Vec2::<Pixels, f32>::new(8.0, 24.0), "hi", Pixels::new(14.0))?;

ctx.draw_progress_bar(Fraction::new(0.68), Pixels::new(5.0))?;
ctx.draw_icon_grid(&names, grid, |name| resolve(name))?;
```

The same layout conventions the spec-level API leaves to the caller are
fixed here: `draw_icon` centers at `0.4 · height` (the visual sweet
spot above a bottom label) at `0.6 · min(w, h)` size; labels and the
progress strip anchor to the bottom edge. `size()`, `width()`,
`height()` forward the target's extent for consumer-side layout math.

## Error type

All methods return `Result<(), T::Error>` — `Infallible` on buffer
targets, `cairo::Error` on `CairoTarget`. The context adds no error
type of its own.
