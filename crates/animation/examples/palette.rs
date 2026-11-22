//! Palette playground: a scalar index trajectory sweeps a historical
//! color palette - `Span` walks through, `Steps` hops entry by
//! entry, `Mirror` ping-pongs, `Repeat` cycles. `Smooth` blends
//! neighbors, `Steps` snaps to exact swatches.
//!
//! Run with `cargo run -p smearor-commons-animation --features gtk4 --example palette`.

mod common;

use common::Animatable;
use common::ComponentKnobs;
use common::ComponentPanel;
use common::EASINGS;
use common::Effect;
use common::PhaseShape;
use common::SCALAR_TRAJECTORIES;
use common::apply_spec;
use common::build_scalar_trajectory;
use common::component_knobs;
use common::component_panel;
use common::connect_panel;
use common::dropdown_row;
use common::eased;
use common::easing_by_name;
use common::effect_group;
use common::emit_effect_rust;
use common::group_frame;
use common::install_tick;
use common::load_effect;
use common::paned_view;
use common::playback_group;
use common::save_effect;
use common::select_by_name;
use common::spec_for_panel;
use common::update_panel;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::DrawingArea;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_animation::EasingFunction;
use smearor_commons_animation::TimedAnimation;
use smearor_commons_animation::Trajectory;
use smearor_commons_color::Color;
use smearor_commons_color::PaletteDefinition;
use smearor_commons_color::StaticPalette;
use smearor_commons_geometry::Fraction;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_animation.palette";

/// The blend modes - smooth lerps neighbors, steps snaps.
const BLENDS: &[&str] = &["Smooth", "Steps"];

/// A palette as `(name, color)` entries - the slot enums differ
/// per palette, `entries()` normalizes them.
type PaletteEntries = Vec<(&'static str, Color<u8>)>;

/// Extracts a static palette's `(slot name, color)` pairs.
fn entries<N: PaletteDefinition>(palette: &'static StaticPalette<N, u8>) -> PaletteEntries {
    palette.entries().iter().map(|(slot, color)| (slot.name(), *color)).collect()
}

/// The selectable palettes - the ten signature hues per decade plus
/// a few iconic contemporaries.
fn palettes() -> Vec<(&'static str, PaletteEntries)> {
    use smearor_commons_color as c;
    vec![
        ("Signature 1900s", entries(&c::SIGNATURE_HUES_1900)),
        ("Fauves", entries(&c::FAUVES)),
        ("Signature 1910s", entries(&c::SIGNATURE_HUES_1910)),
        ("Cubism", entries(&c::CUBISM)),
        ("Signature 1920s", entries(&c::SIGNATURE_HUES_1920)),
        ("Bauhaus", entries(&c::BAUHAUS)),
        ("Art Deco", entries(&c::ART_DECO)),
        ("Signature 1930s", entries(&c::SIGNATURE_HUES_1930)),
        ("Signature 1940s", entries(&c::SIGNATURE_HUES_1940)),
        ("Film Noir", entries(&c::FILM_NOIR)),
        ("Signature 1950s", entries(&c::SIGNATURE_HUES_1950)),
        ("Mid-Century Modernists", entries(&c::MID_CENTURY_MODERNISTS)),
        ("Signature 1960s", entries(&c::SIGNATURE_HUES_1960)),
        ("Psychedelia", entries(&c::PSYCHEDELIA)),
        ("Signature 1970s", entries(&c::SIGNATURE_HUES_1970)),
        ("Hotel California", entries(&c::HOTEL_CALIFORNIA)),
        ("Signature 1980s", entries(&c::SIGNATURE_HUES_1980)),
        ("Miami Vice", entries(&c::MIAMI_VICE)),
        ("Memphis", entries(&c::MEMPHIS_MICHAEL_AND_PHILIPPE)),
        ("Signature 1990s", entries(&c::SIGNATURE_HUES_1990)),
        ("Grunge & Graffiti", entries(&c::GRUNGE_AND_GRAFFITI)),
        ("Sesame Street", entries(&c::SESAME_STREET)),
        ("Warhol", entries(&c::WARHOL)),
    ]
}

/// The palette names for the dropdown - built once.
fn palette_titles() -> Vec<&'static str> {
    palettes().iter().map(|(title, _)| *title).collect()
}

/// The animated palette sweep - a scalar index trajectory over the
/// entries, wrapped for overshoot.
struct PaletteDemo {
    /// The selected palette's entries.
    entries: PaletteEntries,
    /// Whether neighbors blend smoothly (`0`) or snap (`1`).
    stepped: bool,
    /// The index animation - `0..=1` maps onto the entry count.
    animation: TimedAnimation<Box<dyn Trajectory<Value = f64>>>,
    /// The segment layout for the phase bar.
    phase: PhaseShape,
    /// The index trajectory's easing.
    easings: Vec<EasingFunction>,
}

impl PaletteDemo {
    /// The wrapped palette position at index progress `u`.
    fn position(&self, u: f64) -> f64 {
        (u * self.entries.len() as f64).rem_euclid(self.entries.len() as f64)
    }

