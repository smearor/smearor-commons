//! GTK playground for the animation crate: pick a trajectory and an
//! easing function, tune their parameters with sliders and watch the
//! result on the frame clock.
//!
//! Run with `cargo run -p smearor-commons-animation --features gtk4 --example playground`.

mod common;

use common::Animatable;
use common::ComponentKnobs;
use common::ComponentPanel;
use common::Compose2;
use common::ComposeXY;
use common::ComposeYZ;
use common::EASINGS;
use common::ORBITAL;
use common::POINT_PATHS;
use common::PhaseShape;
use common::build_scalar_trajectory;
use common::component_knobs;
use common::component_panel;
use common::component_roles;
use common::compositions;
use common::connect_panel;
use common::dropdown_row;
use common::eased;
use common::easing_by_name;
use common::group_frame;
use common::install_tick;
use common::paned_view;
use common::planar_trajectory;
use common::playback_group;
use common::role_choices;
use common::spatial_trajectory;
use common::update_panel;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::DrawingArea;
use gtk4::Orientation;
use gtk4::StringList;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_animation::EasingFunction;
use smearor_commons_animation::TimedAnimation;
use smearor_commons_animation::Trajectory;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_animation.playground";

/// A draggable center marker of an orbital trajectory component.
struct CenterMark {
    /// The center in model coordinates.
    point: Vec2<()>,
    /// The component slot the marker controls.
    slot: usize,
}

