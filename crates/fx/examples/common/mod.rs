//! Shared experimenter scaffolding for the fx example - widget
//! helpers, the metadata-driven parameter panel, the playback group
//! plus frame clock, the rendered test card and the serializable
//! `Effect` envelope. Mirrors `crates/animation/examples/common`,
//! minus the trajectory machinery (fx parameters come from
//! `ParamSpec`, not from `ComponentKnobs`).

use gtk4::Box as GtkBox;
use gtk4::DropDown;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::StringList;
use gtk4::glib;
use gtk4::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_animation::EasingFunction;
use smearor_commons_color::Color;
use smearor_commons_fx::Direction;
use smearor_commons_fx::EffectSpec;
use smearor_commons_fx::ParamKind;
use smearor_commons_fx::ParamSpec;
use smearor_commons_fx::ParamValue;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

// ------------------------------------------------------------------
// Widget rows - one labeled control per row.
// ------------------------------------------------------------------

/// A labeled horizontal slider row for the sidebar.
pub fn slider(parent: &GtkBox, label: &str, min: f64, max: f64, value: f64, digits: i32) -> (GtkBox, Scale) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let scale = Scale::with_range(Orientation::Horizontal, min, max, 0.01);
    scale.set_hexpand(true);
    scale.set_digits(digits);
    scale.set_value(value);
    row.append(&text);
    row.append(&scale);
    parent.append(&row);
    (row, scale)
}

/// A labeled dropdown row for the sidebar - same shape as `slider`.
pub fn dropdown_row(parent: &GtkBox, label: &str, items: &[&str]) -> (GtkBox, DropDown) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let dropdown = DropDown::new(Some(StringList::new(items)), gtk4::Expression::NONE);
    dropdown.set_hexpand(true);
    row.append(&text);
    row.append(&dropdown);
    parent.append(&row);
    (row, dropdown)
}

/// A labeled check-button row for `ParamKind::Toggle`.
pub fn check_row(parent: &GtkBox, label: &str, default: bool) -> (GtkBox, gtk4::CheckButton) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let check = gtk4::CheckButton::new();
    check.set_active(default);
    row.append(&text);
    row.append(&check);
    parent.append(&row);
    (row, check)
}

/// The `Color<f64>` channels as a `gdk::RGBA`.
pub fn to_rgba(color: &Color<f64>) -> gtk4::gdk::RGBA {
    gtk4::gdk::RGBA::new(color.r as f32, color.g as f32, color.b as f32, color.a as f32)
}

/// A `gdk::RGBA` back as normalized `Color<f64>`.
pub fn from_rgba(rgba: &gtk4::gdk::RGBA) -> Color<f64> {
    Color::<f64>::new_rgba(f64::from(rgba.red()), f64::from(rgba.green()), f64::from(rgba.blue()), f64::from(rgba.alpha()))
}

/// A labeled color-picker row for `ParamKind::Color` - the gtk4
/// `ColorDialogButton` is the whole control.
pub fn color_row(parent: &GtkBox, label: &str, default: Color<f64>) -> (GtkBox, gtk4::ColorDialogButton) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let dialog = gtk4::ColorDialog::new();
    let button = gtk4::ColorDialogButton::new(Some(dialog));
    button.set_rgba(&to_rgba(&default));
    button.set_hexpand(true);
    row.append(&text);
    row.append(&button);
    parent.append(&row);
    (row, button)
}

/// A labeled integer spin row plus a dice button that re-rolls the
/// seed (BMW's `Math.random()` uniform).
pub fn seed_row(parent: &GtkBox, label: &str, value: u64) -> (GtkBox, gtk4::SpinButton, gtk4::Button) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let spin = gtk4::SpinButton::with_range(0.0, u32::MAX as f64, 1.0);
    spin.set_digits(0);
    spin.set_value(value as f64);
    spin.set_hexpand(true);
    let dice = gtk4::Button::with_label("Dice");
    row.append(&text);
    row.append(&spin);
    row.append(&dice);
    parent.append(&row);
    (row, spin, dice)
}

/// A framed section with a bold, larger title and padded content.
pub fn group_frame(title: &str, child: &GtkBox) -> gtk4::Frame {
    let frame = gtk4::Frame::new(None);
    let label = Label::new(None);
    label.set_markup(&format!("<span size=\"large\" weight=\"bold\">{title}</span>"));
    frame.set_label_widget(Some(&label));
    child.set_margin_top(8);
    child.set_margin_bottom(8);
    child.set_margin_start(8);
    child.set_margin_end(8);
    frame.set_child(Some(child));
    frame.set_vexpand(true);
    frame
}

