//! The experimenter's JSON contract, tested headlessly: the
//! `{"demo":"fx","object":nick,"options":{...}}` envelope resolves
//! through `by_nick` + serde, and a fixed seed replays a frame
//! bitwise - the two properties Save/Load rely on.

use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_fx::Direction;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::FxContext;
use smearor_commons_fx::Layer;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Size;
use smearor_commons_noise::Seed;
use std::time::Duration;

/// A gradient card - enough pixel variety for an effect to read.
fn card(size: Size<u32>) -> PixelBuffer<Rgba> {
    let mut card = PixelBuffer::<Rgba>::new(size);
    let mut view = card.view_mut();
    for y in 0..size.y {
        for x in 0..size.x {
            view.put_color(
                Pixels::new(x),
                Pixels::new(y),
                smearor_commons_color::Color::<f64>::new_rgba(f64::from(x) / f64::from(size.x), f64::from(y) / f64::from(size.y), 0.5, 1.0),
            );
        }
    }
    card
}

/// Renders `spec` at `progress`/`seed` - the experimenter's frame.
fn render(spec: &EffectSpec, card: &PixelBuffer<Rgba>, progress: f64, seed: u64) -> PixelBuffer<Rgba> {
    render_dir(spec, card, progress, seed, Direction::Open)
}

/// `render` with a chosen direction.
fn render_dir(spec: &EffectSpec, card: &PixelBuffer<Rgba>, progress: f64, seed: u64, direction: Direction) -> PixelBuffer<Rgba> {
    let size = card.size();
    let ctx = FxContext {
        progress: Fraction::new(progress),
        duration: Duration::from_secs_f64(1.6),
        seed: Seed(seed),
        direction,
        layer: Layer::Window,
        window: card.view(),
        surface: None,
        assets: smearor_commons_fx::FxAssets::default(),
        pointer: None,
        fullscreen: false,
        padding: Pixels::new(0.0),
        window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
    };
    let mut out = PixelBuffer::<Rgba>::new(size);
    spec.build().apply(&ctx, &mut out.view_mut());
    out
}

#[test]
fn envelope_resolves_object_and_options() {
    // a saved "fire" envelope - the Load algorithm: nick -> by_nick,
    // options -> the variant's serde payload
    let envelope = r#"{
        "demo": "fx",
        "object": "fire",
        "options": {
            "scale": 2.5,
            "noise_3d": true,
            "direction": "close",
            "seed": 9001
        },
        "duration_secs": 1.5,
        "components": []
    }"#;
    let envelope: serde_json::Value = serde_json::from_str(envelope).unwrap();
    let object = envelope["object"].as_str().unwrap();
    assert!(EffectSpec::by_nick(object).is_ok());

    let options = envelope["options"].as_object().unwrap();
    let mut payload = serde_json::Map::new();
    payload.insert("effect".to_string(), serde_json::Value::from(object));
    for (key, value) in options {
        if !matches!(key.as_str(), "direction" | "seed") {
            payload.insert(key.clone(), value.clone());
        }
    }
    let spec: EffectSpec = serde_json::from_value(serde_json::Value::Object(payload)).unwrap();
    let EffectSpec::Fire(fire) = spec else { panic!("expected fire") };
    assert_eq!(fire.scale.as_f64(), 2.5);
    assert!(fire.noise_3d);
    assert_eq!(envelope["options"]["direction"], "close");
    assert_eq!(envelope["options"]["seed"], 9001);
}

#[test]
fn every_spec_lands_in_the_envelope() {
    // the Save direction: serde payload minus the `"effect"` tag is
    // the `options` map; restoring the tag round-trips
    for spec in EffectSpec::all() {
        let payload = serde_json::to_value(&spec).unwrap();
        let mut options = payload.as_object().unwrap().clone();
        assert_eq!(options.remove("effect").and_then(|tag| tag.as_str().map(str::to_string)).as_deref(), Some(spec.nick()));
        let mut restored = options.clone();
        restored.insert("effect".to_string(), serde_json::Value::from(spec.nick()));
        let back: EffectSpec = serde_json::from_value(serde_json::Value::Object(restored)).unwrap();
        let again = serde_json::to_value(&back).unwrap();
        assert_eq!(again, payload, "{} drifted", spec.nick());
    }
}

#[test]
fn seed_replays_the_frame_bitwise() {
    let card = card(Size::new(64, 48));
    for spec in EffectSpec::all() {
        let first = render(&spec, &card, 0.5, 9001);
        let second = render(&spec, &card, 0.5, 9001);
        assert_eq!(first.pixels(), second.pixels(), "{} must replay deterministically", spec.nick());
    }
}

/// Mid-animation every spec must leave the window visible or
/// visibly altered somewhere - catches silently-empty shaders. Some
/// effects touch only a thin moving band (e.g. incinerate's scorch
/// zone), so several progress points are sampled.
#[test]
fn every_spec_renders_something_mid_animation() {
    let card = card(Size::new(64, 48));
    let untouched = card.pixels().to_vec();
    for spec in EffectSpec::all() {
        let changed = [0.25, 0.5, 0.75]
            .iter()
            .any(|&p| render(&spec, &card, p, 9001).pixels().iter().zip(untouched.iter()).any(|(a, b)| a != b));
        assert!(changed, "{} rendered a bit-identical frame at every mid progress", spec.nick());
    }
}

#[test]
fn seed_changes_the_frame() {
    // Close at mid-progress is the active burn phase - the
    // seed-offset noise field actually reaches the pixels (for Open
    // the effect mask is already zero there).
    let card = card(Size::new(64, 48));
    let spec = EffectSpec::by_nick("fire").unwrap();
    let one = render_dir(&spec, &card, 0.5, 1_234_567, Direction::Close);
    let two = render_dir(&spec, &card, 0.5, 7_654_321, Direction::Close);
    assert_ne!(one.pixels(), two.pixels(), "different seeds must de-correlate the noise field");
}
