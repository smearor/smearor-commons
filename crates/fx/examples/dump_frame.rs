//! Renders one experimenter frame - an effect applied to the test
//! card, composited over the desktop backdrop - and writes it as a
//! PNG. The headless visual check for every `EffectSpec`, and the
//! image source for the fx book chapter.
//!
//! Run with
//! `cargo run -p smearor-commons-fx --features gtk4 --example dump_frame -- <effect> [progress] [direction] [path.png]`,
//! e.g. `... dump_frame matrix 0.6 open book/src/fx/img/matrix.png`.

#[allow(dead_code)]
mod common;

use gtk4::prelude::*;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_fx::Direction;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::FxContext;
use smearor_commons_fx::Layer;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Seed;
use std::time::Duration;

fn main() {
    let mut args = std::env::args().skip(1);
    let nick = args.next().unwrap_or_else(|| "matrix".to_string());
    let progress = args.next().and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.6);
    let direction = match args.next().as_deref() {
        Some("close") => Direction::Close,
        _ => Direction::Open,
    };
    let path = args.next().unwrap_or_else(|| format!("book/src/fx/img/{nick}.png"));
    let Ok(spec) = EffectSpec::by_nick(&nick) else {
        eprintln!(
            "unknown effect '{nick}' - known: {:?}",
            EffectSpec::all().iter().map(|s| s.build().nick().to_string()).collect::<Vec<_>>()
        );
        return;
    };
    let _ = gtk4::init();
    let size = Size::new(400, 300);
    let card = common::test_card(size);
    let desktop = common::desktop(size);
    let mut out = PixelBuffer::<Rgba>::new(size);
    let ctx = FxContext {
        progress: Fraction::new(progress.clamp(0.0, 1.0)),
        duration: Duration::from_secs_f64(3.0),
        seed: Seed(0x5EED_CAFE_F00D_1234),
        direction,
        layer: Layer::Window,
        window: card.view(),
        surface: None,
        pointer: None,
        fullscreen: false,
        padding: Pixels::new(12.0),
        window_rect: common::card_rect(size),
    };
    spec.build().apply(&ctx, &mut out.view_mut());
    // the compositor pass: `out` source-over `desktop`
    let mut display = PixelBuffer::<Rgba>::new(size);
    let inv = Vec2::<()>::new(1.0 / f64::from(size.x), 1.0 / f64::from(size.y));
    for y in 0..size.y {
        for x in 0..size.x {
            let uv = Vec2::<()>::new((x as f64 + 0.5) * inv.x, (y as f64 + 0.5) * inv.y);
            let over = out.view().sample_nearest(uv);
            let under = desktop.view().sample_nearest(uv);
            display.view_mut().put_color(Pixels::new(x), Pixels::new(y), over.alpha_over(&under));
        }
    }
    let texture: gtk4::gdk::Texture = display.to_memory_texture().upcast();
    match std::fs::write(&path, texture.save_to_png_bytes()) {
        Ok(()) => println!("wrote {path}"),
        Err(error) => eprintln!("dump failed: {error}"),
    }
}