/// The experimenter layout: the rendered canvas left, the scrolled
/// control column right.
pub fn paned_view(canvas: &impl IsA<gtk4::Widget>, controls: &GtkBox) -> gtk4::Paned {
    let view = gtk4::Paned::new(Orientation::Horizontal);
    view.set_wide_handle(true);
    view.set_start_child(Some(canvas));
    let scroller = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(controls)
        .build();
    view.set_end_child(Some(&scroller));
    view.set_resize_start_child(true);
    view.set_resize_end_child(false);
    view.set_shrink_start_child(true);
    view.set_shrink_end_child(true);
    view
}

// ------------------------------------------------------------------
// The metadata-driven parameter panel: `ParamSpec` -> widget, and
// back to `ParamValue`. Row order is `params()` order - it is the
// runtime twin `values()`/`set_values()` speaks.
// ------------------------------------------------------------------

/// The slider's display digits implied by a `step` - `0.01` reads
/// two decimals, `0.5` one, `1.0` none.
fn step_digits(step: f64) -> i32 {
    ((-step.log10()).ceil().max(0.0) as i32).min(6)
}

/// One generated parameter row's widget - a `ParamValue` source and
/// sink, and a change-listener target like the builtin widgets.
pub enum ParamWidget {
    /// `ParamKind::Float` - a `Scale`.
    Float(Scale),
    /// `ParamKind::Int` - a `SpinButton`.
    Int(gtk4::SpinButton),
    /// `ParamKind::Toggle` - a `CheckButton`.
    Toggle(gtk4::CheckButton),
    /// `ParamKind::Color` - a `ColorDialogButton`.
    Color(gtk4::ColorDialogButton),
    /// `ParamKind::Choice` - a `DropDown` (index = choice index).
    Choice(DropDown),
}

impl ParamWidget {
    /// The widget's current value as a `ParamValue` - the input to
    /// `EffectSpec::from_params`.
    pub fn value(&self) -> ParamValue {
        match self {
            Self::Float(scale) => ParamValue::Float(scale.value()),
            Self::Int(spin) => ParamValue::Int(spin.value() as i64),
            Self::Toggle(check) => ParamValue::Bool(check.is_active()),
            Self::Color(button) => ParamValue::Color(from_rgba(&button.rgba())),
            Self::Choice(dropdown) => ParamValue::Choice(dropdown.selected() as usize),
        }
    }

    /// Writes a `ParamValue` back - mismatched kinds are ignored so
    /// a stale values list from another effect cannot scramble the
    /// panel.
    pub fn set_value(&self, value: &ParamValue) {
        match (self, value) {
            (Self::Float(scale), ParamValue::Float(v)) => scale.set_value(*v),
            (Self::Int(spin), ParamValue::Int(v)) => spin.set_value(*v as f64),
            (Self::Toggle(check), ParamValue::Bool(v)) => check.set_active(*v),
            (Self::Color(button), ParamValue::Color(c)) => button.set_rgba(&to_rgba(c)),
            (Self::Choice(dropdown), ParamValue::Choice(v)) => dropdown.set_selected(*v as u32),
            _ => {}
        }
    }

    /// Registers a change listener - the `connect_panel` pattern:
    /// every widget fires the same re-read callback.
    pub fn connect_changed(&self, listener: impl Fn() + 'static) {
        let listener = Rc::new(listener);
        match self {
            Self::Float(scale) => {
                scale.connect_change_value(move |_, _, _| {
                    listener();
                    glib::Propagation::Proceed
                });
            }
            Self::Int(spin) => {
                spin.connect_value_changed(move |_| listener());
            }
            Self::Toggle(check) => {
                check.connect_toggled(move |_| listener());
            }
            Self::Color(button) => {
                button.connect_rgba_notify(move |_| listener());
            }
            Self::Choice(dropdown) => {
                dropdown.connect_selected_notify(move |_| listener());
            }
        }
    }
}

/// The collected handles of one parameter panel - rows in
/// `params()` order, for re-read and Load write-back.
pub struct ParamWidgets {
    /// One widget per `ParamSpec`, same order.
    pub widgets: Vec<ParamWidget>,
}

impl ParamWidgets {
    /// Re-reads every row in panel order - the runtime twin of
    /// `WindowEffect::params`.
    pub fn values(&self) -> Vec<ParamValue> {
        self.widgets.iter().map(ParamWidget::value).collect()
    }

    /// Writes a `params()`-ordered value list back onto the widgets -
    /// what Load restores. Missing/mismatched entries are skipped.
    pub fn set_values(&self, values: &[ParamValue]) {
        for (widget, value) in self.widgets.iter().zip(values.iter()) {
            widget.set_value(value);
        }
    }
}

