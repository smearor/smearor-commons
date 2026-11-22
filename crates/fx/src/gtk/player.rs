//! [`FxPlayer`] - a `DrawingArea` subclass that owns the animation
//! clock and frame buffers and plays any [`EffectSpec`].
//!
//! The player is the *mechanism*; choreography stays with the
//! consumer. The floating-board recipe:
//!
//! 1. `card-added`: [`crate::gtk::snapshot`] the card, place an
//!    `FxPlayer` at the same coordinates, hide (not destroy) the card,
//!    `play` an `Open`-direction spec, on `animation-ended` remove
//!    the player and show the card.
//! 2. `card-removed`: same in `Close` direction, destroy the card on
//!    end.
//! 3. Board-level transitions: snapshot card + board, run a layered
//!    spec with a [`crate::gtk::SurfaceProvider`], hand back to live
//!    widgets.

use crate::Direction;
use crate::EffectSpec;
use crate::FxContext;
use crate::Layer;
use crate::Layers;
use crate::gtk::SurfaceProvider;
use crate::gtk::pointer::wire_pointer;
use gtk4::glib;
use gtk4::glib::object::Cast;
use gtk4::glib::object::ObjectExt;
use gtk4::glib::subclass::types::ObjectSubclassIsExt;
use gtk4::gsk::prelude::IsRenderNode;
use gtk4::prelude::DrawingAreaExtManual;
use gtk4::prelude::WidgetExt;
use gtk4::prelude::WidgetExtManual;
use smearor_commons_animation::EasingFunction;
use smearor_commons_animation::Span;
use smearor_commons_animation::TimedAnimation;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Seed;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// Play-time options for [`FxPlayer::play`] - everything that is *not*
/// part of the serializable [`EffectSpec`] payload.
#[derive(Debug, Clone, Copy)]
pub struct FxPlayOptions {
    /// Animation duration (the `animation-time` setting in BMW).
    pub duration: Duration,
    /// `Open` (window materializes) or `Close` (window dissolves).
    pub direction: Direction,
    /// Deterministic frame seed - same seed + progress replays
    /// bitwise.
    pub seed: Seed,
    /// `ctx.fullscreen` - forwarded to effects that branch on it.
    pub fullscreen: bool,
    /// Transparent margin around the window rect inside the input
    /// texture (e.g. where a shadow is drawn).
    pub padding: Pixels<f64>,
    /// The time easing applied by [`TimedAnimation`] - `Linear` is
    /// honest: BMW eases inside the shader, our effects do the same.
    pub easing: EasingFunction,
    /// The window rect inside the input texture - `None` means the
    /// full buffer (the common player == window case).
    pub window_rect: Option<Rect<Pixels, f64>>,
}

impl Default for FxPlayOptions {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(600),
            direction: Direction::Open,
            seed: Seed(0),
            fullscreen: false,
            padding: Pixels::new(0.0),
            easing: EasingFunction::Linear,
            window_rect: None,
        }
    }
}

mod imp {
    use super::*;

    /// Mutable render state of [`FxPlayer`] - all RefCell/Cell, GTK is
    /// single-threaded.
    pub struct FxPlayerImp {
        pub spec: RefCell<Option<EffectSpec>>,
        pub input: RefCell<Option<PixelBuffer<Rgba>>>,
        pub out_surface: RefCell<PixelBuffer<Rgba>>,
        pub out_window: RefCell<PixelBuffer<Rgba>>,
        pub display: RefCell<PixelBuffer<Rgba>>,
        pub animation: RefCell<TimedAnimation<Span<f64>>>,
        pub direction: Cell<Direction>,
        pub seed: Cell<Seed>,
        pub fullscreen: Cell<bool>,
        pub padding: Cell<f64>,
        pub window_rect: Cell<Option<(f64, f64, f64, f64)>>,
        pub provider: RefCell<Option<SurfaceProvider>>,
        pub pointer: Rc<Cell<Option<Vec2<()>>>>,
        pub playing: Cell<bool>,
        pub paused: Cell<Option<f64>>,
        pub ended_emitted: Cell<bool>,
    }

    #[glib::object_subclass]
    impl glib::subclass::types::ObjectSubclass for FxPlayerImp {
        const NAME: &'static str = "SmearorFxPlayer";
        type Type = FxPlayer;
        type ParentType = gtk4::DrawingArea;

        fn new() -> Self {
            Self {
                spec: RefCell::new(None),
                input: RefCell::new(None),
                out_surface: RefCell::new(PixelBuffer::<Rgba>::new(Size::new(1, 1))),
                out_window: RefCell::new(PixelBuffer::<Rgba>::new(Size::new(1, 1))),
                display: RefCell::new(PixelBuffer::<Rgba>::new(Size::new(1, 1))),
                animation: RefCell::new(TimedAnimation::new(Span::new(0.0, 1.0), Duration::from_millis(600), EasingFunction::Linear)),
                direction: Cell::new(Direction::Open),
                seed: Cell::new(Seed(0)),
                fullscreen: Cell::new(false),
                padding: Cell::new(0.0),
                window_rect: Cell::new(None),
                provider: RefCell::new(None),
                pointer: Rc::new(Cell::new(None)),
                playing: Cell::new(false),
                paused: Cell::new(None),
                ended_emitted: Cell::new(false),
            }
        }
    }

