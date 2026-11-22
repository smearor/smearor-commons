//! Geometry playground: a primitive's aspects - positions, radii,
//! arc angles - are driven by independent trajectories. Positions
//! split per axis: a center can run one planar path or separate
//! scalar trajectories on x/y/z. A line's endpoints orbit, an arc
//! sweeps open and closed, a polygon's vertices crawl along a
//! shared path like a snake.
//!
//! Run with `cargo run -p smearor-commons-animation --features gtk4 --example geometry`.

mod common;

use common::Animatable;
use common::ComponentKnobs;
use common::ComponentPanel;
use common::Compose2;
use common::ComposeXY;
use common::ComposeYZ;
use common::EASINGS;
use common::Effect;
use common::LiftVec2;
use common::PhaseShape;
use common::apply_spec;
use common::build_scalar_trajectory;
use common::component_knobs;
use common::component_panel;
use common::component_roles;
use common::compositions;
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
use common::planar_trajectory;
use common::playback_group;
use common::role_choices;
use common::save_effect;
use common::select_by_name;
use common::selected_name;
use common::slider;
use common::spatial_trajectory;
use common::spec_for_panel;
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

const APP_ID: &str = "io.smearor.commons_animation.geometry";

/// The selectable primitives.
const PRIMITIVES: &[&str] = &["Line", "Arc", "Annulus", "Polyline", "Polygon", "Rectangle", "Circle"];

/// The position space - 2D stays planar, 3D adds the projected `z`.
const SPACES: &[&str] = &["2D", "3D"];

/// A nicer default than `Span` for a fresh trajectory model - a
/// scalar `Span` on a position is the plain diagonal, positions
/// start on an orbit or helix instead.
fn default_trajectory_index(choices: &[&str]) -> u32 {
    for preferred in ["Orbit", "Helix"] {
        if let Some(index) = choices.iter().position(|name| *name == preferred) {
            return index as u32;
        }
    }
    0
}

/// The primitive's aspects `(label, is_position)` - positions take
/// the selected space's roles, scalars stay one column each.
fn aspects(primitive: u32) -> &'static [(&'static str, bool)] {
    match primitive {
        // Line - two independently driven endpoints
        0 => &[("A", true), ("B", true)],
        // Arc - center, radius and the start/sweep angles
        1 => &[("Center", true), ("Radius", false), ("Start", false), ("Sweep", false)],
        // Annulus - center, both radii and the sector sweep
        2 => &[("Center", true), ("Outer", false), ("Inner", false), ("Sweep", false)],
        // Polyline + Polygon - one shared path, vertices sampled
        // at spread phase offsets
        3 | 4 => &[("Path", true)],
        // Rectangle - center plus the two side lengths
        5 => &[("Center", true), ("Width", false), ("Height", false)],
        // Circle - center and radius
        _ => &[("Center", true), ("Radius", false)],
    }
}

/// One sidebar column: the aspect it belongs to, its title and the
/// trajectory role (single letters scalar, `"xy"`/`"xyz"` native).
struct Column {
    /// The aspect index inside `aspects()`.
    aspect: usize,
    /// The column heading - `Center x`, `A xy` or `Radius`.
    title: String,
    /// The role driving `role_choices` and the value type.
    role: &'static str,
}

/// Expands the aspects into columns - positions fan out over the
/// space's component roles, scalars stay single.
fn columns(primitive: u32, dimension: u32, composition: u32) -> Vec<Column> {
    let mut columns = Vec::new();
    for (i, (label, is_position)) in aspects(primitive).iter().enumerate() {
        if !is_position {
            columns.push(Column {
                aspect: i,
                title: label.to_string(),
                role: "x",
            });
            continue;
        }
        for role in component_roles(dimension, composition) {
            columns.push(Column {
                aspect: i,
                title: format!("{label} {role}"),
                role,
            });
        }
    }
    columns
}

/// One column's boxed trajectory - the value type follows the role.
enum Sub {
    /// A scalar role - drives one axis or a scalar aspect.
    S(Box<dyn Trajectory<Value = f64>>),
    /// A planar role - drives `x`/`y` or `y`/`z`.
    P(Box<dyn Trajectory<Value = Vec2<()>>>),
    /// A spatial role - drives `x`/`y`/`z`.
    V(Box<dyn Trajectory<Value = Vec3<()>>>),
}