/// Generates one widget row per `ParamSpec` - the `ParamKind` ->
/// widget mapping of the experimenter grammar.
pub fn param_row(parent: &GtkBox, spec: &ParamSpec) -> ParamWidget {
    match &spec.kind {
        ParamKind::Float { min, max, step, default } => {
            let (_, scale) = slider(parent, spec.label, *min, *max, *default, step_digits(*step));
            scale.set_increments(*step, *step * 10.0);
            ParamWidget::Float(scale)
        }
        ParamKind::Int { min, max, default } => {
            let row = GtkBox::new(Orientation::Horizontal, 8);
            let text = Label::new(Some(spec.label));
            text.set_width_chars(11);
            text.set_halign(gtk4::Align::Start);
            let spin = gtk4::SpinButton::with_range(*min as f64, *max as f64, 1.0);
            spin.set_digits(0);
            spin.set_value(*default as f64);
            spin.set_hexpand(true);
            row.append(&text);
            row.append(&spin);
            parent.append(&row);
            ParamWidget::Int(spin)
        }
        ParamKind::Toggle { default } => {
            let (_, check) = check_row(parent, spec.label, *default);
            ParamWidget::Toggle(check)
        }
        ParamKind::Color { default } => {
            let (_, button) = color_row(parent, spec.label, *default);
            ParamWidget::Color(button)
        }
        ParamKind::Choice { options, default } => {
            let (_, dropdown) = dropdown_row(parent, spec.label, options);
            dropdown.set_selected(*default as u32);
            ParamWidget::Choice(dropdown)
        }
    }
}

/// Builds the whole panel for `specs` into `parent` - one row per
/// parameter, widgets collected in order.
pub fn param_panel(parent: &GtkBox, specs: &'static [ParamSpec]) -> ParamWidgets {
    let widgets = specs.iter().map(|spec| param_row(parent, spec)).collect();
    ParamWidgets { widgets }
}

// ------------------------------------------------------------------
// Playback - the timing widgets every experimenter shares.
// ------------------------------------------------------------------

/// The timing widgets every experimenter shares: duration and pause
/// sliders, the loop toggle, play/pause/replay buttons plus the two
/// progress bars and the phase counter.
pub struct Playback {
    /// The run duration in seconds.
    pub duration: Scale,
    /// The loop breather in seconds.
    pub pause: Scale,
    /// Whether completed runs restart.
    pub looping: gtk4::CheckButton,
    /// Restarts the animation.
    pub replay: gtk4::Button,
    /// Resumes a paused animation.
    pub play: gtk4::Button,
    /// Freezes the animation at its current progress.
    pub pause_button: gtk4::Button,
    /// The run progress - dragging it seeks the animation.
    pub total: Scale,
    /// The eased sub-progress bar.
    pub phase_bar: gtk4::ProgressBar,
    /// The `Phase n / m` caption.
    pub phase_number: Label,
}

/// Appends the framed Playback group to `controls`.
pub fn playback_group(controls: &GtkBox) -> Playback {
    let playback_box = GtkBox::new(Orientation::Vertical, 6);
    let (_duration_row, duration) = slider(&playback_box, "Duration", 0.25, 6.0, 1.6, 2);
    let (_pause_row, pause) = slider(&playback_box, "Pause", 0.0, 3.0, 0.5, 2);
    let looping = gtk4::CheckButton::with_label("Loop");
    looping.set_active(true);
    playback_box.append(&looping);
    let buttons = GtkBox::new(Orientation::Horizontal, 8);
    buttons.set_homogeneous(true);
    let play = gtk4::Button::with_label("Play");
    let pause_button = gtk4::Button::with_label("Pause");
    let replay = gtk4::Button::with_label("Replay");
    buttons.append(&play);
    buttons.append(&pause_button);
    buttons.append(&replay);
    playback_box.append(&buttons);
    let total_label = Label::new(Some("Total"));
    total_label.set_halign(gtk4::Align::Start);
    playback_box.append(&total_label);
    let total = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.005);
    total.add_mark(0.0, gtk4::PositionType::Bottom, Some("0"));
    total.add_mark(1.0, gtk4::PositionType::Bottom, Some("1"));
    playback_box.append(&total);
    let phase_label = Label::new(Some("Phase"));
    phase_label.set_halign(gtk4::Align::Start);
    playback_box.append(&phase_label);
    let phase_bar = gtk4::ProgressBar::new();
    playback_box.append(&phase_bar);
    let phase_number = Label::new(Some("Phase 1 / 1"));
    phase_number.set_halign(gtk4::Align::Center);
    playback_box.append(&phase_number);
    controls.append(&group_frame("Playback", &playback_box));
    Playback {
        duration,
        pause,
        looping,
        replay,
        play,
        pause_button,
        total,
        phase_bar,
        phase_number,
    }
}

