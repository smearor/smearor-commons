//! GTK experimenter for the fx crate: pick an effect, tune its
//! metadata-generated parameters, scrub the frame clock and save,
//! load or emit the configuration as an `Effect` envelope.
//!
//! The right panel is generated from `effect.params()` - there is no
//! hand-written per-effect UI.
//!
//! Run with `cargo run -p smearor-commons-fx --features gtk4 --example fx`.

mod common;

use common::Animatable;
use common::Effect;
use common::ParamWidgets;
use common::card_rect;
use common::desktop;
use common::dropdown_row;
use common::effect_group;
use common::effect_options;
use common::emit_fx_rust;
use common::group_frame;
use common::install_tick;
use common::load_effect;
use common::paned_view;
use common::param_panel;
use common::playback_group;
use common::save_effect;
use common::seed_row;
use common::select_by_name;
use common::spec_from_envelope;
use common::test_card;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::Orientation;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_animation::EasingFunction;
use smearor_commons_animation::Span;
use smearor_commons_animation::TimedAnimation;
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
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_fx.fx";

/// The test card's pixel size - the stand-in window texture.
const CARD: Size<u32> = Size::new(400, 300);

/// The demo state: which effect is configured, the card it applies
/// to, the frame output and the linear clock plus context inputs.
struct FxDemo {
    /// The currently configured spec - rebuilt on every widget read.
    spec: EffectSpec,
    /// The rendered stand-in window texture (`FxContext::window`) -
    /// card pixels on transparency; the desktop is *not* part of it.
    card: PixelBuffer<Rgba>,
    /// The compositor's backdrop - display only, the effect never
    /// sees it.
    desktop: PixelBuffer<Rgba>,
    /// The frame output the effect writes into - card-shaped alpha
    /// on transparency.
    out: PixelBuffer<Rgba>,
    /// The window pass of a `SurfaceAndWindow` effect - composited
    /// over `out` (the surface pass) before `out` lands on `desktop`.
    out_window: PixelBuffer<Rgba>,
    /// `out` composited over `desktop` - what the canvas shows (the
    /// compositor's job).
    display: PixelBuffer<Rgba>,
    /// The linear 0..=1 run clock - `Span` is the identity
    /// trajectory, the `FxContext` wants the bare `Fraction`.
    animation: TimedAnimation<Span<f64>>,
    /// `FxContext::direction` - open or close.
    direction: Direction,
    /// `FxContext::seed` - deterministic per-run seed.
    seed: Seed,
}

impl FxDemo {
    fn new(spec: EffectSpec, duration: Duration) -> Self {
        Self {
            spec,
            card: test_card(CARD),
            desktop: desktop(CARD),
            out: PixelBuffer::<Rgba>::new(CARD),
            out_window: PixelBuffer::<Rgba>::new(CARD),
            display: PixelBuffer::<Rgba>::new(CARD),
            animation: TimedAnimation::new(Span::new(0.0, 1.0), duration, EasingFunction::Linear),
            direction: Direction::Open,
            seed: Seed(42),
        }
    }

    /// Renders one frame of the configured spec into `out`, then
    /// composites it over the desktop backdrop into `display`.
    fn render(&mut self) {
        let mut ctx = FxContext {
            progress: self.animation.progress().unwrap_or_else(|| Fraction::new(0.0)),
            duration: self.animation.duration,
            seed: self.seed,
            direction: self.direction,
            layer: Layer::Window,
            window: self.card.view(),
            surface: Some(self.desktop.view()),
            assets: smearor_commons_fx::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            // the fake soft shadow is window content - the fade zone
            // around the card rect
            padding: Pixels::new(12.0),
            window_rect: card_rect(CARD),
        };
        let effect = self.spec.build();
        ctx.layer = if effect.layers() == smearor_commons_fx::Layers::SurfaceAndWindow {
            Layer::Surface
        } else {
            Layer::Window
        };
        effect.apply(&ctx, &mut self.out.view_mut());
        // The compositor pass: `out` source-over `desktop`. Layered
        // effects stack a second (window) pass on top of the first
        // (surface) pass.
        let layered = effect.layers() == smearor_commons_fx::Layers::SurfaceAndWindow;
        if layered {
            ctx.layer = Layer::Window;
            effect.apply(&ctx, &mut self.out_window.view_mut());
        }
        let size = self.display.size();
        let inv = Vec2::<()>::new(1.0 / f64::from(size.x), 1.0 / f64::from(size.y));
        for y in 0..size.y {
            for x in 0..size.x {
                let uv = Vec2::<()>::new((x as f64 + 0.5) * inv.x, (y as f64 + 0.5) * inv.y);
                let over = if layered { &self.out_window } else { &self.out }.view().sample_nearest(uv);
                let base = self.desktop.view().sample_nearest(uv);
                let under = if layered {
                    self.out.view().sample_nearest(uv).alpha_over(&base)
                } else {
                    base
                };
                self.display.view_mut().put_color(Pixels::new(x), Pixels::new(y), over.alpha_over(&under));
            }
        }
    }
}

