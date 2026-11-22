//! Dumps the experimenter's test card composited over the desktop
//! backdrop as a PNG - the quick visual check for
//! `common::test_card` + `common::desktop` without opening the
//! window. The output path defaults to the fx book chapter's image
//! directory.
//!
//! Run with `cargo run -p smearor-commons-fx --features gtk4 --example dump_card -- [path.png]`.

#[allow(dead_code)]
mod common;

use gtk4::prelude::*;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "book/src/fx/img/test_card.png".to_string());
    let _ = gtk4::init();
    let size = Size::new(400, 300);
    let card = common::test_card(size);
    let desktop = common::desktop(size);
    // the compositor pass the experimenter runs per frame: card
    // source-over desktop
    let mut display = PixelBuffer::<Rgba>::new(size);
    let inv = Vec2::<()>::new(1.0 / f64::from(size.x), 1.0 / f64::from(size.y));
    for y in 0..size.y {
        for x in 0..size.x {
            let uv = Vec2::<()>::new((x as f64 + 0.5) * inv.x, (y as f64 + 0.5) * inv.y);
            let over = card.view().sample_nearest(uv);
            let under = desktop.view().sample_nearest(uv);
            display.view_mut().put_color(Pixels::new(x), Pixels::new(y), over.alpha_over(&under));
        }
    }
    let texture: gtk4::gdk::Texture = display.to_memory_texture().upcast();
    let png = texture.save_to_png_bytes();
    match std::fs::write(&path, png) {
        Ok(()) => println!("wrote {path}"),
        Err(error) => eprintln!("dump failed: {error}"),
    }
}