/// What the frame clock needs from an example's animation - the
/// `TimedAnimation` plus the phase layout the bars subdivide by.
pub trait Animatable {
    /// Whether the animation ran through.
    fn is_complete(&self) -> bool;
    /// Restarts the clock.
    fn start(&mut self);
    /// Pins the clock at `progress` - seek for scrubbing; called
    /// every frame it is what pausing looks like.
    fn seek(&mut self, progress: Fraction);
    /// The linear run progress, `None` before the first start.
    fn progress(&self) -> Option<Fraction>;
    /// The first component's easing - the phase bar tracks what
    /// that component's trajectory sees.
    fn first_easing(&self) -> Option<EasingFunction>;
    /// The current trajectory segment as `(index, count)`.
    fn phase_at(&self, progress: f64) -> (u32, u32);
    /// Progress within the current trajectory segment.
    fn phase_fraction(&self, progress: f64) -> f64;
}

/// Installs the shared frame clock on a widget: drives the demo,
/// handles the loop-with-pause restarts and feeds both progress
/// bars. `on_sample` runs after each update with mutable access -
/// render the current frame there.
pub fn install_tick<A: Animatable + 'static>(
    canvas: &gtk4::Widget,
    demo: Rc<RefCell<A>>,
    playback: &Playback,
    hold: Rc<Cell<Option<std::time::Instant>>>,
    on_sample: impl Fn(&mut A) + 'static,
) {
    let (pause, looping) = (playback.pause.clone(), playback.looping.clone());
    let (total, phase_bar, phase_number) = (playback.total.clone(), playback.phase_bar.clone(), playback.phase_number.clone());

    // paused holds the pinned progress - `None` means running
    let paused = Rc::new(Cell::new(None::<f64>));
    // while the user drags the Total scale the tick does not
    // write the thumb back
    let scrubbing = Rc::new(Cell::new(false));

    playback.play.connect_clicked({
        let paused = Rc::clone(&paused);
        move |_| paused.set(None)
    });
    playback.pause_button.connect_clicked({
        let paused = Rc::clone(&paused);
        let demo = Rc::clone(&demo);
        move |_| {
            let u = demo.borrow().progress().map(|p| p.as_f64()).unwrap_or(0.0).min(1.0);
            paused.set(Some(u));
        }
    });
    playback.replay.connect_clicked({
        let paused = Rc::clone(&paused);
        let demo = Rc::clone(&demo);
        move |_| {
            paused.set(None);
            demo.borrow_mut().start();
        }
    });
    let drag = gtk4::GestureDrag::new();
    drag.connect_drag_begin({
        let scrubbing = Rc::clone(&scrubbing);
        move |_, _, _| scrubbing.set(true)
    });
    drag.connect_drag_end({
        let scrubbing = Rc::clone(&scrubbing);
        move |_, _, _| scrubbing.set(false)
    });
    total.add_controller(drag);
    total.connect_change_value({
        let paused = Rc::clone(&paused);
        let demo = Rc::clone(&demo);
        move |_, _, value| {
            let u = value.clamp(0.0, 1.0);
            demo.borrow_mut().seek(Fraction::new(u));
            if paused.get().is_some() {
                paused.set(Some(u));
            }
            glib::Propagation::Proceed
        }
    });

    let last_phase = Rc::new(Cell::new((u32::MAX, u32::MAX)));
    canvas.add_tick_callback(move |canvas, _clock| {
        {
            let mut animation = demo.borrow_mut();
            if let Some(u) = paused.get() {
                // pinned clock - the animation stands still here
                animation.seek(Fraction::new(u));
                hold.set(None);
            } else if animation.is_complete() && looping.is_active() {
                // a breather between the end and the restart
                match hold.get() {
                    None => hold.set(Some(std::time::Instant::now())),
                    Some(at) if at.elapsed() >= Duration::from_secs_f64(pause.value()) => {
                        animation.start();
                        hold.set(None);
                    }
                    _ => {}
                }
            }
            on_sample(&mut animation);
            let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
            if !scrubbing.get() {
                total.set_value(progress);
            }
            let eased = animation.first_easing().map(|e| e.apply(Fraction::new(progress)).as_f64()).unwrap_or(progress);
            if hold.get().is_some() {
                phase_bar.pulse();
            } else {
                phase_bar.set_fraction(animation.phase_fraction(eased).clamp(0.0, 1.0));
            }
            let phase = animation.phase_at(eased);
            if phase != last_phase.get() {
                last_phase.set(phase);
                phase_number.set_text(&format!("Phase {} / {}", phase.0, phase.1));
            }
        }
        canvas.queue_draw();
        glib::ControlFlow::Continue
    });
}

