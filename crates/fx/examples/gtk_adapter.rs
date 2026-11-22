//! GTK adapter demo - the phase-5 recipe end to end: a real widget
//! is snapshotted into a frozen texture, an `FxPlayer` runs any
//! `EffectSpec` on it at the same spot, and `animation-ended` swaps
//! back to the live widget.
//!
//! Run: `cargo run -p smearor-commons-fx --features gtk4 --example gtk_adapter`

#[allow(dead_code)]
mod common;

use common::dropdown_row;
use common::group_frame;
use gtk4::Box as GtkBox;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_fx::Direction;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::Layers;
use smearor_commons_fx::gtk::FxPlayOptions;
use smearor_commons_fx::gtk::FxPlayer;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::time::Duration;

const APP_ID: &str = "ai.devin.smearor.fx.gtk-adapter";
/// Where the card sits on the board (`Fixed` coordinates).
const CARD_POS: (f64, f64) = (90.0, 70.0);

/// The "real" widget - a mini window made of an actual widget tree,
/// not a texture. Adwaita supplies the chrome: `.card` styling, an
/// `adw::HeaderBar` with `WindowTitle`, and genuine `WindowControls`
/// (real theme buttons). Everything the player animates is a snapshot
/// of this.
fn build_card() -> GtkBox {
    let body = GtkBox::new(Orientation::Vertical, 0);
    body.add_css_class("card");

    let header = adw::HeaderBar::new();
    header.set_show_start_title_buttons(false);
    header.set_show_end_title_buttons(false);
    header.set_title_widget(Some(&adw::WindowTitle::new("adapter-demo", "frozen texture")));
    let controls = gtk4::WindowControls::new(gtk4::PackType::End);
    controls.set_decoration_layout(Some(":minimize,maximize,close"));
    // The buttons are real and would act on the demo window - claim
    // pointer presses in the capture phase so they stay decorative.
    let eater = gtk4::GestureClick::new();
    eater.set_propagation_phase(gtk4::PropagationPhase::Capture);
    eater.connect_pressed(|gesture, _, _, _| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
    });
    controls.add_controller(eater);
    header.pack_end(&controls);
    body.append(&header);

    let content = GtkBox::new(Orientation::Vertical, 8);
    content.set_margin_top(12);
    content.set_margin_bottom(12);
    content.set_margin_start(12);
    content.set_margin_end(12);
    content.append(&gtk4::Label::new(Some("A real widget tree -")));
    content.append(&gtk4::Label::new(Some("snapshotted, animated, swapped back.")));
    let entry = gtk4::Entry::new();
    entry.set_text("live input");
    content.append(&entry);
    let button = gtk4::Button::with_label("Real button");
    content.append(&button);
    body.append(&content);
    body
}

/// A couple of "desktop icons" so the board behind the card is not
/// empty - the layered effects sample this as `ctx.surface`.
fn build_board() -> gtk4::Fixed {
    let board = gtk4::Fixed::new();
    board.set_size_request(560, 360);
    for (i, name) in ["📁 files", "🖥 machine", "🗑 trash", "📄 notes"].iter().enumerate() {
        let icon = gtk4::Label::new(Some(name));
        board.put(&icon, 24.0 + i as f64 * 72.0, 12.0);
    }
    let hint = gtk4::Label::new(Some("the board"));
    board.put(&hint, 460.0, 330.0);
    board
}

/// Crops `region` (buffer pixels) out of `buffer`.
fn crop(buffer: &PixelBuffer<Rgba>, x: u32, y: u32, w: u32, h: u32) -> PixelBuffer<Rgba> {
    let size = buffer.size();
    let mut out = PixelBuffer::<Rgba>::new(Size::new(w.min(size.x), h.min(size.y)));
    let inv = Vec2::<()>::new(1.0 / f64::from(size.x), 1.0 / f64::from(size.y));
    let view = buffer.view();
    let mut out_view = out.view_mut();
    for dy in 0..h.min(size.y) {
        for dx in 0..w.min(size.x) {
            let uv = Vec2::<()>::new((f64::from(x + dx) + 0.5) * inv.x, (f64::from(y + dy) + 0.5) * inv.y);
            out_view.put_color(Pixels::new(dx), Pixels::new(dy), view.sample_nearest(uv));
        }
    }
    out
}

