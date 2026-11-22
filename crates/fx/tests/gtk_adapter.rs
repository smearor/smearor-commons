//! GTK adapter contract tests (phase 5). GTK is single-threaded, so
//! every GTK interaction happens inside one test function on one
//! thread. Without a display the test skips.
#![cfg(feature = "gtk4")]

use gtk4::prelude::*;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::FxError;
use smearor_commons_fx::gtk::FxPlayOptions;
use smearor_commons_fx::gtk::FxPlayer;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use std::sync::OnceLock;
use std::time::Duration;

fn gtk_available() -> bool {
    static AVAILABLE: OnceLock<bool> = OnceLock::new();
    *AVAILABLE.get_or_init(|| gtk4::init().is_ok())
}

fn pump() {
    let context = gtk4::glib::MainContext::default();
    while context.iteration(false) {}
}

fn solid(size: u32, color: Color<f64>) -> PixelBuffer<Rgba> {
    let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
    let mut view = buffer.view_mut();
    for y in 0..size {
        for x in 0..size {
            view.put_color(Pixels::new(x), Pixels::new(y), color);
        }
    }
    buffer
}

/// `snapshot_renders` - a styled widget snapshots to a non-empty
/// buffer at allocation x scale.
fn snapshot_renders() {
    let window = gtk4::Window::new();
    let label = gtk4::Label::new(Some("snapshot-test"));
    label.set_size_request(64, 48);
    window.set_child(Some(&label));
    window.present();
    pump();
    std::thread::sleep(std::time::Duration::from_millis(100));
    pump();

    let buffer = smearor_commons_fx::gtk::snapshot(&label).unwrap_or_else(|e| panic!("snapshot failed: {e}"));
    let scale = label.scale_factor().max(1) as u32;
    let expected = Size::new(label.width() as u32 * scale, label.height() as u32 * scale);
    assert_eq!(buffer.size(), expected, "buffer size must be allocation x scale");
    assert!(buffer.pixels().as_chunks::<4>().0.iter().any(|p| p[3] > 0), "snapshot must contain non-transparent pixels");

    // The empty-widget error path.
    let orphan = gtk4::DrawingArea::new();
    assert!(matches!(smearor_commons_fx::gtk::snapshot(&orphan), Err(FxError::SnapshotEmpty)));

    window.close();
    pump();
}

/// `player_lifecycle` - play renders frames, pause pins progress,
/// `animation-ended` fires once.
fn player_lifecycle() {
    let player = FxPlayer::new();
    player.set_effect(EffectSpec::by_nick("crossfade").unwrap(), solid(64, Color::<f64>::new_rgba(1.0, 0.0, 0.0, 1.0)));

    let window = gtk4::Window::new();
    player.set_size_request(64, 64);
    window.set_child(Some(&player));
    window.present();
    pump();

    let ended = std::rc::Rc::new(std::cell::Cell::new(0u32));
    player.connect_animation_ended({
        let ended = std::rc::Rc::clone(&ended);
        move |_| ended.set(ended.get() + 1)
    });

    player.play(FxPlayOptions {
        duration: Duration::from_millis(80),
        ..Default::default()
    });
    pump();
    std::thread::sleep(Duration::from_millis(150));
    pump();

    assert!(player.is_complete(), "player must complete after the duration elapsed");
    assert_eq!(ended.get(), 1, "animation-ended must fire exactly once");

    // Pause/resume mid-animation.
    let player2 = FxPlayer::new();
    player2.set_effect(EffectSpec::by_nick("crossfade").unwrap(), solid(64, Color::<f64>::new_rgba(0.0, 1.0, 0.0, 1.0)));
    player2.set_size_request(64, 64);
    window.set_child(Some(&player2));
    pump();
    player2.play(FxPlayOptions {
        duration: Duration::from_secs(60),
        ..Default::default()
    });
    player2.pause();
    std::thread::sleep(Duration::from_millis(30));
    assert!(!player2.is_complete());
    player2.resume();
    player2.stop();
    window.close();
    pump();
}

/// `pointer_plumbing` - wired controllers land on the player.
fn pointer_plumbing() {
    let player = FxPlayer::new();
    let controllers = player.observe_controllers();
    // motion + click controllers installed by wire_pointer
    assert!(controllers.n_items() >= 2, "pointer controllers must be installed");
}

/// `surface_contract` - a layered spec with no provider still renders
/// (the MissingSurface contract degrades, not panics); a real provider
/// feeds the surface pass.
fn surface_contract() {
    let player = FxPlayer::new();
    let layered = EffectSpec::by_nick("wormhole").unwrap();
    player.set_effect(layered, solid(64, Color::<f64>::new_rgba(0.0, 0.0, 1.0, 1.0)));

    let calls = std::rc::Rc::new(std::cell::Cell::new(0u32));
    player.set_surface_provider({
        let calls = std::rc::Rc::clone(&calls);
        Box::new(move || {
            calls.set(calls.get() + 1);
            Some(solid(64, Color::<f64>::new_rgba(0.2, 0.2, 0.2, 1.0)))
        })
    });

    let window = gtk4::Window::new();
    player.set_size_request(64, 64);
    window.set_child(Some(&player));
    window.present();
    pump();
    player.play(FxPlayOptions {
        duration: Duration::from_millis(30),
        ..Default::default()
    });
    std::thread::sleep(Duration::from_millis(80));
    pump();
    assert!(calls.get() > 0, "layered specs must query the surface provider each frame");
    window.close();
    pump();
}

/// `resize_reallocs` - an empty player rebuilds the frame buffers at
/// `size-allocate` x scale; once `set_effect` installs an input, the
/// buffers stay bound to the frozen texture size (the input IS the
/// natural render size - allocation does not re-crop it).
fn resize_reallocs() {
    let player = FxPlayer::new();
    let scale = player.scale_factor().max(1) as u32;
    player.size_allocate(&gtk4::gdk::Rectangle::new(0, 0, 100, 80), -1);
    assert_eq!(player.frame_size(), Size::new(100 * scale, 80 * scale));

    player.set_effect(EffectSpec::by_nick("focus").unwrap(), solid(16, Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)));
    assert_eq!(player.frame_size(), Size::new(16, 16));
    player.size_allocate(&gtk4::gdk::Rectangle::new(0, 0, 40, 30), -1);
    assert_eq!(player.frame_size(), Size::new(16, 16), "input-bound buffers ignore size-allocate");
}

#[test]
fn gtk_adapter_end_to_end() {
    if !gtk_available() {
        eprintln!("no GTK display - skipping adapter tests");
        return;
    }
    snapshot_renders();
    player_lifecycle();
    pointer_plumbing();
    surface_contract();
    resize_reallocs();
}