// ------------------------------------------------------------------
// The Effect envelope - a serializable snapshot of everything the
// experimenter is configured with. `demo` is `"fx"`, `object` the
// effect nick, `options` the serde-flattened `EffectSpec` payload
// plus `direction`/`seed`; `components` stays empty (trajectory
// machinery does not apply here).
// ------------------------------------------------------------------

/// One component column of an effect - unused by fx but part of the
/// shared envelope grammar.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentSpec {
    /// The column title.
    pub title: String,
    /// The trajectory role.
    pub role: String,
    /// The selected trajectory name.
    pub trajectory: String,
    /// The selected easing name.
    pub easing: String,
}

/// A complete, serializable effect envelope: the demo it came from,
/// the object it applies to plus its options.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Effect {
    /// Which example created it - `"fx"` here.
    pub demo: String,
    /// The effect nick: `fire`, `glide`, ...
    pub object: String,
    /// The serde-flattened `EffectSpec` payload plus `direction`
    /// and `seed` so a saved effect replays deterministically.
    pub options: std::collections::BTreeMap<String, serde_json::Value>,
    /// The run duration in seconds.
    pub duration_secs: f64,
    /// Unused by fx - kept for envelope parity with the animation
    /// experimenters.
    pub components: Vec<ComponentSpec>,
}

/// The options map for a spec plus context inputs - `object` is the
/// nick, `direction`/`seed` ride in `options` next to the spec's own
/// serde fields.
pub fn effect_options(spec: &EffectSpec, direction: Direction, seed: u64) -> std::collections::BTreeMap<String, serde_json::Value> {
    let mut options = std::collections::BTreeMap::new();
    if let Ok(serde_json::Value::Object(payload)) = serde_json::to_value(spec) {
        for (key, value) in payload {
            if key != "effect" {
                options.insert(key, value);
            }
        }
    }
    options.insert("direction".to_string(), serde_json::to_value(direction).unwrap_or(serde_json::Value::Null));
    options.insert("seed".to_string(), serde_json::Value::from(seed));
    options
}

/// Rebuilds the `EffectSpec` payload from an envelope - `object`
/// becomes the `"effect"` tag, `direction`/`seed` are peeled off as
/// context inputs. Returns the resolved spec plus context.
pub fn spec_from_envelope(effect: &Effect) -> Option<(EffectSpec, Direction, u64)> {
    let mut payload = serde_json::Map::new();
    payload.insert("effect".to_string(), serde_json::Value::from(effect.object.clone()));
    let mut direction = Direction::default();
    let mut seed = 0_u64;
    for (key, value) in &effect.options {
        match key.as_str() {
            "direction" => {
                direction = serde_json::from_value(value.clone()).unwrap_or_default();
            }
            "seed" => {
                seed = value.as_u64().unwrap_or(0);
            }
            _ => {
                payload.insert(key.clone(), value.clone());
            }
        }
    }
    serde_json::from_value::<EffectSpec>(serde_json::Value::Object(payload))
        .ok()
        .map(|spec| (spec, direction, seed))
}

/// Selects a dropdown entry by name - unknown names keep the
/// current selection.
pub fn select_by_name(dropdown: &DropDown, items: &[&str], name: &str) {
    if let Some(index) = items.iter().position(|item| *item == name) {
        dropdown.set_selected(index as u32);
    }
}

/// The file widgets of the Effect group: a path entry plus the
/// save / load / emit buttons.
pub struct EffectWidgets {
    /// The effect file path, default `effect.json`.
    pub path: gtk4::Entry,
    /// Saves the current configuration as JSON.
    pub save: gtk4::Button,
    /// Loads a JSON effect back onto the widgets.
    pub load: gtk4::Button,
    /// Emits the current configuration as Rust code next to the
    /// JSON path (`<name>.rs`).
    pub emit: gtk4::Button,
}

/// Appends the framed Effect group to `controls`.
pub fn effect_group(controls: &GtkBox) -> EffectWidgets {
    let effect_box = GtkBox::new(Orientation::Vertical, 6);
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some("File"));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let path = gtk4::Entry::new();
    path.set_text("effect.json");
    path.set_hexpand(true);
    row.append(&text);
    row.append(&path);
    effect_box.append(&row);
    let buttons = GtkBox::new(Orientation::Horizontal, 8);
    buttons.set_homogeneous(true);
    let save = gtk4::Button::with_label("Save");
    let load = gtk4::Button::with_label("Load");
    let emit = gtk4::Button::with_label("Emit Rust");
    buttons.append(&save);
    buttons.append(&load);
    buttons.append(&emit);
    effect_box.append(&buttons);
    controls.append(&group_frame("Effect", &effect_box));
    EffectWidgets { path, save, load, emit }
}