    impl glib::subclass::prelude::ObjectImpl for FxPlayerImp {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: std::sync::LazyLock<Vec<glib::subclass::Signal>> =
                std::sync::LazyLock::new(|| vec![glib::subclass::Signal::builder("animation-ended").build()]);
            SIGNALS.as_slice()
        }
    }

    impl gtk4::subclass::prelude::WidgetImpl for FxPlayerImp {
        /// The frozen input texture is the player's natural size -
        /// device pixels divided back by the surface scale.
        fn measure(&self, orientation: gtk4::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            use gtk4::glib::subclass::prelude::ObjectSubclassExt;
            use gtk4::subclass::prelude::WidgetImplExt;
            if let Some(input) = self.input.borrow().as_ref() {
                let scale = self.obj().scale_factor().max(1);
                let extent = match orientation {
                    gtk4::Orientation::Horizontal => input.size().x as i32 / scale,
                    _ => input.size().y as i32 / scale,
                };
                return (extent, extent, -1, -1);
            }
            self.parent_measure(orientation, for_size)
        }

        /// `size-allocate` rebuilds the frame buffers at
        /// `allocation * scale_factor` - only while no input is
        /// installed; with an input the buffers stay bound to the
        /// frozen texture size (see `set_effect`).
        fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
            use gtk4::glib::subclass::prelude::ObjectSubclassExt;
            use gtk4::subclass::prelude::WidgetImplExt;
            self.parent_size_allocate(width, height, _baseline);
            if self.input.borrow().is_some() {
                return;
            }
            let scale = self.obj().scale_factor().max(1);
            let size = Size::new((width * scale).max(1) as u32, (height * scale).max(1) as u32);

            *self.out_surface.borrow_mut() = PixelBuffer::<Rgba>::new(size);
            *self.out_window.borrow_mut() = PixelBuffer::<Rgba>::new(size);
            *self.display.borrow_mut() = PixelBuffer::<Rgba>::new(size);
        }
    }

    impl gtk4::subclass::prelude::DrawingAreaImpl for FxPlayerImp {}
}

