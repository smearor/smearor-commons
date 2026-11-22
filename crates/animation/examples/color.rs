//! Color playground: each channel of a color space is driven by an
//! independent scalar trajectory. The canvas shows the animated
//! swatch, a gradient strip of the whole trajectory path and the
//! three channel values.
//!
//! Run with `cargo run -p smearor-commons-animation --features gtk4 --example color`.

mod common;

use common::Animatable;
use common::ComponentKnobs;
use common::ComponentPanel;
use common::EASINGS;
use common::Effect;
use common::PhaseShape;
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
use common::selected_name;
use common::slider;
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
use smearor_commons_color::Hsl;
use smearor_commons_color::Hsv;
use smearor_commons_color::Lab;
use smearor_commons_color::Xyz;
use smearor_commons_geometry::Fraction;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_animation.color";

/// The selectable color spaces.
const SPACES: &[&str] = &["sRGB", "HSV", "HSL", "Lab"];

/// The channel titles per space - they relabel the three columns.
fn channel_labels(space: u32) -> [&'static str; 3] {
    match space {
        1 => ["H", "S", "V"],
        2 => ["H", "S", "L"],
        3 => ["L*", "a*", "b*"],
        _ => ["R", "G", "B"],
    }
}

/// The animated color - one timed scalar trajectory per channel.
struct ColorDemo {
    /// The selected color space - indexes `SPACES`.
    space: u32,
    /// The three channel animations.
    channels: Vec<TimedAnimation<Box<dyn Trajectory<Value = f64>>>>,
    /// The constant alpha.
    alpha: f64,
    /// The segment layout for the phase bar - channel 0's shape.
    phase: PhaseShape,
    /// The per-channel easings - bars take their shade.
    easings: Vec<EasingFunction>,
}

impl ColorDemo {
    /// The raw channel values at trajectory progress `t` - easing
    /// wrappers included, overshoot unclamped.
    fn channels_at(&self, t: Fraction) -> [f64; 3] {
        let mut values = [0.0; 3];
        for (channel, value) in self.channels.iter().zip(values.iter_mut()) {
            *value = channel.trajectory.value_at(t);
        }
        values
    }

    /// The color at trajectory progress `t` - hue channels wrap,
    /// the rest clamp into their space's gamut.
    fn color_at(&self, t: Fraction) -> Color<f64> {
        let [a, b, c] = self.channels_at(t);
        match self.space {
            // hue is circular - `Hue::new` wraps into 0..360
            1 => Color::from(Hsv::new(a * 360.0, b.clamp(0.0, 1.0), c.clamp(0.0, 1.0))),
            2 => Color::from(Hsl::new(a * 360.0, b.clamp(0.0, 1.0), c.clamp(0.0, 1.0))),
            // CIELAB via XYZ (D65) - L* 0..100, a*/b* centered on 0
            3 => Color::from(Xyz::from(Lab::new(a.clamp(0.0, 1.0) * 100.0, (b - 0.5) * 256.0, (c - 0.5) * 256.0))),
            _ => Color::<f64>::new(a.clamp(0.0, 1.0), b.clamp(0.0, 1.0), c.clamp(0.0, 1.0)),
        }
    }
}

impl Animatable for ColorDemo {
    fn is_complete(&self) -> bool {
        // the channels share the duration and start together
        self.channels.first().is_none_or(|c| c.is_complete())
    }

    fn start(&mut self) {
        for channel in &mut self.channels {
            channel.start();
        }
    }

    fn seek(&mut self, progress: Fraction) {
        for channel in &mut self.channels {
            channel.seek(progress);
        }
    }