/// Serializes the effect envelope to pretty JSON at `path`.
pub fn save_effect(effect: &Effect, path: &str) {
    match serde_json::to_string_pretty(effect) {
        Ok(json) => {
            if let Err(error) = std::fs::write(path, json) {
                eprintln!("effect save failed: {error}");
            }
        }
        Err(error) => eprintln!("effect serialize failed: {error}"),
    }
}

/// Reads an effect envelope from a JSON file.
pub fn load_effect(path: &str) -> Option<Effect> {
    match std::fs::read_to_string(path) {
        Ok(json) => match serde_json::from_str(&json) {
            Ok(effect) => Some(effect),
            Err(error) => {
                eprintln!("effect parse failed: {error}");
                None
            }
        },
        Err(error) => {
            eprintln!("effect read failed: {error}");
            None
        }
    }
}

// ------------------------------------------------------------------
// The test card - a stand-in window texture: sharp edges, smooth
// gradients, text and transparency so pixelation, masks, glow and
// particles all have something to chew on.
// ------------------------------------------------------------------

/// The card rect inside a test-card buffer - the "window" region
/// the effects animate. The margin around it stays transparent.
pub fn card_rect(size: Size<u32>) -> Rect<Pixels> {
    Rect::<Pixels>::from_xywh(f64::from(size.x) * 0.14, f64::from(size.y) * 0.15, f64::from(size.x) * 0.72, f64::from(size.y) * 0.7)
}

/// The compositor's backdrop: a calm vertical gradient - what sits
/// *behind* the window texture, never part of `ctx.window`.
pub fn desktop(size: Size<u32>) -> smearor_commons_buffer::PixelBuffer<smearor_commons_buffer::Rgba> {
    let mut desktop = smearor_commons_buffer::PixelBuffer::<smearor_commons_buffer::Rgba>::new(size);
    let mut view = desktop.view_mut();
    let bg_top = Color::<f64>::new(0.05, 0.065, 0.105);
    let bg_bottom = Color::<f64>::new(0.02, 0.026, 0.045);
    let h = f64::from(size.y);
    for y in 0..size.y {
        let t = f64::from(y) / h;
        let color = Color::<f64>::new(
            bg_top.r + (bg_bottom.r - bg_top.r) * t,
            bg_top.g + (bg_bottom.g - bg_top.g) * t,
            bg_top.b + (bg_bottom.b - bg_top.b) * t,
        );
        for x in 0..size.x {
            view.put_color(Pixels::new(x), Pixels::new(y), color);
        }
    }
    desktop
}