impl Animatable for FxDemo {
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
        Some(EasingFunction::Linear)
    }

    /// One run is one phase - the phase bar mirrors the total bar.
    fn phase_at(&self, _progress: f64) -> (u32, u32) {
        (1, 1)
    }

    fn phase_fraction(&self, progress: f64) -> f64 {
        progress
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        // Picture scales the texture to the pane - Image would show
        // the 400x300 card at native size, centered and tiny.
        let canvas = gtk4::Picture::new();
        canvas.set_content_fit(gtk4::ContentFit::Contain);
        canvas.set_can_shrink(false);
        canvas.set_hexpand(true);
        canvas.set_vexpand(true);

        // --- controls -------------------------------------------------

        let controls = GtkBox::new(Orientation::Vertical, 6);
        controls.set_margin_top(12);
        controls.set_margin_bottom(12);
        controls.set_margin_start(12);
        controls.set_margin_end(12);
        controls.set_width_request(320);

        let specs = EffectSpec::all();
        let nicks: Vec<&'static str> = specs.iter().map(EffectSpec::nick).collect();
        let labels: Vec<&'static str> = specs.iter().map(|spec| spec.build().label()).collect();

        let fx_box = GtkBox::new(Orientation::Vertical, 6);
        let (_effect_row, effect_dd) = dropdown_row(&fx_box, "Effect", &labels);
        let (_direction_row, direction_dd) = dropdown_row(&fx_box, "Direction", &["Open", "Close"]);
        let (_seed_row, seed_spin, dice) = seed_row(&fx_box, "Seed", 42);
        controls.append(&group_frame("Effect", &fx_box));

        // the metadata-generated parameter panel - rebuilt when the
        // selected effect changes
        let params_box = GtkBox::new(Orientation::Vertical, 6);
        controls.append(&group_frame("Parameters", &params_box));

        let playback = playback_group(&controls);
        let effects = effect_group(&controls);

        // --- state ------------------------------------------------------

        let demo = Rc::new(RefCell::new(FxDemo::new(specs[0].clone(), Duration::from_secs_f64(playback.duration.value()))));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));
        let param_widgets: Rc<RefCell<Option<ParamWidgets>>> = Rc::new(RefCell::new(None));

        // Re-reads the whole panel into the demo: spec from widgets,
        // direction and seed from their rows.
        let sync_demo = {
            let (demo, param_widgets) = (Rc::clone(&demo), Rc::clone(&param_widgets));
            let (effect_dd, direction_dd, seed_spin) = (effect_dd.clone(), direction_dd.clone(), seed_spin.clone());
            let nicks = nicks.clone();
            move || {
                let mut demo = demo.borrow_mut();
                if let Some(widgets) = param_widgets.borrow().as_ref()
                    && let Ok(spec) = EffectSpec::from_params(nicks[effect_dd.selected() as usize], &widgets.values())
                {
                    demo.spec = spec;
                }
                demo.direction = if direction_dd.selected() == 0 { Direction::Open } else { Direction::Close };
                demo.seed = Seed(seed_spin.value() as u64);
            }
        };
        let sync_demo: Rc<dyn Fn()> = Rc::new(sync_demo);

        // Rebuilds the parameter rows for the selected effect. The
        // `updating` guard stops widget-write-backs from re-entering.
        let rebuild_params = {
            let (params_box, param_widgets) = (params_box.clone(), Rc::clone(&param_widgets));
            let (effect_dd, specs) = (effect_dd.clone(), specs.clone());
            let sync_demo = Rc::clone(&sync_demo);
            let updating = Cell::new(false);
            move || {
                if updating.replace(true) {
                    return;
                }
                while let Some(child) = params_box.first_child() {
                    params_box.remove(&child);
                }
                let widgets = param_panel(&params_box, specs[effect_dd.selected() as usize].build().params());
                for widget in &widgets.widgets {
                    let sync_demo = Rc::clone(&sync_demo);
                    widget.connect_changed(move || sync_demo());
                }
                *param_widgets.borrow_mut() = Some(widgets);
                sync_demo();
                updating.set(false);
            }
        };
        let rebuild_params: Rc<dyn Fn()> = Rc::new(rebuild_params);

        effect_dd.connect_selected_notify({
            let rebuild_params = Rc::clone(&rebuild_params);
            move |_| rebuild_params()
        });
        direction_dd.connect_selected_notify({
            let sync_demo = Rc::clone(&sync_demo);
            move |_| sync_demo()
        });
        seed_spin.connect_value_changed({
            let sync_demo = Rc::clone(&sync_demo);
            move |_| sync_demo()
        });
        dice.connect_clicked({
            let seed_spin = seed_spin.clone();
            move |_| seed_spin.set_value(Seed::random().0 as f64)
        });
        playback.duration.connect_change_value({
            let demo = Rc::clone(&demo);
            move |_, _, value| {
                demo.borrow_mut().animation.duration = Duration::from_secs_f64(value);
                glib::Propagation::Proceed
            }
        });

        // --- effect save / load / emit ----------------------------------

        let current_effect: Rc<dyn Fn() -> Effect> = Rc::new({
            let demo = Rc::clone(&demo);
            let duration = playback.duration.clone();
            move || {
                let demo = demo.borrow();
                Effect {
                    demo: "fx".to_string(),
                    object: demo.spec.nick().to_string(),
                    options: effect_options(&demo.spec, demo.direction, demo.seed.0),
                    duration_secs: duration.value(),
                    components: Vec::new(),
                }
            }
        });

        // Writes a loaded envelope back onto the widgets: effect
        // selection rebuilds the panel, then spec values, direction,
        // seed and duration follow.
        let apply_effect: Rc<dyn Fn(&Effect)> = Rc::new({
            let (demo, param_widgets) = (Rc::clone(&demo), Rc::clone(&param_widgets));
            let (effect_dd, direction_dd, seed_spin) = (effect_dd.clone(), direction_dd.clone(), seed_spin.clone());
            let duration = playback.duration.clone();
            let (labels, nicks) = (labels.clone(), nicks.clone());
            move |effect| {
                let Some((spec, direction, seed)) = spec_from_envelope(effect) else {
                    eprintln!("effect {:?} is not a known fx spec", effect.object);
                    return;
                };
                select_by_name(&effect_dd, &labels, spec.build().label());
                // the dropdown's notify rebuilt the panel - write the
                // loaded values back in params() order
                let values = spec.build().values();
                if let Some(widgets) = param_widgets.borrow().as_ref() {
                    widgets.set_values(&values);
                }
                select_by_name(&direction_dd, &["Open", "Close"], if direction == Direction::Open { "Open" } else { "Close" });
                seed_spin.set_value(seed as f64);
                duration.set_value(effect.duration_secs);
                let _ = nicks;
                {
                    let mut demo = demo.borrow_mut();
                    demo.spec = spec;
                    demo.direction = direction;
                    demo.seed = Seed(seed);
                    demo.animation.duration = Duration::from_secs_f64(effect.duration_secs);
                }
            }
        });

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
                let rust = emit_fx_rust(&current());
                let target = format!("{}.rs", path.text().trim_end_matches(".json"));
                if let Err(error) = std::fs::write(&target, rust) {
                    eprintln!("effect emit failed: {error}");
                }
            });
        }

        // --- frame clock -------------------------------------------------

        install_tick(canvas.upcast_ref(), Rc::clone(&demo), &playback, Rc::clone(&hold), |demo| {
            demo.render();
        });
        {
            let canvas = canvas.clone();
            let demo = Rc::clone(&demo);
            canvas.add_tick_callback(move |canvas, _| {
                canvas.set_paintable(Some(&demo.borrow().display.to_memory_texture()));
                glib::ControlFlow::Continue
            });
        }

        let view = paned_view(&canvas, &controls);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("smearor-commons fx")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        rebuild_params();
        demo.borrow_mut().start();
        window.present();
    });
    app.run()
}