/// The animation under test - one boxed slot per value dimension.
/// `control_points` carries the bezier handles for drawing.
enum Demo {
    /// 1D - scalar trajectories on the x axis.
    Scalar {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = f64>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
    /// 2D - planar trajectories.
    Planar {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = Vec2<()>>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// Bezier handles drawn as reference points.
        control_points: Vec<Vec2<()>>,
        /// Draggable centers of orbital components.
        centers: Vec<CenterMark>,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
    /// 3D - spatial trajectories, `z` scales the logo.
    Spatial {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = Vec3<()>>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// Bezier handles drawn as reference points.
        control_points: Vec<Vec2<()>>,
        /// Draggable centers of orbital components.
        centers: Vec<CenterMark>,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
}

impl Demo {
    /// The current value as `(point, scale)` - scalar demos sit on
    /// the mid line, spatial demos scale the logo with `z`.
    fn value(&self) -> Option<(Vec2<()>, f64)> {
        match self {
            Self::Scalar { animation, .. } => animation.value().map(|v| (Vec2::new(v, 0.5), 1.0)),
            Self::Planar { animation, .. } => animation.value().map(|v| (v, 1.0)),
            Self::Spatial { animation, .. } => animation.value().map(|v| (Vec2::new(v.x, v.y), 0.4 + 0.8 * v.z.clamp(0.0, 1.0))),
        }
    }

    fn progress(&self) -> Option<Fraction> {
        match self {
            Self::Scalar { animation, .. } => animation.progress(),
            Self::Planar { animation, .. } => animation.progress(),
            Self::Spatial { animation, .. } => animation.progress(),
        }
    }

    /// The trajectory value at eased progress `t` - scalar demos
    /// report their value as x.
    fn sample(&self, t: Fraction) -> Vec2<()> {
        match self {
            Self::Scalar { animation, .. } => Vec2::new(animation.trajectory.value_at(t), 0.5),
            Self::Planar { animation, .. } => animation.trajectory.value_at(t),
            Self::Spatial { animation, .. } => {
                let v = animation.trajectory.value_at(t);
                Vec2::new(v.x, v.y)
            }
        }
    }

    /// The draggable center markers of orbital components.
    fn centers(&self) -> &[CenterMark] {
        match self {
            Self::Scalar { .. } => &[],
            Self::Planar { centers, .. } | Self::Spatial { centers, .. } => centers,
        }
    }

    /// The per-component easings - one curve per split component.
    fn easings(&self) -> &[EasingFunction] {
        match self {
            Self::Scalar { easings, .. } | Self::Planar { easings, .. } | Self::Spatial { easings, .. } => easings,
        }
    }

    /// Bezier control points for drawing - empty for other shapes.
    fn control_points(&self) -> &[Vec2<()>] {
        match self {
            Self::Scalar { .. } => &[],
            Self::Planar { control_points, .. } | Self::Spatial { control_points, .. } => control_points,
        }
    }
}

impl Animatable for Demo {
    fn is_complete(&self) -> bool {
        match self {
            Self::Scalar { animation, .. } => animation.is_complete(),
            Self::Planar { animation, .. } => animation.is_complete(),
            Self::Spatial { animation, .. } => animation.is_complete(),
        }
    }

    fn start(&mut self) {
        match self {
            Self::Scalar { animation, .. } => animation.start(),
            Self::Planar { animation, .. } => animation.start(),
            Self::Spatial { animation, .. } => animation.start(),
        }
    }

    fn seek(&mut self, progress: Fraction) {
        match self {
            Self::Scalar { animation, .. } => animation.seek(progress),
            Self::Planar { animation, .. } => animation.seek(progress),
            Self::Spatial { animation, .. } => animation.seek(progress),
        }
    }

    fn progress(&self) -> Option<Fraction> {
        self.progress()
    }

    fn first_easing(&self) -> Option<EasingFunction> {
        self.easings().first().copied()
    }

    fn phase_at(&self, progress: f64) -> (u32, u32) {
        match self {
            Self::Scalar { phase, .. } | Self::Planar { phase, .. } | Self::Spatial { phase, .. } => phase.phase_at(progress),
        }
    }

    fn phase_fraction(&self, progress: f64) -> f64 {
        match self {
            Self::Scalar { phase, .. } | Self::Planar { phase, .. } | Self::Spatial { phase, .. } => phase.fraction(progress),
        }
    }
}

/// The global parameters - layout and timing, not per-component.
struct Knobs {
    duration: Duration,
    dimension: u32,
    composition: u32,
    components: [ComponentKnobs; 3],
}

/// Builds the animation from the current control state - the
/// composition splits the value axes into independently driven
/// components, each with its own trajectory.
fn build_demo(knobs: &Knobs) -> Demo {
    let roles = component_roles(knobs.dimension, knobs.composition);
    let name = |slot: usize| role_choices(roles[slot])[knobs.components[slot].trajectory as usize];
    let planar = |slot: usize| -> (Box<dyn Trajectory<Value = Vec2<()>>>, PhaseShape) {
        let (trajectory, phase) = planar_trajectory(name(slot), &knobs.components[slot]);
        (eased(trajectory, &knobs.components[slot]), phase)
    };
    let spatial = |slot: usize| -> (Box<dyn Trajectory<Value = Vec3<()>>>, PhaseShape) {
        let (trajectory, phase) = spatial_trajectory(name(slot), &knobs.components[slot]);
        (eased(trajectory, &knobs.components[slot]), phase)
    };
    let scalar = |slot: usize| -> (Box<dyn Trajectory<Value = f64>>, PhaseShape) {
        let (trajectory, phase) = build_scalar_trajectory(name(slot), &knobs.components[slot]);
        (eased(trajectory, &knobs.components[slot]), phase)
    };
    let easings = |count: usize| {
        knobs.components[..count]
            .iter()
            .map(|c| easing_by_name(EASINGS[c.easing as usize], c).0)
            .collect()
    };
    let points = |knobs: &ComponentKnobs| knobs.points.iter().map(|p| Vec2::<()>::new(p.x, p.y)).collect();
    // orbital components own their center knob - the marker mirrors it
    let centers = |count: usize| -> Vec<CenterMark> {
        (0..count)
            .filter(|&slot| ORBITAL.contains(&name(slot)))
            .map(|slot| CenterMark {
                point: Vec2::<()>::new(knobs.components[slot].center[0], knobs.components[slot].center[1]),
                slot,
            })
            .collect()
    };
    let duration = knobs.duration;
    let linear = EasingFunction::Linear;
    match roles {
        ["x"] => {
            let (trajectory, phase) = scalar(0);
            Demo::Scalar {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                easings: easings(1),
            }
        }
        ["xy"] => {
            let (trajectory, phase) = planar(0);
            Demo::Planar {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(1),
                easings: easings(1),
            }
        }
        ["x", "y"] => {
            let (x, phase) = scalar(0);
            let (y, _) = scalar(1);
            Demo::Planar {
                animation: TimedAnimation::new(Box::new(Compose2 { x, y }), duration, linear),
                phase,
                control_points: Vec::new(),
                centers: Vec::new(),
                easings: easings(2),
            }
        }
        ["xyz"] => {
            let (trajectory, phase) = spatial(0);
            Demo::Spatial {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(1),
                easings: easings(1),
            }
        }
        ["x", "y", "z"] => {
            let (x, phase) = scalar(0);
            let (y, _) = scalar(1);
            let (z, _) = scalar(2);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeXY { xy: Compose2 { x, y }, z }), duration, linear),
                phase,
                control_points: Vec::new(),
                centers: Vec::new(),
                easings: easings(3),
            }
        }
        ["xy", "z"] => {
            let (xy, phase) = planar(0);
            let (z, _) = scalar(1);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeXY { xy, z }), duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(2),
                easings: easings(2),
            }
        }
        ["x", "yz"] => {
            let (x, phase) = scalar(0);
            let (yz, _) = planar(1);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeYZ { x, yz }), duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(1)) {
                    points(&knobs.components[1])
                } else {
                    Vec::new()
                },
                centers: centers(2),
                easings: easings(2),
            }
        }
        _ => unreachable!("component_roles covers every composition"),
    }
}