/// Renders the fixed test card into a `PixelBuffer<Rgba>` - a
/// floating-board window *on transparency*: soft shadow, a GNOME
/// headerbar (title, right-side window controls), an icon disc with
/// tile mark, text skeleton lines, accent chips and a footer
/// progress bar. Every pixel outside [`card_rect`] keeps `a = 0` -
/// the backdrop belongs to the compositor, not to `ctx.window`.
/// Sharp edges, smooth gradients and layered semi-transparent alpha
/// give every effect something to read.
pub fn test_card(size: Size<u32>) -> smearor_commons_buffer::PixelBuffer<smearor_commons_buffer::Rgba> {
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_buffer::Rgba;
    use smearor_commons_color::Alpha;
    use smearor_commons_drawing::DrawPrimitiveOps;
    use smearor_commons_drawing::FillCircle;
    use smearor_commons_drawing::FillRect;
    use smearor_commons_drawing::StrokePolyline;
    use smearor_commons_drawing::StrokeRect;
    use smearor_commons_geometry::Circle;
    use smearor_commons_geometry::Polyline;

    // the site palette (docs/css) - the card should read like one of
    // the floating-board cards the effects will animate
    let surface = Color::<f64>::new(0.098, 0.115, 0.165);
    let surface_alt = Color::<f64>::new(0.075, 0.09, 0.13);
    let border = Color::<f64>::new(0.17, 0.21, 0.29);
    let text_hi = Color::<f64>::new(0.88, 0.91, 0.95);
    let text_mid = Color::<f64>::new(0.38, 0.44, 0.53);
    let text_lo = Color::<f64>::new(0.24, 0.29, 0.36);
    let blue = Color::<f64>::new(0.0, 0.63, 0.9);
    let green = Color::<f64>::new(0.016, 0.906, 0.384);
    let pink = Color::<f64>::new(0.863, 0.0, 0.45);

    let px = |x: f64, y: f64| Vec2::<Pixels, f64>::new(x, y);
    let rect = |x: f64, y: f64, rw: f64, rh: f64| Rect::<Pixels>::from_xywh(x, y, rw, rh);
    let mut card = PixelBuffer::<Rgba>::new(size);
    let rect_of = card_rect(size);
    let (cx, cy, cw, ch) = (rect_of.min.x, rect_of.min.y, rect_of.size.x, rect_of.size.y);
    {
        // a fake soft shadow - three stepped offset rects, the cheap
        // blur the pixel effects read as depth
        let mut view = card.view_mut();
        let shadow = Color::<f64>::new(0.0, 0.0, 0.0);
        for (i, alpha) in [(3.0_f64, 0.10), (2.0, 0.14), (1.0, 0.20)] {
            let _ = view.fill_rect(FillRect::new(rect(cx + i * 2.0, cy + i * 2.5 + 2.0, cw, ch), shadow, Alpha::new(alpha)));
        }
        // the card - surface fill plus a one-pixel hairline border
        let _ = view.fill_rect(FillRect::new(rect(cx, cy, cw, ch), surface, Alpha::OPAQUE));
        let _ = view.stroke_rect(StrokeRect::new(rect(cx, cy, cw, ch), border, Pixels::new(1.0), Alpha::OPAQUE));
        // the headerbar - GNOME-style: centered title, the window
        // controls sit right (_ - minimize, square - maximize, x -
        // close). No traffic-light dots: this is not a Mac.
        let bar_h = ch * 0.16;
        let _ = view.fill_rect(FillRect::new(rect(cx, cy, cw, bar_h), surface_alt, Alpha::OPAQUE));
        let _ = view.fill_rect(FillRect::new(rect(cx, cy + bar_h, cw, 1.0), border, Alpha::new(0.7)));
        let title_w = cw * 0.22;
        let title = rect(cx + (cw - title_w) * 0.5, cy + bar_h * 0.5 - 4.0, title_w, 8.0);
        let _ = view.fill_rect(FillRect::new(title, text_mid, Alpha::new(0.85)));
        let mid_y = cy + bar_h * 0.5;
        let mut bx = cx + cw - 12.0;
        // close - the x glyph, subtly tinted toward pink
        let c = |dx: f64, dy: f64| px(bx + dx, mid_y + dy);
        let close = Color::<f64>::new(0.75, 0.42, 0.5);
        let _ = view.stroke_polyline(StrokePolyline::new(
            Polyline::<Pixels>::new(vec![c(-4.0, -4.0), c(4.0, 4.0)]),
            close,
            Pixels::new(1.5),
            Alpha::new(0.9),
        ));
        let _ = view.stroke_polyline(StrokePolyline::new(
            Polyline::<Pixels>::new(vec![c(-4.0, 4.0), c(4.0, -4.0)]),
            close,
            Pixels::new(1.5),
            Alpha::new(0.9),
        ));
        // maximize - an empty square
        bx -= 24.0;
        let _ = view.stroke_rect(StrokeRect::new(rect(bx - 4.0, mid_y - 4.0, 8.0, 8.0), text_mid, Pixels::new(1.5), Alpha::new(0.9)));
        // minimize - a short underscore
        bx -= 24.0;
        let _ = view.fill_rect(FillRect::new(rect(bx - 4.5, mid_y + 3.0, 9.0, 1.5), text_mid, Alpha::new(0.9)));
    }
    // body: icon disc with a tile mark, text skeleton, accent chips
    let body_y = cy + ch * 0.16;
    let content_x = cx + cw * 0.08;
    let content_w = cw * 0.84;
    {
        let mut view = card.view_mut();
        // the icon disc - the strongest hue on the card; the 2x2
        // tile mark echoes the smearor letter tiles
        let disc_c = px(content_x + 30.0, body_y + ch * 0.30);
        let _ = view.fill_circle(FillCircle::new(Circle::<Pixels>::new(disc_c, 27.0), Color::<f64>::new(0.04, 0.08, 0.13), Alpha::OPAQUE));
        let _ = view.fill_circle(FillCircle::new(Circle::<Pixels>::new(disc_c, 27.0), blue, Alpha::new(0.12)));
        let tile = 9.0;
        let gap = 4.0;
        let yellow = Color::<f64>::new(0.961, 0.718, 0.0);
        for (col, row, tile_color) in [(0.0, 0.0, green), (1.0, 0.0, yellow), (0.0, 1.0, blue), (1.0, 1.0, pink)] {
            let tx = disc_c.x - tile - gap * 0.5 + col * (tile + gap);
            let ty = disc_c.y - tile - gap * 0.5 + row * (tile + gap);
            let _ = view.fill_rect(FillRect::new(rect(tx, ty, tile, tile), tile_color, Alpha::new(0.95)));
        }
        // text skeleton - a strong title line, two muted body lines
        let line_x = content_x + 72.0;
        let line_w = content_w - 72.0;
        let mut line = |ly: f64, lw: f64, lh: f64, color: Color<f64>, alpha: f64| {
            let _ = view.fill_rect(FillRect::new(rect(line_x, body_y + ly * ch, line_w - lw, lh), color, Alpha::new(alpha)));
        };
        line(0.20, 0.0, 11.0, text_hi, 0.9);
        line(0.335, line_w * 0.16, 6.5, text_mid, 0.8);
        line(0.45, line_w * 0.30, 6.5, text_lo, 0.75);
        // accent chips - small saturated rects the glow effects key on
        for (i, chip) in [blue, green, pink].iter().enumerate() {
            let chip_rect = rect(line_x + i as f64 * (line_w * 0.24), body_y + ch * 0.55, line_w * 0.19, 11.0);
            let _ = view.fill_rect(FillRect::new(chip_rect, *chip, Alpha::new(0.9)));
        }
        // a muted secondary button under the chips
        let ghost = rect(line_x, body_y + ch * 0.66, line_w * 0.42, 16.0);
        let _ = view.stroke_rect(StrokeRect::new(ghost, text_lo, Pixels::new(1.0), Alpha::new(0.9)));
        let _ = view.fill_rect(FillRect::new(rect(line_x + 10.0, body_y + ch * 0.66 + 5.0, line_w * 0.2, 6.0), text_mid, Alpha::new(0.8)));
        // the footer - hairline plus the progress bar on a track
        let footer_y = cy + ch * 0.86;
        let _ = view.fill_rect(FillRect::new(rect(cx, footer_y, cw, 1.0), border, Alpha::new(0.7)));
        let track = rect(content_x, footer_y + 12.0, content_w, 7.0);
        let _ = view.stroke_rect(StrokeRect::new(track, text_lo, Pixels::new(1.0), Alpha::new(0.7)));
        let bar = rect(content_x + 1.0, footer_y + 13.0, content_w * 0.62 - 2.0, 5.0);
        let _ = view.fill_rect(FillRect::new(bar, blue, Alpha::new(0.95)));
    }
    card
}