    /// The color at index progress `u` - smooth blends the two
    /// neighbors, steps snaps to the entry below.
    fn color_at(&self, u: f64) -> Color<f64> {
        let n = self.entries.len();
        let position = self.position(u);
        let lower = position as usize % n;
        let base = &self.entries[lower].1;
        let (r, g, b) = (f64::from(base.r), f64::from(base.g), f64::from(base.b));
        let first = Color::<f64>::new(r / 255.0, g / 255.0, b / 255.0);
        if self.stepped {
            return first;
        }
        let upper = (lower + 1) % n;
        let next = &self.entries[upper].1;
        let second = Color::<f64>::new(f64::from(next.r) / 255.0, f64::from(next.g) / 255.0, f64::from(next.b) / 255.0);
        first.lerp(&second, position.fract())
    }

    /// The name of the entry the position sits on.
    fn name_at(&self, u: f64) -> &'static str {
        self.entries[self.position(u) as usize % self.entries.len()].0
    }
}

impl Animatable for PaletteDemo {
    fn is_complete(&self) -> bool {
        self.animation.is_complete()
    }

    fn start(&mut self) {
        self.animation.start();
    }

    fn seek(&mut self, progress: Fraction) {
        self.animation.seek(progress);
    }

    fn progress(&self) -> Option<Fraction> {
        self.animation.progress()
    }

    fn first_easing(&self) -> Option<EasingFunction> {
        self.easings.first().copied()
    }

    fn phase_at(&self, progress: f64) -> (u32, u32) {
        self.phase.phase_at(progress)
    }

    fn phase_fraction(&self, progress: f64) -> f64 {
        self.phase.fraction(progress)
    }
}

/// The global parameters - palette, blend and timing.
struct Knobs {
    duration: Duration,
    palette: u32,
    stepped: bool,
    index: ComponentKnobs,
}

