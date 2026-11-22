//! Phase-6 benchmark: times `apply` for representative effects.
//! `FX_THREADS` controls the fill fan-out (`1` = sequential) so the
//! spec's ">2x speedup" include criterion for parallel rows is
//! measurable:
//!
//! ```sh
//! FX_THREADS=1 cargo run --release -p smearor-commons-fx --example fx_bench
//! cargo run --release -p smearor-commons-fx --example fx_bench
//! ```

use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_fx::Direction;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::FxContext;
use smearor_commons_fx::Layer;
use smearor_commons_fx::Layers;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Size;
use smearor_commons_noise::Seed;
use std::time::Duration;
use std::time::Instant;

/// A deterministic card stand-in - a diagonal gradient, enough
/// variation to keep the sampling paths honest.
fn input(size: Size<u32>) -> PixelBuffer<Rgba> {
    let mut buffer = PixelBuffer::<Rgba>::new(size);
    let mut view = buffer.view_mut();
    let w = f64::from(size.x).max(1.0);
    let h = f64::from(size.y).max(1.0);
    for y in 0..size.y {
        for x in 0..size.x {
            let c = Color::<f64>::new_rgba(f64::from(x) / w, f64::from(y) / h, 0.5, if (x / 16 + y / 16) % 2 == 0 { 1.0 } else { 0.8 });
            view.put_color(Pixels::new(x), Pixels::new(y), c);
        }
    }
    buffer
}

fn bench(nick: &str, size: Size<u32>, reps: usize) {
    let spec = EffectSpec::by_nick(nick).unwrap();
    let effect = spec.build();
    let card = input(size);
    let mut out = PixelBuffer::<Rgba>::new(size);
    let ctx = FxContext {
        progress: Fraction::new(0.5),
        duration: Duration::from_millis(1200),
        seed: Seed(0x5EED),
        direction: Direction::Close,
        layer: Layer::Window,
        window: card.view(),
        surface: None,
        assets: smearor_commons_fx::FxAssets::default(),
        pointer: None,
        fullscreen: false,
        padding: Pixels::new(0.0),
        window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
    };
    // warmup, then timed reps
    effect.apply(&ctx, &mut out.view_mut());
    let start = Instant::now();
    for _ in 0..reps {
        effect.apply(&ctx, &mut out.view_mut());
    }
    let ms = start.elapsed().as_secs_f64() * 1000.0 / reps as f64;
    println!("{nick:<16} {}x{}  {ms:7.2} ms/frame  ({} reps)", size.x, size.y, reps);
}

fn main() {
    let threads = std::env::var("FX_THREADS").unwrap_or_else(|_| "auto".into());
    println!("FX_THREADS={threads}  cores={:?}", std::thread::available_parallelism().map(|n| n.get()));
    // heavy: 45-tap cone blur | medium: fbm | light: plain lerp
    for nick in ["focus", "fire", "crossfade"] {
        if EffectSpec::by_nick(nick).map(|s| s.build().layers() == Layers::WindowOnly).unwrap_or(false) {
            bench(nick, Size::new(400, 300), 10);
            bench(nick, Size::new(800, 600), 4);
        }
    }
}