/// One animated aspect of a primitive - a 3D position or a scalar.
enum Aspect {
    /// A composed position trajectory - `z` projects into the
    /// oblique offset, 2D positions sit at mid depth.
    Position(TimedAnimation<Box<dyn Trajectory<Value = Vec3<()>>>>),
    /// A scalar trajectory - a radius, angle or size.
    Scalar(TimedAnimation<Box<dyn Trajectory<Value = f64>>>),
}

/// The animated primitive - one timed animation per aspect.
struct GeoDemo {
    /// The selected primitive - indexes `PRIMITIVES`.
    primitive: u32,
    /// One animation per aspect, in `aspects()` order.
    aspects: Vec<Aspect>,
    /// `Polyline`/`Polygon` vertex count.
    verts: usize,
    /// `Polyline`/`Polygon` phase offset between neighbors.
    spread: f64,
    /// The segment layout for the phase bar - column 0's shape.
    phase: PhaseShape,
    /// The per-column easings.
    easings: Vec<EasingFunction>,
}

/// The oblique projection - `z` shifts up-right around mid depth,
/// so 2D positions project unchanged and helix/knot paths show
/// their depth.
fn project(p: Vec3<()>) -> Vec2<()> {
    Vec2::new(p.x + (p.z - 0.5) * 0.35, p.y - (p.z - 0.5) * 0.3)
}

impl GeoDemo {
    /// The current projected position of an aspect.
    fn point(&self, i: usize) -> Vec2<()> {
        match self.aspects.get(i) {
            Some(Aspect::Position(animation)) => project(animation.value().unwrap_or_else(|| Vec3::new(0.5, 0.5, 0.5))),
            _ => Vec2::new(0.5, 0.5),
        }
    }

    /// The current value of a scalar aspect.
    fn scalar(&self, i: usize) -> f64 {
        match self.aspects.get(i) {
            Some(Aspect::Scalar(animation)) => animation.value().unwrap_or(0.5),
            _ => 0.5,
        }
    }