// ------------------------------------------------------------------
// Emit - Rust source generation.
// ------------------------------------------------------------------

/// Emits the effect as Rust source: a `serde_json` constructor for
/// the spec payload plus a ready `FxContext`/`apply` skeleton. The
/// generated file needs `smearor-commons-fx`, `-buffer`, `-geometry`,
/// `-noise` and `serde_json`.
pub fn emit_fx_rust(effect: &Effect) -> String {
    let Some((spec, direction, seed)) = spec_from_envelope(effect) else {
        return "// emit failed: envelope does not resolve to a known effect spec\n".to_string();
    };
    let payload = serde_json::to_string(&spec).unwrap_or_else(|_| "{}".to_string());
    let direction_variant = match direction {
        Direction::Open => "Direction::Open",
        Direction::Close => "Direction::Close",
    };
    format!(
        "//! Generated by the fx experimenter's Emit button.\n\
         //! {} - serialized as `{payload}`.\n\
         //! Needs `smearor-commons-fx`, `-buffer`, `-geometry`, `-noise` and `serde_json`.\n\n\
         use smearor_commons_buffer::PixelBuffer;\n\
         use smearor_commons_buffer::Rgba;\n\
         use smearor_commons_fx::Direction;\n\
         use smearor_commons_fx::EffectSpec;\n\
         use smearor_commons_fx::FxContext;\n\
         use smearor_commons_fx::Layer;\n\
         use smearor_commons_geometry::Fraction;\n\
         use smearor_commons_geometry::Pixels;\n\
         use smearor_commons_geometry::Rect;\n\
         use smearor_commons_noise::Seed;\n\
         use std::time::Duration;\n\n\
         /// The configured effect spec - identical JSON to the saved effect file.\n\
         pub fn spec() -> EffectSpec {{\n    \
             serde_json::from_str::<EffectSpec>(r#\"{payload}\"#).expect(\"emitted spec is valid\")\n\
         }}\n\n\
         /// Applies `spec()` to `window` at `progress` (0.0..=1.0), writing `out`.\n\
         pub fn apply(window: &PixelBuffer<Rgba>, out: &mut PixelBuffer<Rgba>, progress: f64) {{\n    \
             let size = window.size();\n    \
             let ctx = FxContext {{\n        \
                 progress: Fraction::new(progress.clamp(0.0, 1.0)),\n        \
                 duration: Duration::from_secs_f64({:?}),\n        \
                 seed: Seed({seed}),\n        \
                 direction: {direction_variant},\n        \
                 layer: Layer::Window,\n        \
                 window: window.view(),\n        \
                 surface: None,\n        \
                 pointer: None,\n        \
                 fullscreen: false,\n        \
                 padding: Pixels::new(0.0),\n        \
                 window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),\n    \
             }};\n    \
             spec().build().apply(&ctx, &mut out.view_mut());\n\
         }}\n",
        effect.object, effect.duration_secs,
    )
}