    fn progress(&self) -> Option<Fraction> {
        self.channels.first().and_then(|c| c.progress())
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

/// The global parameters - space, alpha and timing.
struct Knobs {
    duration: Duration,
    space: u32,
    alpha: f64,
    components: Vec<ComponentKnobs>,
}

/// Builds the animated color - one eased, timed scalar trajectory
/// per channel.
fn build_demo(knobs: &Knobs) -> ColorDemo {
    let mut channels = Vec::with_capacity(3);
    let mut phase = PhaseShape::Identity;
    for (slot, component) in knobs.components.iter().take(3).enumerate() {
        let name = common::SCALAR_TRAJECTORIES[component.trajectory as usize];
        let (trajectory, shape) = build_scalar_trajectory(name, component);
        if slot == 0 {
            phase = shape;
        }
        channels.push(TimedAnimation::new(eased(trajectory, component), knobs.duration, EasingFunction::Linear));
    }
    let easings = knobs
        .components
        .iter()
        .take(3)
        .map(|c| easing_by_name(EASINGS[c.easing as usize], c).0)
        .collect();
    ColorDemo {
        space: knobs.space,
        channels,
        alpha: knobs.alpha,
        phase,
        easings,
    }
}

/// The channel bar shades - one per channel column.
const CHANNEL_COLORS: [(f64, f64, f64); 3] = [(0.72, 0.72, 0.86), (0.86, 0.7, 0.66), (0.66, 0.8, 0.7)];

/// A `#rrggbb` string of the color.
fn hex(color: &Color<f64>) -> String {
    format!("#{:02x}{:02x}{:02x}", (color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8)
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

        let space_box = GtkBox::new(Orientation::Vertical, 6);
        let (_space_row, space) = dropdown_row(&space_box, "Space", SPACES);
        let (_alpha_row, alpha) = slider(&space_box, "Alpha", 0.0, 1.0, 1.0, 2);
        for mark in [0.0, 0.5, 1.0] {
            alpha.add_mark(mark, gtk4::PositionType::Bottom, Some(&format!("{mark:.1}")));
        }
        controls.append(&group_frame("Color Space", &space_box));

        // one column per channel - all three stay scalar
        let columns = GtkBox::new(Orientation::Horizontal, 4);
        columns.set_homogeneous(true);
        controls.append(&columns);
        let panels: Rc<Vec<ComponentPanel>> = Rc::new((0..3).map(|_| component_panel(&columns)).collect());

        let playback = playback_group(&controls);
        let effects = effect_group(&controls);

        // --- state ------------------------------------------------------

        let knobs = Rc::new({
            let (duration, space, alpha) = (playback.duration.clone(), space.clone(), alpha.clone());
            let panels = Rc::clone(&panels);
            move || Knobs {
                duration: Duration::from_secs_f64(duration.value()),
                space: space.selected(),
                alpha: alpha.value(),
                components: panels.iter().map(component_knobs).collect(),
            }
        });

        let demo = Rc::new(RefCell::new(build_demo(&knobs())));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));

        // Visibility + rebuild: channel labels follow the space, all
        // channels keep the scalar trajectory model.
        let update_visibility = {
            let panels = Rc::clone(&panels);
            let space = space.clone();
            let updating = Cell::new(false);
            let knobs = Rc::clone(&knobs);
            let demo = Rc::clone(&demo);
            let hold = Rc::clone(&hold);
            move || {
                if updating.replace(true) {
                    return;
                }
                for (i, panel) in panels.iter().enumerate() {
                    panel.column.set_visible(true);
                    let label = channel_labels(space.selected())[i];
                    panel.title.set_markup(&format!("<span size=\"large\" weight=\"bold\">{label}</span>"));
                    let name = common::SCALAR_TRAJECTORIES[panel.trajectory.selected() as usize];
                    let (_, params) = easing_by_name(EASINGS[panel.easing.selected() as usize], &component_knobs(panel));
                    update_panel(panel, name, params, false);
                }
                hold.set(None);
                let mut animation = demo.borrow_mut();
                *animation = build_demo(&knobs());
                animation.start();
                updating.set(false);
            }
        };

        let restart: Rc<dyn Fn()> = Rc::new({
            let update_visibility = update_visibility.clone();
            move || update_visibility()
        });
        for panel in panels.iter() {
            connect_panel(panel, &restart);
        }
        for scale in [&playback.duration, &alpha] {
            let restart = Rc::clone(&restart);
            scale.connect_value_changed(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            space.connect_selected_notify(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            playback.replay.connect_clicked(move |_| restart());
        }

        // --- effect: save / load / emit -----------------------------------

        let current_effect = {
            let (space, alpha) = (space.clone(), alpha.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            move || {
                let mut options = std::collections::BTreeMap::new();
                options.insert("alpha".to_string(), serde_json::json!(alpha.value()));
                Effect {
                    demo: "color".into(),
                    object: selected_name(&space, SPACES),
                    options,
                    duration_secs: duration.value(),
                    components: panels
                        .iter()
                        .enumerate()
                        .map(|(i, panel)| spec_for_panel(channel_labels(space.selected())[i], "x", panel))
                        .collect(),
                }
            }
        };

        let apply_effect = {
            let (space, alpha) = (space.clone(), alpha.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            let restart = Rc::clone(&restart);
            move |effect: &Effect| {
                select_by_name(&space, SPACES, &effect.object);
                if let Some(value) = effect.options.get("alpha").and_then(|v| v.as_f64()) {
                    alpha.set_value(value);
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
                let color = animation.color_at(Fraction::new(progress));

                context.set_source_rgb(0.92, 0.92, 0.92);
                let _ = context.paint();

                // the animated swatch - left two thirds of the plot
                let swatch_h = h * 0.55;
                context.set_source_rgba(color.r, color.g, color.b, animation.alpha);
                context.rectangle(margin, margin, w * 0.62 - margin, swatch_h);
                let _ = context.fill();
                context.set_source_rgb(0.75, 0.75, 0.75);
                context.rectangle(margin, margin, w * 0.62 - margin, swatch_h);
                context.set_line_width(1.5);
                let _ = context.stroke();

                // the channel bars - one per column, in its shade
                let labels = channel_labels(animation.space);
                let values = animation.channels_at(Fraction::new(progress));
                let bar_x = w * 0.68;
                let bar_w = w - bar_x - margin;
                context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
                    let y = margin + i as f64 * 64.0;
                    let (r, g, b) = CHANNEL_COLORS[i % CHANNEL_COLORS.len()];
                    context.set_source_rgb(0.85, 0.85, 0.85);
                    context.rectangle(bar_x, y, bar_w, 20.0);
                    let _ = context.fill();
                    context.set_source_rgb(r, g, b);
                    context.rectangle(bar_x, y, bar_w * value.clamp(0.0, 1.0), 20.0);
                    let _ = context.fill();
                    context.set_source_rgb(0.35, 0.35, 0.35);
                    context.set_font_size(12.0);
                    context.move_to(bar_x, y + 36.0);
                    let _ = context.show_text(&format!("{label} {value:.3}"));
                }

                // the trajectory's gradient strip - the whole path
                // the swatch travels, with the current position
                let strip_y = h - margin - 64.0;
                const SEGMENTS: usize = 96;
                for i in 0..SEGMENTS {
                    let t = Fraction::new(i as f64 / (SEGMENTS - 1) as f64);
                    let c = animation.color_at(t);
                    context.set_source_rgb(c.r, c.g, c.b);
                    context.rectangle(
                        margin + (w - 2.0 * margin) * i as f64 / SEGMENTS as f64,
                        strip_y,
                        (w - 2.0 * margin) / SEGMENTS as f64 + 1.0,
                        64.0,
                    );
                    let _ = context.fill();
                }
                context.set_source_rgb(0.75, 0.75, 0.75);
                context.rectangle(margin, strip_y, w - 2.0 * margin, 64.0);
                let _ = context.stroke();
                let marker = margin + (w - 2.0 * margin) * progress;
                context.set_source_rgb(0.2, 0.2, 0.2);
                context.set_line_width(2.0);
                context.move_to(marker, strip_y - 4.0);
                context.line_to(marker, strip_y + 68.0);
                let _ = context.stroke();

                context.set_source_rgb(0.35, 0.35, 0.35);
                context.set_font_size(13.0);
                context.move_to(margin, margin - 14.0);
                let _ = context.show_text(&format!("{}   {}   t {progress:.2}", SPACES[animation.space as usize], hex(&color)));
            }
        });

        let view = paned_view(&canvas, &controls);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Color Animation")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        restart();
        window.present();
    });
    app.run()
}