    /// The projected trajectory curve of a position aspect, sampled
    /// `steps` times - drawn as the faint trail it rides on.
    fn trail(&self, i: usize, steps: usize) -> Vec<Vec2<()>> {
        match self.aspects.get(i) {
            Some(Aspect::Position(animation)) => (0..=steps)
                .map(|k| project(animation.trajectory.value_at(Fraction::new(k as f64 / steps as f64))))
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl Animatable for GeoDemo {
    fn is_complete(&self) -> bool {
        // all aspects share the duration and start together -
        // aspect 0 speaks for the whole demo
        match self.aspects.first() {
            Some(Aspect::Position(animation)) => animation.is_complete(),
            Some(Aspect::Scalar(animation)) => animation.is_complete(),
            None => true,
        }
    }

    fn start(&mut self) {
        for aspect in &mut self.aspects {
            match aspect {
                Aspect::Position(animation) => animation.start(),
                Aspect::Scalar(animation) => animation.start(),
            }
        }
    }

    fn seek(&mut self, progress: Fraction) {
        for aspect in &mut self.aspects {
            match aspect {
                Aspect::Position(animation) => animation.seek(progress),
                Aspect::Scalar(animation) => animation.seek(progress),
            }
        }
    }

    fn progress(&self) -> Option<Fraction> {
        match self.aspects.first() {
            Some(Aspect::Position(animation)) => animation.progress(),
            Some(Aspect::Scalar(animation)) => animation.progress(),
            None => None,
        }
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

/// The global parameters - primitive, space/split, shape sliders
/// and timing.
struct Knobs {
    duration: Duration,
    primitive: u32,
    space: u32,
    split: u32,
    verts: usize,
    spread: f64,
    components: Vec<ComponentKnobs>,
}

/// Composes one aspect's role trajectories into a `Vec3` position
/// trajectory - same per-axis assembly the playground uses for its
/// split compositions, 2D roles sit at mid depth.
fn position_trajectory(roles: &[&str], subs: Vec<Sub>) -> Option<Box<dyn Trajectory<Value = Vec3<()>>>> {
    let mut subs = subs.into_iter();
    let next = |subs: &mut std::vec::IntoIter<Sub>| subs.next();
    match roles {
        ["xy"] => match next(&mut subs) {
            Some(Sub::P(xy)) => Some(Box::new(LiftVec2(xy))),
            _ => None,
        },
        ["x", "y"] => match (next(&mut subs), next(&mut subs)) {
            (Some(Sub::S(x)), Some(Sub::S(y))) => Some(Box::new(LiftVec2(Compose2 { x, y }))),
            _ => None,
        },
        ["xyz"] => next(&mut subs).and_then(|sub| match sub {
            Sub::V(xyz) => Some(xyz),
            _ => None,
        }),
        ["x", "y", "z"] => match (next(&mut subs), next(&mut subs), next(&mut subs)) {
            (Some(Sub::S(x)), Some(Sub::S(y)), Some(Sub::S(z))) => Some(Box::new(ComposeXY { xy: Compose2 { x, y }, z })),
            _ => None,
        },
        ["xy", "z"] => match (next(&mut subs), next(&mut subs)) {
            (Some(Sub::P(xy)), Some(Sub::S(z))) => Some(Box::new(ComposeXY { xy, z })),
            _ => None,
        },
        ["x", "yz"] => match (next(&mut subs), next(&mut subs)) {
            (Some(Sub::S(x)), Some(Sub::P(yz))) => Some(Box::new(ComposeYZ { x, yz })),
            _ => None,
        },
        _ => None,
    }
}

/// Builds the animated primitive - every column contributes its
/// eased trajectory to its aspect, roles compose into positions.
fn build_demo(knobs: &Knobs) -> GeoDemo {
    let cols = columns(knobs.primitive, knobs.space, knobs.split);
    let aspect_count = aspects(knobs.primitive).len();
    let mut phase = PhaseShape::Identity;
    let mut roles: Vec<Vec<&'static str>> = vec![Vec::new(); aspect_count];
    let mut subs: Vec<Vec<Sub>> = (0..aspect_count).map(|_| Vec::new()).collect();
    for (slot, column) in cols.iter().enumerate() {
        let component = &knobs.components[slot];
        let name = role_choices(column.role)[component.trajectory as usize];
        roles[column.aspect].push(column.role);
        match column.role.len() {
            2 => {
                let (trajectory, shape) = planar_trajectory(name, component);
                if slot == 0 {
                    phase = shape;
                }
                subs[column.aspect].push(Sub::P(eased(trajectory, component)));
            }
            3 => {
                let (trajectory, shape) = spatial_trajectory(name, component);
                if slot == 0 {
                    phase = shape;
                }
                subs[column.aspect].push(Sub::V(eased(trajectory, component)));
            }
            _ => {
                let (trajectory, shape) = build_scalar_trajectory(name, component);
                if slot == 0 {
                    phase = shape;
                }
                subs[column.aspect].push(Sub::S(eased(trajectory, component)));
            }
        }
    }
    let mut built = Vec::with_capacity(aspect_count);
    for (i, (_, is_position)) in aspects(knobs.primitive).iter().enumerate() {
        if *is_position {
            if let Some(trajectory) = position_trajectory(&roles[i], std::mem::take(&mut subs[i])) {
                built.push(Aspect::Position(TimedAnimation::new(trajectory, knobs.duration, EasingFunction::Linear)));
            }
        } else if let Some(Sub::S(trajectory)) = subs[i].drain(..).next() {
            built.push(Aspect::Scalar(TimedAnimation::new(trajectory, knobs.duration, EasingFunction::Linear)));
        }
    }
    let easings = cols
        .iter()
        .enumerate()
        .map(|(slot, _)| {
            let component = &knobs.components[slot];
            easing_by_name(EASINGS[component.easing as usize], component).0
        })
        .collect();
    GeoDemo {
        primitive: knobs.primitive,
        aspects: built,
        verts: knobs.verts,
        spread: knobs.spread,
        phase,
        easings,
    }
}

/// One column's shade for position trails.
const TRAIL_COLORS: [(f64, f64, f64); 6] = [
    (0.72, 0.72, 0.86),
    (0.86, 0.7, 0.66),
    (0.66, 0.8, 0.7),
    (0.82, 0.82, 0.6),
    (0.7, 0.76, 0.86),
    (0.84, 0.68, 0.78),
];

/// The vertices of a `Polyline`/`Polygon`: the shared path sampled
/// at `t + i·spread` - the shape crawls along its own trajectory.
fn snake_vertices(demo: &GeoDemo) -> Vec<Vec2<()>> {
    let t = demo.progress().map(|p| p.as_f64()).unwrap_or(0.0);
    match demo.aspects.first() {
        Some(Aspect::Position(animation)) => {
            let trajectory = &animation.trajectory;
            (0..demo.verts.max(2))
                .map(|i| project(trajectory.value_at(Fraction::new(t + i as f64 * demo.spread))))
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Draws the primitive - `to_px` maps the projected model space
/// into pixels, `px_len` converts a model length along `x`.
fn draw_primitive(context: &gtk4::cairo::Context, demo: &GeoDemo, to_px: impl Fn(Vec2<()>) -> (f64, f64), px_len: impl Fn(f64) -> f64) {
    context.set_source_rgb(0.2, 0.4, 0.8);
    context.set_line_width(2.5);
    match demo.primitive {
        // Line
        0 => {
            let (ax, ay) = to_px(demo.point(0));
            let (bx, by) = to_px(demo.point(1));
            context.move_to(ax, ay);
            context.line_to(bx, by);
            let _ = context.stroke();
            for (x, y) in [(ax, ay), (bx, by)] {
                context.arc(x, y, 5.0, 0.0, std::f64::consts::TAU);
                let _ = context.fill();
            }
        }
        // Arc - start and sweep are turns of a full circle
        1 => {
            let (cx, cy) = to_px(demo.point(0));
            let radius = px_len(demo.scalar(1).abs() * 0.4);
            let start = demo.scalar(2) * std::f64::consts::TAU;
            let sweep = demo.scalar(3) * std::f64::consts::TAU;
            let (sx, sy) = (cx + radius * start.cos(), cy + radius * start.sin());
            context.arc(cx, cy, radius, start, start + sweep);
            let _ = context.stroke();
            context.arc(cx, cy, 4.0, 0.0, std::f64::consts::TAU);
            let _ = context.fill();
            // the radius arm marks the start angle
            context.move_to(cx, cy);
            context.line_to(sx, sy);
            context.set_line_width(1.0);
            let _ = context.stroke();
        }
        // Annulus - outer arc forward, inner arc back
        2 => {
            let (cx, cy) = to_px(demo.point(0));
            let outer = px_len(demo.scalar(1).abs() * 0.4);
            let inner = px_len(demo.scalar(2).abs().min(demo.scalar(1).abs()) * 0.4);
            let start = -std::f64::consts::FRAC_PI_2;
            let sweep = demo.scalar(3) * std::f64::consts::TAU;
            context.arc(cx, cy, outer, start, start + sweep);
            context.arc_negative(cx, cy, inner, start + sweep, start);
            context.close_path();
            let _ = context.save();
            context.set_source_rgba(0.2, 0.4, 0.8, 0.25);
            let _ = context.fill_preserve();
            let _ = context.restore();
            context.set_source_rgb(0.2, 0.4, 0.8);
            let _ = context.stroke();
        }
        // Polyline + Polygon - vertices crawling along the path
        3 | 4 => {
            let vertices = snake_vertices(demo);
            if let Some(first) = vertices.first().map(|p| to_px(*p)) {
                context.move_to(first.0, first.1);
                for vertex in vertices.iter().skip(1).map(|p| to_px(*p)) {
                    context.line_to(vertex.0, vertex.1);
                }
                if demo.primitive == 4 {
                    context.close_path();
                    let _ = context.save();
                    context.set_source_rgba(0.2, 0.4, 0.8, 0.15);
                    let _ = context.fill_preserve();
                    let _ = context.restore();
                    context.set_source_rgb(0.2, 0.4, 0.8);
                }
                let _ = context.stroke();
                for vertex in &vertices {
                    let (x, y) = to_px(*vertex);
                    context.arc(x, y, 3.5, 0.0, std::f64::consts::TAU);
                    let _ = context.fill();
                }
            }
        }
        // Rectangle - center plus both side lengths
        5 => {
            let (cx, cy) = to_px(demo.point(0));
            let (px_w, px_h) = (px_len(demo.scalar(1).abs() * 0.7), px_len(demo.scalar(2).abs() * 0.7));
            context.rectangle(cx - px_w / 2.0, cy - px_h / 2.0, px_w, px_h);
            let _ = context.stroke();
        }
        // Circle
        _ => {
            let (cx, cy) = to_px(demo.point(0));
            context.arc(cx, cy, px_len(demo.scalar(1).abs() * 0.4), 0.0, std::f64::consts::TAU);
            let _ = context.stroke();
            context.arc(cx, cy, 4.0, 0.0, std::f64::consts::TAU);
            let _ = context.fill();
        }
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

        let shape_box = GtkBox::new(Orientation::Vertical, 6);
        let (_primitive_row, primitive) = dropdown_row(&shape_box, "Primitive", PRIMITIVES);
        let (_space_row, space) = dropdown_row(&shape_box, "Space", SPACES);
        let (_split_row, split) = dropdown_row(&shape_box, "Split", compositions(1));
        let (verts_row, verts) = slider(&shape_box, "Verts", 2.0, 16.0, 6.0, 0);
        verts.set_round_digits(0);
        for mark in 2..=16 {
            verts.add_mark(f64::from(mark), gtk4::PositionType::Bottom, (mark % 2 == 0).then(|| mark.to_string()).as_deref());
        }
        let (spread_row, spread) = slider(&shape_box, "Spread", 0.0, 0.5, 0.12, 2);
        for mark in [0.0, 0.25, 0.5] {
            spread.add_mark(mark, gtk4::PositionType::Bottom, Some(&format!("{mark:.2}")));
        }
        controls.append(&group_frame("Primitive", &shape_box));

        // up to six aspect/axis columns - hidden slots stay invisible
        let columns_box = GtkBox::new(Orientation::Horizontal, 4);
        columns_box.set_homogeneous(true);
        controls.append(&columns_box);
        let panels: Rc<Vec<ComponentPanel>> = Rc::new((0..6).map(|_| component_panel(&columns_box)).collect());

        let playback = playback_group(&controls);
        let effects = effect_group(&controls);

        // --- state ------------------------------------------------------

        let knobs = Rc::new({
            let (duration, primitive, space, split, verts, spread) =
                (playback.duration.clone(), primitive.clone(), space.clone(), split.clone(), verts.clone(), spread.clone());
            let panels = Rc::clone(&panels);
            move || Knobs {
                duration: Duration::from_secs_f64(duration.value()),
                primitive: primitive.selected(),
                // the space dropdown indexes like the playground's
                // dimensions: 0 -> 2D roles, 1 -> 3D roles
                space: space.selected() + 1,
                split: split.selected(),
                verts: verts.value().max(2.0) as usize,
                spread: spread.value(),
                components: panels.iter().map(component_knobs).collect(),
            }
        });

        let demo = Rc::new(RefCell::new(build_demo(&knobs())));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));

        // Visibility + rebuild: column count, titles and trajectory
        // models follow the primitive's aspects and the space split.
        let update_visibility = {
            let panels = Rc::clone(&panels);
            let (primitive, space, split) = (primitive.clone(), space.clone(), split.clone());
            let (verts_row, spread_row) = (verts_row.clone(), spread_row.clone());
            let last_space = Cell::new(u32::MAX);
            let last_roles: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
            let updating = Cell::new(false);
            let knobs = Rc::clone(&knobs);
            let demo = Rc::clone(&demo);
            let hold = Rc::clone(&hold);
            move || {
                if updating.replace(true) {
                    return;
                }
                let dim = space.selected() + 1;
                if dim != last_space.replace(dim) {
                    // the space's own split options
                    split.set_model(Some(&StringList::new(compositions(dim))));
                    split.set_selected(0);
                }
                let cols = columns(primitive.selected(), dim, split.selected());
                let is_path = matches!(primitive.selected(), 3 | 4);
                verts_row.set_visible(is_path);
                spread_row.set_visible(is_path);
                {
                    let mut last_roles = last_roles.borrow_mut();
                    for (i, panel) in panels.iter().enumerate() {
                        match cols.get(i) {
                            Some(column) => {
                                panel.column.set_visible(true);
                                panel.title.set_markup(&format!("<span size=\"large\" weight=\"bold\">{}</span>", column.title));
                                if last_roles.get(i) != Some(&column.role) {
                                    panel.trajectory.set_model(Some(&StringList::new(role_choices(column.role))));
                                    panel.trajectory.set_selected(default_trajectory_index(role_choices(column.role)));
                                }
                                let name = role_choices(column.role)[panel.trajectory.selected() as usize];
                                let (_, params) = easing_by_name(EASINGS[panel.easing.selected() as usize], &component_knobs(panel));
                                update_panel(panel, name, params, column.role.len() == 3);
                            }
                            None => panel.column.set_visible(false),
                        }
                    }
                    *last_roles = cols.iter().map(|c| c.role).collect();
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
        for scale in [&playback.duration, &verts, &spread] {
            let restart = Rc::clone(&restart);
            scale.connect_value_changed(move |_| restart());
        }
        for dropdown in [&primitive, &space, &split] {
            let restart = Rc::clone(&restart);
            dropdown.connect_selected_notify(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            playback.replay.connect_clicked(move |_| restart());
        }

        // --- effect: save / load / emit -----------------------------------

        let current_effect = {
            let (primitive, space, split, verts, spread) = (primitive.clone(), space.clone(), split.clone(), verts.clone(), spread.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            move || {
                let cols = columns(primitive.selected(), space.selected() + 1, split.selected());
                let mut options = std::collections::BTreeMap::new();
                options.insert("space".to_string(), serde_json::json!(SPACES[space.selected() as usize]));
                options.insert("split".to_string(), serde_json::json!(compositions(space.selected() + 1)[split.selected() as usize]));
                options.insert("verts".to_string(), serde_json::json!(verts.value()));
                options.insert("spread".to_string(), serde_json::json!(spread.value()));
                Effect {
                    demo: "geometry".into(),
                    object: selected_name(&primitive, PRIMITIVES),
                    options,
                    duration_secs: duration.value(),
                    components: cols
                        .iter()
                        .zip(panels.iter())
                        .map(|(column, panel)| spec_for_panel(&column.title, column.role, panel))
                        .collect(),
                }
            }
        };

        let apply_effect = {
            let (primitive, space, split, verts, spread) = (primitive.clone(), space.clone(), split.clone(), verts.clone(), spread.clone());
            let duration = playback.duration.clone();
            let panels = Rc::clone(&panels);
            let restart = Rc::clone(&restart);
            move |effect: &Effect| {
                select_by_name(&primitive, PRIMITIVES, &effect.object);
                if let Some(name) = effect.options.get("space").and_then(|v| v.as_str()) {
                    select_by_name(&space, SPACES, name);
                }
                // the space rebuilds the split model before the
                // stored split can be selected by name
                restart();
                if let Some(name) = effect.options.get("split").and_then(|v| v.as_str()) {
                    select_by_name(&split, compositions(space.selected() + 1), name);
                }
                if let Some(value) = effect.options.get("verts").and_then(|v| v.as_f64()) {
                    verts.set_value(value);
                }
                if let Some(value) = effect.options.get("spread").and_then(|v| v.as_f64()) {
                    spread.set_value(value);
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
                let (pw, ph) = (w - 2.0 * margin, h - 2.0 * margin);
                // -0.3..1.3 domain - overshoot, orbits and the z
                // projection leave the unit square
                const DOMAIN: f64 = 1.6;
                let to_px = move |p: Vec2<()>| (margin + pw * (p.x + 0.3) / DOMAIN, margin + ph * (1.0 - (p.y + 0.3) / DOMAIN));
                let px_len = move |v: f64| v * pw / DOMAIN;

                context.set_source_rgb(0.92, 0.92, 0.92);
                context.rectangle(margin, margin, pw, ph);
                let _ = context.fill();
                context.set_source_rgb(0.75, 0.75, 0.75);
                for v in [0.0, 1.0] {
                    let gx = to_px(Vec2::new(v, 0.0)).0;
                    let gy = to_px(Vec2::new(0.0, v)).1;
                    context.move_to(gx, margin);
                    context.line_to(gx, margin + ph);
                    context.move_to(margin, gy);
                    context.line_to(margin + pw, gy);
                }
                context.set_line_width(1.5);
                let _ = context.stroke();
                let _ = context.save();
                context.rectangle(margin, margin, pw, ph);
                context.clip();
                {
                    // faint trails: the curve each position rides
                    let animation = demo.borrow();
                    for (slot, aspect) in animation.aspects.iter().enumerate() {
                        if let Aspect::Position(_) = aspect {
                            let (r, g, b) = TRAIL_COLORS[slot % TRAIL_COLORS.len()];
                            context.set_source_rgb(r, g, b);
                            for (k, point) in animation.trail(slot, 200).iter().enumerate() {
                                let (x, y) = to_px(*point);
                                if k == 0 { context.move_to(x, y) } else { context.line_to(x, y) }
                            }
                            context.set_line_width(1.5);
                            let _ = context.stroke();
                        }
                    }
                }
                let _ = context.restore();

                let animation = demo.borrow();
                draw_primitive(context, &animation, to_px, px_len);
                let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0);
                context.set_source_rgb(0.35, 0.35, 0.35);
                context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                context.set_font_size(13.0);
                context.move_to(margin, margin - 14.0);
                let _ = context.show_text(&format!("{}   t {progress:.2}", PRIMITIVES[animation.primitive as usize]));
            }
        });

        let view = paned_view(&canvas, &controls);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Geometry Animation")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        restart();
        window.present();
    });
    app.run()
}