/// The plot domain `(x_min, x_max, y_min, y_max)` - fitted to the
/// sampled trajectory, the per-component easing curves and the
/// markers. Overshoot, back and elastic leave `0..=1` by design.
fn plot_domain(demo: &Demo) -> (f64, f64, f64, f64) {
    let mut x_min = 0.0_f64;
    let mut x_max = 1.0_f64;
    let mut y_min = 0.0_f64;
    let mut y_max = 1.0_f64;
    for i in 0..=200 {
        let t = Fraction::new(i as f64 / 200.0);
        for easing in demo.easings() {
            let eased = easing.apply(t).as_f64();
            if eased.is_finite() {
                y_min = y_min.min(eased);
                y_max = y_max.max(eased);
            }
        }
        // the easings live inside the trajectories - the linear
        // sample already reflects them
        let value = demo.sample(t);
        x_min = x_min.min(value.x);
        x_max = x_max.max(value.x);
        y_min = y_min.min(value.y);
        y_max = y_max.max(value.y);
        for point in demo.control_points() {
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
        }
    }
    for mark in demo.centers() {
        x_min = x_min.min(mark.point.x);
        x_max = x_max.max(mark.point.x);
        y_min = y_min.min(mark.point.y);
        y_max = y_max.max(mark.point.y);
    }
    let x_pad = 0.25 * (x_max - x_min).max(0.2);
    let y_pad = 0.25 * (y_max - y_min).max(0.2);
    (x_min - x_pad, x_max + x_pad, y_min - y_pad, y_max + y_pad)
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

        // the value dimension and how it splits into components
        let dimensions_box = GtkBox::new(Orientation::Vertical, 6);
        let (_dimension_row, dimension) = dropdown_row(&dimensions_box, "Axes", &["1D", "2D", "3D"]);
        let (_composition_row, composition) = dropdown_row(&dimensions_box, "Split", compositions(0));
        controls.append(&group_frame("Dimensions", &dimensions_box));

        // one column per split component - hidden slots stay invisible
        let columns = GtkBox::new(Orientation::Horizontal, 4);
        columns.set_homogeneous(true);
        controls.append(&columns);
        let panels: Rc<Vec<ComponentPanel>> = Rc::new((0..3).map(|_| component_panel(&columns)).collect());

        let playback = playback_group(&controls);

        // --- state ------------------------------------------------------

        let knobs = Rc::new({
            let (duration, dimension, composition) = (playback.duration.clone(), dimension.clone(), composition.clone());
            let panels = Rc::clone(&panels);
            move || Knobs {
                duration: Duration::from_secs_f64(duration.value()),
                dimension: dimension.selected(),
                composition: composition.selected(),
                components: [component_knobs(&panels[0]), component_knobs(&panels[1]), component_knobs(&panels[2])],
            }
        });

        let demo = Rc::new(RefCell::new(build_demo(&knobs())));
        let latest = Rc::new(Cell::new(None::<(Vec2<()>, f64)>));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));
        let logo = gtk4::cairo::ImageSurface::create_from_png(&mut std::io::Cursor::new(include_bytes!("assets/smearor.png"))).ok();

        // Visibility + rebuild: only the rows each component's
        // selection actually reads stay visible.
        let update_visibility = {
            let panels = Rc::clone(&panels);
            let dimension = dimension.clone();
            let composition = composition.clone();
            // remembers the roles the panels were last configured
            // for - model swaps fire `selected_notify`, so the
            // re-entered update must not run again
            let last_dimension = Cell::new(u32::MAX);
            let last_roles: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
            let updating = Cell::new(false);
            let knobs = Rc::clone(&knobs);
            let demo = Rc::clone(&demo);
            let hold = Rc::clone(&hold);
            move || {
                if updating.replace(true) {
                    return;
                }
                let dim = dimension.selected();
                if dim != last_dimension.replace(dim) {
                    // the dimension's own split options
                    composition.set_model(Some(&StringList::new(compositions(dim))));
                    composition.set_selected(0);
                }
                let roles = component_roles(dim, composition.selected());
                {
                    let mut last_roles = last_roles.borrow_mut();
                    for (i, panel) in panels.iter().enumerate() {
                        match roles.get(i) {
                            Some(role) => {
                                panel.column.set_visible(true);
                                panel.title.set_markup(&format!("<span size=\"large\" weight=\"bold\">Axis {role}</span>"));
                                if last_roles.get(i) != Some(role) {
                                    panel.trajectory.set_model(Some(&StringList::new(role_choices(role))));
                                    panel.trajectory.set_selected(0);
                                }
                                let name = role_choices(role)[panel.trajectory.selected() as usize];
                                let (_, params) = easing_by_name(EASINGS[panel.easing.selected() as usize], &component_knobs(panel));
                                update_panel(panel, name, params, role.len() == 3);
                            }
                            None => panel.column.set_visible(false),
                        }
                    }
                    *last_roles = roles.to_vec();
                }
                hold.set(None);
                let mut animation = demo.borrow_mut();
                *animation = build_demo(&knobs());
                animation.start();
                updating.set(false);
            }
        };

        // Rebuild + restart on every control change.
        let restart: Rc<dyn Fn()> = Rc::new({
            let update_visibility = update_visibility.clone();
            move || update_visibility()
        });
        for panel in panels.iter() {
            connect_panel(panel, &restart);
        }
        {
            let restart = Rc::clone(&restart);
            playback.duration.connect_value_changed(move |_| restart());
        }
        for dropdown in [&dimension, &composition] {
            let restart = Rc::clone(&restart);
            dropdown.connect_selected_notify(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            playback.replay.connect_clicked(move |_| restart());
        }

        // --- frame clock -------------------------------------------------

        install_tick(&canvas, Rc::clone(&demo), &playback, Rc::clone(&hold), {
            let latest = Rc::clone(&latest);
            move |animation| latest.set(animation.value())
        });

        canvas.set_draw_func({
            let demo = Rc::clone(&demo);
            let latest = Rc::clone(&latest);
            let logo = logo.clone();
            move |_, context, width, height| {
                let (w, h) = (f64::from(width), f64::from(height));
                let margin = 40.0;
                let (pw, ph) = (w - 2.0 * margin, h - 2.0 * margin);
                let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                let to_x = |v: f64| margin + pw * (v - x_min) / (x_max - x_min);
                let to_y = |v: f64| margin + ph * (1.0 - (v - y_min) / (y_max - y_min));

                context.set_source_rgb(0.92, 0.92, 0.92);
                context.rectangle(margin, margin, pw, ph);
                let _ = context.fill();
                // the 0 and 1 reference lines stand out on both axes
                context.set_source_rgb(0.75, 0.75, 0.75);
                for v in [0.0, 1.0] {
                    let gy = to_y(v);
                    context.move_to(margin, gy);
                    context.line_to(margin + pw, gy);
                    let gx = to_x(v);
                    context.move_to(gx, margin);
                    context.line_to(gx, margin + ph);
                }
                context.set_line_width(1.5);
                let _ = context.stroke();
                let _ = context.save();
                context.rectangle(margin, margin, pw, ph);
                context.clip();
                {
                    // one curve per component easing - a distinct
                    // shade per axis group
                    const CURVE_COLORS: [(f64, f64, f64); 3] = [(0.72, 0.72, 0.86), (0.86, 0.7, 0.66), (0.66, 0.8, 0.7)];
                    for (slot, easing) in demo.borrow().easings().iter().enumerate() {
                        let (r, g, b) = CURVE_COLORS[slot % CURVE_COLORS.len()];
                        context.set_source_rgb(r, g, b);
                        for i in 0..=200 {
                            let t = Fraction::new(i as f64 / 200.0);
                            let eased = easing.apply(t).as_f64();
                            if !eased.is_finite() {
                                continue;
                            }
                            let (x, y) = (to_x(t.as_f64()), to_y(eased));
                            if i == 0 { context.move_to(x, y) } else { context.line_to(x, y) }
                        }
                    }
                    context.set_line_width(2.0);
                    let _ = context.stroke();
                }
                let _ = context.restore();

                // The animated content on top of the curve.
                let animation = demo.borrow();
                for point in animation.control_points() {
                    let (x, y) = (to_x(point.x), to_y(point.y));
                    context.arc(x, y, 4.0, 0.0, std::f64::consts::TAU);
                    context.set_source_rgb(0.5, 0.5, 0.5);
                    let _ = context.fill();
                }
                // one draggable center crosshair per orbital component,
                // in its easing curve's color but darker
                const MARK_COLORS: [(f64, f64, f64); 3] = [(0.4, 0.4, 0.62), (0.62, 0.36, 0.3), (0.32, 0.52, 0.36)];
                for mark in animation.centers() {
                    let (x, y) = (to_x(mark.point.x), to_y(mark.point.y));
                    let (r, g, b) = MARK_COLORS[mark.slot % MARK_COLORS.len()];
                    context.set_source_rgb(r, g, b);
                    context.set_line_width(1.5);
                    context.arc(x, y, 7.0, 0.0, std::f64::consts::TAU);
                    let _ = context.stroke();
                    context.move_to(x - 10.0, y);
                    context.line_to(x + 10.0, y);
                    context.move_to(x, y - 10.0);
                    context.line_to(x, y + 10.0);
                    let _ = context.stroke();
                }
                if let Some((point, scale)) = latest.get() {
                    let (x, y) = (to_x(point.x), to_y(point.y));
                    match &logo {
                        Some(logo) => {
                            let logo_w = 192.0 * scale;
                            let logo_h = logo_w * f64::from(logo.height()) / f64::from(logo.width());
                            let _ = context.save();
                            context.translate(x - logo_w / 2.0, y - logo_h / 2.0);
                            context.scale(logo_w / f64::from(logo.width()), logo_h / f64::from(logo.height()));
                            let _ = context.set_source_surface(logo, 0.0, 0.0);
                            let _ = context.paint();
                            let _ = context.restore();
                        }
                        None => {
                            context.arc(x, y, 10.0, 0.0, std::f64::consts::TAU);
                            context.set_source_rgb(0.2, 0.4, 0.8);
                            let _ = context.fill();
                        }
                    }
                }
                let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0);
                let eased: Vec<String> = animation
                    .easings()
                    .iter()
                    .map(|e| format!("{:.2}", e.apply(Fraction::new(progress)).as_f64()))
                    .collect();
                // cairo text, not a Label - widget updates inside a
                // draw callback queue a resize every frame
                context.set_source_rgb(0.35, 0.35, 0.35);
                context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                context.set_font_size(13.0);
                context.move_to(margin, margin - 14.0);
                let _ = context.show_text(&format!("t {progress:.2}   eased {}", eased.join(" | ")));
            }
        });

        // Dragging a center crosshair rewrites that component's
        // Center spins - the spin change rebuilds the demo, which
        // moves the marker under the pointer.
        {
            let grabbed = Rc::new(Cell::new(usize::MAX));
            let drag = gtk4::GestureDrag::new();
            {
                let (demo, grabbed) = (Rc::clone(&demo), Rc::clone(&grabbed));
                drag.connect_drag_begin(move |drag, x, y| {
                    let Some(canvas) = drag.widget() else { return };
                    let (w, h) = (f64::from(canvas.width()), f64::from(canvas.height()));
                    let margin = 40.0;
                    let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                    let to_px = |vx: f64, vy: f64| {
                        (
                            margin + (w - 2.0 * margin) * (vx - x_min) / (x_max - x_min),
                            margin + (h - 2.0 * margin) * (1.0 - (vy - y_min) / (y_max - y_min)),
                        )
                    };
                    // the nearest marker inside the grab radius wins
                    let mut best = usize::MAX;
                    let mut best_dist = 24.0_f64;
                    for mark in demo.borrow().centers() {
                        let (mx, my) = to_px(mark.point.x, mark.point.y);
                        let dist = ((mx - x).powi(2) + (my - y).powi(2)).sqrt();
                        if dist < best_dist {
                            best_dist = dist;
                            best = mark.slot;
                        }
                    }
                    grabbed.set(best);
                });
            }
            {
                let (demo, panels, grabbed) = (Rc::clone(&demo), Rc::clone(&panels), Rc::clone(&grabbed));
                drag.connect_drag_update(move |drag, dx, dy| {
                    let slot = grabbed.get();
                    let (Some((start_x, start_y)), Some(canvas)) = (drag.start_point(), drag.widget()) else {
                        return;
                    };
                    if slot == usize::MAX {
                        return;
                    }
                    let (w, h) = (f64::from(canvas.width()), f64::from(canvas.height()));
                    let margin = 40.0;
                    let (x, y) = {
                        // the borrow must end before the spins fire the
                        // restart that rebuilds `demo`
                        let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                        let (px, py) = (start_x + dx - margin, start_y + dy - margin);
                        (x_min + px / (w - 2.0 * margin) * (x_max - x_min), y_min + (1.0 - py / (h - 2.0 * margin)) * (y_max - y_min))
                    };
                    // the Center spin range clamps the drag target
                    panels[slot].center[0].set_value(x.clamp(-0.5, 1.5));
                    panels[slot].center[1].set_value(y.clamp(-0.5, 1.5));
                });
            }
            {
                let grabbed = Rc::clone(&grabbed);
                drag.connect_drag_end(move |_, _, _| grabbed.set(usize::MAX));
            }
            canvas.add_controller(drag);
        }

        let view = paned_view(&canvas, &controls);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Animation Playground")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        restart();
        window.present();
    });
    app.run()
}