fn main() -> glib::ExitCode {
    adw::init().expect("libadwaita init failed");
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let window = adw::ApplicationWindow::builder().application(app).title("fx GTK adapter demo").build();

        // --- controls -------------------------------------------------
        let controls = GtkBox::new(Orientation::Horizontal, 8);
        controls.set_margin_top(10);
        controls.set_margin_bottom(10);
        controls.set_margin_start(10);
        controls.set_margin_end(10);

        let specs = EffectSpec::all();
        let labels: Vec<&'static str> = specs.iter().map(|spec| spec.build().label()).collect();
        let nicks: Vec<&'static str> = specs.iter().map(EffectSpec::nick).collect();

        let effect_group = GtkBox::new(Orientation::Horizontal, 6);
        let (_r1, effect_dd) = dropdown_row(&effect_group, "Effect", &labels);
        controls.append(&group_frame("", &effect_group));
        let direction_group = GtkBox::new(Orientation::Horizontal, 6);
        let (_r2, direction_dd) = dropdown_row(&direction_group, "Direction", &["Open", "Close"]);
        controls.append(&group_frame("", &direction_group));

        let animate = gtk4::Button::with_label("▶ Animate");
        controls.append(&animate);

        // --- the board -------------------------------------------------
        let board = build_board();
        let card = build_card();
        board.put(&card, CARD_POS.0, CARD_POS.1);

        let page = GtkBox::new(Orientation::Vertical, 0);
        page.append(&controls);
        let scroll = gtk4::Frame::new(None);
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);
        scroll.set_child(Some(&board));
        page.append(&scroll);
        window.set_content(Some(&page));
        window.set_default_size(600, 460);

        // --- the adapter flow -------------------------------------------
        animate.connect_clicked({
            let board = board.clone();
            let card = card.clone();
            let window = window.clone();
            move |_| {
                let spec = specs[effect_dd.selected() as usize].clone();
                let direction = if direction_dd.selected() == 0 { Direction::Open } else { Direction::Close };
                run_effect(&board, &card, &window, spec, direction, &nicks);
            }
        });

        window.present();
        if std::env::var("FX_AUTO_ANIMATE").is_ok() {
            let animate = animate.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || {
                animate.emit_clicked();
            });
        }
    });
    app.run()
}

/// The recipe: snapshot the card, hide it, play an `FxPlayer` at its
/// position, swap back on `animation-ended`.
fn run_effect(board: &gtk4::Fixed, card: &GtkBox, _window: &adw::ApplicationWindow, spec: EffectSpec, direction: Direction, _nicks: &[&'static str]) {
    let Ok(frozen) = smearor_commons_fx::gtk::snapshot(card) else {
        eprintln!("snapshot failed - card not renderable");
        return;
    };
    let layered = spec.build().layers() == Layers::SurfaceAndWindow;
    let scale = card.scale_factor().max(1) as u32;
    let (card_x, card_y, card_w, card_h) = (
        (CARD_POS.0 as u32) * scale,
        (CARD_POS.1 as u32) * scale,
        card.width() as u32 * scale,
        card.height() as u32 * scale,
    );

    // surface = the board under the card, with the card hidden - the
    // backdrop the layered effects sample.
    card.set_visible(false);
    let surface = layered.then(|| {
        let board_buf = smearor_commons_fx::gtk::snapshot(board).unwrap_or_else(|_| PixelBuffer::<Rgba>::new(Size::new(1, 1)));
        crop(&board_buf, card_x, card_y, card_w, card_h)
    });

    let player = FxPlayer::new();
    player.set_effect(spec, frozen);
    if let Some(surface) = surface {
        player.set_surface_provider(Box::new(move || PixelBuffer::<Rgba>::from_raw(surface.size(), surface.pixels().to_vec()).ok()));
    }
    board.put(&player, CARD_POS.0, CARD_POS.1);
    player.connect_animation_ended({
        let board = board.clone();
        let card = card.clone();
        let player = player.clone();
        move |_| {
            board.remove(&player);
            card.set_visible(true);
        }
    });
    player.play(FxPlayOptions {
        direction,
        duration: Duration::from_millis(1200),
        ..Default::default()
    });
}