/// Builds the animated sweep.
fn build_demo(knobs: &Knobs, palettes: &[(&'static str, PaletteEntries)]) -> PaletteDemo {
    let name = SCALAR_TRAJECTORIES[knobs.index.trajectory as usize];
    let (trajectory, phase) = build_scalar_trajectory(name, &knobs.index);
    PaletteDemo {
        entries: palettes[knobs.palette as usize].1.clone(),
        stepped: knobs.stepped,
        animation: TimedAnimation::new(eased(trajectory, &knobs.index), knobs.duration, EasingFunction::Linear),
        phase,
        easings: vec![easing_by_name(EASINGS[knobs.index.easing as usize], &knobs.index).0],
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();

        // --- controls -------------------------------------------------

        let controls = GtkBox::new(Orientation::Vertical, 6);
        controls.set_margin_top(12);
        controls.set_margin_bottom(12);
        controls.set_margin_start(12);
        controls.set_margin_end(12);
        controls.set_width_request(320);

        let palettes = palettes();
        let titles = palette_titles();
        let palette_box = GtkBox::new(Orientation::Vertical, 6);
        let (_palette_row, palette) = dropdown_row(&palette_box, "Palette", &titles);
        let (_blend_row, blend) = dropdown_row(&palette_box, "Blend", BLENDS);
        controls.append(&group_frame("Palette", &palette_box));

        // one index column drives the sweep
        let columns = GtkBox::new(Orientation::Horizontal, 4);
        columns.set_homogeneous(true);
        controls.append(&columns);
        let panels: Rc<Vec<ComponentPanel>> = Rc::new((0..1).map(|_| component_panel(&columns)).collect());
        panels[0].title.set_markup("<span size=\"large\" weight=\"bold\">Index</span>");

        let playback = playback_group(&controls);
        let effects = effect_group(&controls);

        // --- state ------------------------------------------------------

        let knobs = Rc::new({
            let (duration, palette, blend) = (playback.duration.clone(), palette.clone(), blend.clone());
            let panels = Rc::clone(&panels);
            move || Knobs {
                duration: Duration::from_secs_f64(duration.value()),
                palette: palette.selected(),
                stepped: blend.selected() == 1,
                index: component_knobs(&panels[0]),
            }
        });

        let demo = Rc::new(RefCell::new(build_demo(&knobs(), &palettes)));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));

        // Visibility + rebuild: only the rows the index trajectory
        // reads stay visible.
        let update_visibility = {
            let panels = Rc::clone(&panels);
            let updating = Cell::new(false);
            let knobs = Rc::clone(&knobs);
            let demo = Rc::clone(&demo);
            let hold = Rc::clone(&hold);
            move || {
                if updating.replace(true) {
                    return;
                }
                let panel = &panels[0];
                let name = SCALAR_TRAJECTORIES[panel.trajectory.selected() as usize];
                let (_, params) = easing_by_name(EASINGS[panel.easing.selected() as usize], &component_knobs(panel));
                update_panel(panel, name, params, false);
                hold.set(None);
                let mut animation = demo.borrow_mut();
                *animation = build_demo(&knobs(), &palettes);
                animation.start();
                updating.set(false);
            }
        };

        let restart: Rc<dyn Fn()> = Rc::new({
            let update_visibility = update_visibility.clone();
            move || update_visibility()
        });
        connect_panel(&panels[0], &restart);
        {
            let restart = Rc::clone(&restart);
            playback.duration.connect_value_changed(move |_| restart());
        }
        for dropdown in [&palette, &blend] {
            let restart = Rc::clone(&restart);
            dropdown.connect_selected_notify(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            playback.replay.connect_clicked(move |_| restart());
        }

        // --- effect: save / load / emit -----------------------------------

        let current_effect = {
            let (palette, blend) = (palette.clone(), blend.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            move || {
                let mut options = std::collections::BTreeMap::new();
                options.insert("blend".to_string(), serde_json::json!(BLENDS[blend.selected() as usize]));
                Effect {
                    demo: "palette".into(),
                    object: palette_titles()[palette.selected() as usize].to_string(),
                    options,
                    duration_secs: duration.value(),
                    components: vec![spec_for_panel("Index", "x", &panels[0])],
                }
            }
        };

        let apply_effect = {
            let (palette, blend) = (palette.clone(), blend.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            let restart = Rc::clone(&restart);
            move |effect: &Effect| {
                if let Some(index) = palette_titles().iter().position(|title| *title == effect.object) {
                    palette.set_selected(index as u32);
                }
                if let Some(name) = effect.options.get("blend").and_then(|v| v.as_str()) {
                    select_by_name(&blend, BLENDS, name);
                }
                duration.set_value(effect.duration_secs);
                for (panel, spec) in panels.iter().zip(effect.components.iter()) {
                    apply_spec(panel, spec);
                }
                restart();
            }
        };

        {
            let current = current_effect.clone();
            let path = effects.path.clone();
            effects.save.connect_clicked(move |_| save_effect(&current(), &path.text()));
        }
        {
            let path = effects.path.clone();
            let apply_effect = apply_effect.clone();
            effects.load.connect_clicked(move |_| {
                if let Some(effect) = load_effect(&path.text()) {
                    apply_effect(&effect);
                }
            });
        }
        {
            let current = current_effect.clone();
            let path = effects.path.clone();
            effects.emit.connect_clicked(move |_| {
                let rust = emit_effect_rust(&current());
                let target = format!("{}.rs", path.text().trim_end_matches(".json"));
                if let Err(error) = std::fs::write(&target, rust) {
                    eprintln!("effect emit failed: {error}");
                }
            });
        }

        // --- frame clock -------------------------------------------------

        install_tick(&canvas, Rc::clone(&demo), &playback, Rc::clone(&hold), |_| {});

        canvas.set_draw_func({
            let demo = Rc::clone(&demo);
            move |_, context, width, height| {
                let (w, h) = (f64::from(width), f64::from(height));
                let margin = 40.0;
                let animation = demo.borrow();
                let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0);
                let u = animation.animation.value().unwrap_or(0.0);
                let color = animation.color_at(u);

                context.set_source_rgb(0.92, 0.92, 0.92);
                let _ = context.paint();

                // the blended swatch on top
                let swatch_h = h * 0.45;
                context.set_source_rgb(color.r, color.g, color.b);
                context.rectangle(margin, margin, w - 2.0 * margin, swatch_h);
                let _ = context.fill();
                context.set_source_rgb(0.75, 0.75, 0.75);
                context.rectangle(margin, margin, w - 2.0 * margin, swatch_h);
                context.set_line_width(1.5);
                let _ = context.stroke();

                // the palette strip - one column per entry, the
                // indicator rides the wrapped position
                let strip_y = h - margin - 90.0;
                let n = animation.entries.len();
                let column_w = (w - 2.0 * margin) / n as f64;
                for (i, (_, entry)) in animation.entries.iter().enumerate() {
                    let (r, g, b) = (f64::from(entry.r) / 255.0, f64::from(entry.g) / 255.0, f64::from(entry.b) / 255.0);
                    context.set_source_rgb(r, g, b);
                    context.rectangle(margin + column_w * i as f64, strip_y, column_w + 1.0, 70.0);
                    let _ = context.fill();
                }
                context.set_source_rgb(0.75, 0.75, 0.75);
                context.rectangle(margin, strip_y, w - 2.0 * margin, 70.0);
                let _ = context.stroke();
                let position = animation.position(u);
                let marker = margin + column_w * (position + 0.5);
                context.set_source_rgb(0.2, 0.2, 0.2);
                context.set_line_width(2.0);
                context.move_to(marker, strip_y - 6.0);
                context.line_to(marker - 6.0, strip_y - 16.0);
                context.line_to(marker + 6.0, strip_y - 16.0);
                context.close_path();
                let _ = context.fill();
                context.move_to(marker, strip_y - 4.0);
                context.line_to(marker, strip_y + 74.0);
                let _ = context.stroke();

                context.set_source_rgb(0.35, 0.35, 0.35);
                context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                context.set_font_size(13.0);
                context.move_to(margin, margin - 14.0);
                let _ = context.show_text(&format!("{}   t {progress:.2}   index {position:.2}", animation.name_at(u)));
            }
        });

        let view = paned_view(&canvas, &controls);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Palette Animation")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        restart();
        window.present();
    });
    app.run()
}