glib::wrapper! {
    /// A `DrawingArea` that plays an [`EffectSpec`] on a frozen input
    /// texture.
    ///
    /// `set_effect` installs spec + input, `play` runs the clock. The
    /// `animation-ended` signal fires once when the duration elapses.
    /// Layered specs pull their backdrop through
    /// [`FxPlayer::set_surface_provider`].
    pub struct FxPlayer(ObjectSubclass<imp::FxPlayerImp>)
        @extends gtk4::DrawingArea, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl Default for FxPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl FxPlayer {
    /// An empty player - install an effect via [`Self::set_effect`],
    /// then [`Self::play`].
    pub fn new() -> Self {
        let player: Self = glib::Object::new();
        wire_pointer(&player, Rc::clone(&player.imp().pointer));
        player
    }

    /// Installs the effect and its frozen window texture. Does not
    /// start the clock - call [`Self::play`].
    ///
    /// The input texture is the player's natural render size: the
    /// frame buffers are resized to it and `measure` reports it as the
    /// natural widget size, so a parent (`Fixed`, overlay, ...) gives
    /// the player exactly the snapshotted widget's extent.
    pub fn set_effect(&self, spec: EffectSpec, input: PixelBuffer<Rgba>) {
        let size = input.size();
        let imp = self.imp();
        *imp.out_surface.borrow_mut() = PixelBuffer::<Rgba>::new(size);
        *imp.out_window.borrow_mut() = PixelBuffer::<Rgba>::new(size);
        *imp.display.borrow_mut() = PixelBuffer::<Rgba>::new(size);
        *imp.spec.borrow_mut() = Some(spec);
        *imp.input.borrow_mut() = Some(input);
        self.queue_resize();
    }

    /// Installs the surface provider for
    /// [`Layers::SurfaceAndWindow`] specs - called once per frame.
    /// `None` degrades layered passes to transparent surface reads
    /// (the [`FxError::MissingSurface`] contract, graceful).
    pub fn set_surface_provider(&self, provider: SurfaceProvider) {
        *self.imp().provider.borrow_mut() = Some(provider);
    }

    /// Starts (or restarts) the animation with `options`.
    pub fn play(&self, options: FxPlayOptions) {
        let imp = self.imp();
        imp.direction.set(options.direction);
        imp.seed.set(options.seed);
        imp.fullscreen.set(options.fullscreen);
        imp.padding.set(options.padding.0);
        imp.window_rect.set(options.window_rect.map(|r| (r.min.x, r.min.y, r.size.x, r.size.y)));
        imp.ended_emitted.set(false);
        imp.paused.set(None);
        {
            let mut animation = imp.animation.borrow_mut();
            animation.duration = options.duration;
            animation.easing_function = options.easing;
            animation.start();
        }
        imp.playing.set(true);

        self.set_draw_func(|area, cr, width, height| {
            let area = area.downcast_ref::<FxPlayer>().cloned();
            if let Some(player) = area {
                player.paint(cr, width, height);
            }
        });

        let player = self.clone();
        self.add_tick_callback(move |_, _clock| {
            player.render_frame();
            player.queue_draw();
            let imp = player.imp();
            if imp.playing.get() && player.is_complete() {
                imp.playing.set(false);
                if !imp.ended_emitted.replace(true) {
                    player.emit_by_name::<()>("animation-ended", &[]);
                    return glib::ControlFlow::Break;
                }
            }
            glib::ControlFlow::Continue
        });
    }

    /// Pins the clock at the current progress - the animation stays
    /// visible but stops advancing until [`Self::resume`].
    pub fn pause(&self) {
        let imp = self.imp();
        let progress = imp.animation.borrow().progress().map(|p| p.as_f64().min(1.0)).unwrap_or(0.0);
        imp.paused.set(Some(progress));
    }

    /// Unpins the clock after [`Self::pause`].
    pub fn resume(&self) {
        self.imp().paused.set(None);
    }

    /// Whether the animation ran through.
    pub fn is_complete(&self) -> bool {
        self.imp().animation.borrow().is_complete()
    }

    /// The frame buffer size - the input texture size after
    /// [`Self::set_effect`], `allocation * scale_factor` before that.
    pub fn frame_size(&self) -> Size<u32> {
        self.imp().display.borrow().size()
    }

    /// Stops the clock without emitting `animation-ended` - the
    /// consumer's way out when it takes the widget back mid-animation.
    pub fn stop(&self) {
        self.imp().playing.set(false);
        self.imp().paused.set(None);
    }

    /// The `animation-ended` signal - fires once at the end of a
    /// [`Self::play`] run.
    pub fn connect_animation_ended<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("animation-ended", true, move |values| {
            if let Some(player) = values.first().and_then(|v| v.get::<FxPlayer>().ok()) {
                f(&player);
            }
            None
        })
    }

    /// Renders one frame into the `display` buffer - the two-pass
    /// pipeline shared with the `fx.rs` experimenter: surface pass
    /// (layered only), window pass, composite window over surface.
    fn render_frame(&self) {
        let imp = self.imp();
        let Some(spec) = imp.spec.borrow().clone() else {
            return;
        };
        let Some(input) = imp.input.borrow().clone() else {
            return;
        };
        let effect = spec.build();
        let layered = effect.layers() == Layers::SurfaceAndWindow;

        let progress = if let Some(pinned) = imp.paused.get() {
            Fraction::new(pinned)
        } else {
            imp.animation.borrow().progress().unwrap_or(Fraction::ONE)
        };

        let surface = imp.provider.borrow().as_ref().and_then(|provider| provider());

        let size = imp.display.borrow().size();
        let window_rect = imp
            .window_rect
            .get()
            .map(|(x, y, w, h)| Rect::from_xywh(x, y, w, h))
            .unwrap_or_else(|| Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)));

        let ctx = FxContext {
            progress,
            duration: imp.animation.borrow().duration,
            seed: imp.seed.get(),
            direction: imp.direction.get(),
            layer: if layered { Layer::Surface } else { Layer::Window },
            window: input.view(),
            surface: surface.as_ref().map(|s| s.view()),
            assets: crate::FxAssets::default(),
            pointer: imp.pointer.get(),
            fullscreen: imp.fullscreen.get(),
            padding: Pixels::new(imp.padding.get()),
            window_rect,
        };

        // Surface pass only when the layered contract holds.
        if layered && effect.check(&ctx).is_ok() {
            effect.apply(&ctx, &mut imp.out_surface.borrow_mut().view_mut());
        }

        let ctx = FxContext { layer: Layer::Window, ..ctx };
        effect.apply(&ctx, &mut imp.out_window.borrow_mut().view_mut());

        // Composite: window over surface - `display` keeps the
        // transparent regions for whatever sits behind the player.
        let w = f64::from(size.x).max(1.0);
        let h = f64::from(size.y).max(1.0);
        {
            let window_out = imp.out_window.borrow();
            let surface_out = imp.out_surface.borrow();
            let mut display = imp.display.borrow_mut();
            let mut view = display.view_mut();
            for y in 0..size.y {
                for x in 0..size.x {
                    let uv = Vec2::<()>::new((f64::from(x) + 0.5) / w, (f64::from(y) + 0.5) / h);
                    let over = window_out.view().sample_nearest(uv);
                    let under = surface_out.view().sample_nearest(uv);
                    view.put_color(Pixels::new(x), Pixels::new(y), over.alpha_over(&under));
                }
            }
        }
    }

    /// Paints `display` as a texture node onto the draw context.
    fn paint(&self, cr: &cairo::Context, width: i32, height: i32) {
        let texture = self.imp().display.borrow().to_memory_texture();
        let bounds = gtk4::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
        let node = gtk4::gsk::TextureNode::new(&texture, &bounds).upcast();
        node.draw(cr);
    }
}
