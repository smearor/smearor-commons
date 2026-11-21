//! GTK example: the OSA-UCS color lattice (Optical Society of
//! America, 1974) - regular-rhombohedral (cuboctahedral) sampling of
//! the `L`, `j` (jaune, blue<->yellow) and `g` (green<->red)
//! coordinates.
//!
//! Lattice nodes with `L + j + g` even are drawn as a 3D point cloud
//! that can be rotated by dragging. `L` runs on the vertical axis,
//! `g` and `j` span the horizontal plane.
//!
//! The node colors are a *documented approximation*: `L`, `j`, `g`
//! are mapped linearly onto CIELAB (`L* ~ (L+10)*5`, `a* ~ -5g`,
//! `b* ~ 5j`) rather than through the published MacAdam conversion
//! formulas. Positions, however, follow the real OSA lattice.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example osa`.

mod common;

use common::Camera;
use common::Vec3;
use common::colored_dot;
use common::info_bar;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::DrawingArea;
use gtk4::EventControllerMotion;
use gtk4::GestureClick;
use gtk4::GestureDrag;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_color::Color;
use smearor_commons_color::Hsv;
use smearor_commons_color::Lab;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::cell::RefCell;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::TAU;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_color.osa";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "The Optical Society of America spent thirty years (1947-1977) on a single question: \
    can color space be truly uniform - equal perceptual steps in every direction? \
    The answer is OSA-UCS: a cuboctahedral lattice where every lattice point has twelve equidistant neighbors, \
    the densest uniform arrangement geometry allows. \
    Its coordinates are L (lightness), j (jaune - blue vs. yellow) and g (green vs. red) opponent axes. \
    The lattice was published as a physical sample atlas in 1977 and remains the benchmark for uniform color spacing; \
    later spaces like CIECAM and CAM16 still measure themselves against it.";

/// The OSA lightness range (`-10` black .. `8` white, even steps).
const L_MIN: i32 = -10;
const L_MAX: i32 = 8;
/// The opponent plane extent.
const JG_MAX: i32 = 14;

/// Screen radius for picking a lattice node.
const PICK_RADIUS: f64 = 10.0;

/// The faint reference axes and the mid-plane ring.
const GRID: Color<u8> = Color::<u8>::new(110, 110, 110);

/// Intro animation: yaw step per tick and the pitch pendulum limit.
const INTRO_YAW_STEP: f64 = 0.006;
const INTRO_PITCH_LIMIT: f64 = 30.0_f64.to_radians();

/// One OSA lattice node.
#[derive(Clone, Copy)]
struct OsaNode {
    /// The OSA lightness coordinate.
    l: i32,
    /// The OSA `jaune` coordinate (blue <-> yellow).
    j: i32,
    /// The OSA `green` coordinate (green <-> red).
    g: i32,
    /// The 3D position (`L` vertical, `g`/`j` in the plane).
    position: Vec3,
    /// The approximate sRGB value (see module doc).
    color: Color<u8>,
}

/// The cuboctahedral OSA lattice: `L`, `j`, `g` integers with
/// `L + j + g` even, `L` even.
fn osa_nodes() -> Vec<OsaNode> {
    let mut nodes = Vec::new();
    let mut l = L_MIN;
    while l <= L_MAX {
        for j in -JG_MAX..=JG_MAX {
            for g in -JG_MAX..=JG_MAX {
                if (l + j + g) % 2 != 0 || (j * j + g * g) > JG_MAX * JG_MAX {
                    continue;
                }
                let lab = Lab::new(((l - L_MIN) as f64) * 5.0, -5.0 * g as f64, 5.0 * j as f64);
                nodes.push(OsaNode {
                    l,
                    j,
                    g,
                    position: Vec3::new(g as f64 * 6.0, (l - (L_MIN + L_MAX) / 2) as f64 * 7.0, j as f64 * 6.0),
                    color: Color::from(lab),
                });
            }
        }
        l += 2;
    }
    nodes
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let nodes = Rc::new(osa_nodes());
        let camera = Rc::new(Cell::new(Camera {
            yaw: RotationRad::new(0.5),
            pitch: RotationRad::new(-0.25),
            scale: ScaleFactor::new(0.0),
        }));
        let animating = Rc::new(Cell::new(true));
        // Guards the two-way slider sync against feedback loops.
        let syncing = Rc::new(Cell::new(false));
        let hovered = Rc::new(Cell::new(None::<usize>));
        let selected = Rc::new(Cell::new(None::<usize>));
        // The projected dots of the last frame, shared with the pickers.
        let dots = Rc::new(RefCell::new(Vec::<(Vec2<Pixels>, f64, usize)>::new()));
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
        {
            let camera = Rc::clone(&camera);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let dots = Rc::clone(&dots);
            let nodes = Rc::clone(&nodes);
            canvas.set_draw_func(move |_, context, width, height| {
                let mut cam = camera.get();
                cam.scale = ScaleFactor::new(width.min(height) as f64 / 200.0);
                let viewport = Vec2::<Pixels>::new(width as f64, height as f64);
                // The coordinate axes and the j/g mid-plane ring.
                let extent = JG_MAX as f64 * 6.0;
                let y_extent = ((L_MAX - L_MIN) as f64 / 2.0) * 7.0;
                for (a, b) in [
                    (Vec3::<()>::new(0.0, -y_extent, 0.0), Vec3::new(0.0, y_extent, 0.0)),
                    (Vec3::new(-extent, 0.0, 0.0), Vec3::new(extent, 0.0, 0.0)),
                    (Vec3::new(0.0, 0.0, -extent), Vec3::new(0.0, 0.0, extent)),
                ] {
                    let line = [a, b].iter().map(|point| cam.project(*point, viewport).0).collect::<Vec<_>>();
                    common::stroke_polyline(context, &line, 0.6, &GRID);
                }
                let ring: Vec<Vec2<Pixels>> = (0..72)
                    .map(|step| {
                        let angle = step as f64 / 72.0 * TAU;
                        cam.project(Vec3::<()>::new(extent * angle.cos(), 0.0, extent * angle.sin()), viewport).0
                    })
                    .collect();
                common::stroke_polyline(context, &ring, 0.6, &GRID);
                let mut projected: Vec<(Vec2<Pixels>, f64, usize)> = Vec::with_capacity(nodes.len());
                for (index, node) in nodes.iter().enumerate() {
                    let (p, depth) = cam.project(node.position, viewport);
                    projected.push((p, depth, index));
                }
                projected.sort_by(|a, b| b.1.total_cmp(&a.1));
                for (p, _, index) in &projected {
                    colored_dot(context, p.x, p.y, 3.0, &nodes[*index].color);
                }
                // Selection and hover markers on top.
                for (index, ring) in [(selected.get(), 255u8), (hovered.get(), 0u8)] {
                    if let Some(index) = index {
                        let (p, _) = cam.project(nodes[index].position, viewport);
                        colored_dot(context, p.x, p.y, 6.5, &Color::<u8>::new(ring, ring, ring));
                        colored_dot(context, p.x, p.y, 4.5, &nodes[index].color);
                    }
                }
                *dots.borrow_mut() = projected.iter().map(|(p, d, index)| (*p, *d, *index)).collect();
            });
        }

        /// The index of the lattice node nearest a screen position -
        /// the frontmost dot within `PICK_RADIUS`.
        fn pick(dots: &[(Vec2<Pixels>, f64, usize)], position: Vec2<Pixels>) -> Option<usize> {
            dots.iter()
                .filter(|(p, _, _)| p.distance_f64(&position) < PICK_RADIUS)
                .min_by(|(_, da, _), (_, db, _)| da.total_cmp(db))
                .map(|(_, _, index)| *index)
        }

        // --- detail card -------------------------------------------------

        let detail_osa = Label::new(Some("L -  j -  g -"));
        detail_osa.set_selectable(true);
        let detail_lab = Label::new(Some("L* -  a* -  b* -"));
        detail_lab.set_selectable(true);
        let detail_hex = Label::new(Some("#------"));
        detail_hex.set_selectable(true);
        let detail_rgb = Label::new(Some("RGB ---/---/---"));
        detail_rgb.set_selectable(true);
        let detail_hsv = Label::new(Some("HSV -"));
        detail_hsv.set_selectable(true);
        let detail_pantone = Label::new(Some("Nearest Pantone: -"));
        detail_pantone.set_selectable(true);
        detail_pantone.set_wrap(true);
        detail_pantone.set_max_width_chars(24);

        let preview = DrawingArea::builder().content_height(140).build();
        {
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let nodes = Rc::clone(&nodes);
            preview.set_draw_func(move |_, context, width, height| {
                if let Some(index) = hovered.get().or(selected.get()) {
                    let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                    let _ = target.fill_background(nodes[index].color);
                }
            });
        }

        let detail = Box::new(Orientation::Vertical, 6);
        detail.set_margin_top(12);
        detail.set_margin_bottom(12);
        detail.set_margin_start(12);
        detail.set_margin_end(12);
        detail.set_width_request(220);
        detail.append(&preview);
        detail.append(&detail_osa);
        detail.append(&detail_lab);
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_hsv);
        detail.append(&detail_pantone);

        let update_detail = {
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let detail_osa = detail_osa.clone();
            let detail_lab = detail_lab.clone();
            let detail_hex = detail_hex.clone();
            let detail_rgb = detail_rgb.clone();
            let detail_hsv = detail_hsv.clone();
            let detail_pantone = detail_pantone.clone();
            let preview = preview.clone();
            let nodes = Rc::clone(&nodes);
            move || {
                let Some(index) = hovered.get().or(selected.get()) else {
                    return;
                };
                let node = &nodes[index];
                let lab = Lab::from(node.color);
                let hsv = Hsv::from(node.color);
                detail_osa.set_text(&format!("L {}  j {}  g {}", node.l, node.j, node.g));
                detail_lab.set_text(&format!("L* {:.1}  a* {:.1}  b* {:.1}", lab.l.as_f64(), lab.a, lab.b));
                detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", node.color.r, node.color.g, node.color.b));
                detail_rgb.set_text(&format!("RGB {}/{}/{}", node.color.r, node.color.g, node.color.b));
                detail_hsv.set_text(&format!("HSV {:.0}° {:.0}% {:.0}%", hsv.h.as_f64(), hsv.s.as_f64() * 100.0, hsv.v.as_f64() * 100.0));
                detail_pantone.set_text(&format!("~ {}", PantoneNamedColor::nearest(&node.color).as_str()));
                preview.queue_draw();
            }
        };

        let top_spacer = Box::new(Orientation::Vertical, 0);
        top_spacer.set_vexpand(true);
        let bottom_spacer = Box::new(Orientation::Vertical, 0);
        bottom_spacer.set_vexpand(true);
        let sidebar = Box::new(Orientation::Vertical, 8);
        sidebar.append(&top_spacer);
        sidebar.append(&detail);
        sidebar.append(&bottom_spacer);

        // --- yaw / pitch sliders -------------------------------------------

        // Yaw is the horizontal axis below the rendering, pitch the
        // vertical axis on the left (matching dx/dy of the drag).
        let yaw_slider = Scale::with_range(Orientation::Horizontal, 0.0, 360.0, 0.5);
        yaw_slider.set_hexpand(true);
        let pitch_slider = Scale::with_range(Orientation::Vertical, -90.0, 90.0, 0.5);
        pitch_slider.set_vexpand(true);
        pitch_slider.set_inverted(true);

        let sync_sliders = {
            let syncing = Rc::clone(&syncing);
            let yaw_slider = yaw_slider.clone();
            let pitch_slider = pitch_slider.clone();
            move |cam: Camera| {
                syncing.set(true);
                yaw_slider.set_value(cam.yaw.normalized().as_f64().to_degrees());
                pitch_slider.set_value(cam.pitch.as_f64().to_degrees());
                syncing.set(false);
            }
        };
        {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            let animating = Rc::clone(&animating);
            let syncing = Rc::clone(&syncing);
            yaw_slider.connect_value_changed(move |slider| {
                if syncing.get() {
                    return;
                }
                animating.set(false);
                let mut cam = camera.get();
                cam.yaw = RotationRad::new(slider.value().to_radians());
                camera.set(cam);
                canvas.queue_draw();
            });
        }
        {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            let animating = Rc::clone(&animating);
            let syncing = Rc::clone(&syncing);
            pitch_slider.connect_value_changed(move |slider| {
                if syncing.get() {
                    return;
                }
                animating.set(false);
                let mut cam = camera.get();
                cam.pitch = RotationRad::new(slider.value().to_radians());
                camera.set(cam);
                canvas.queue_draw();
            });
        }

        // --- interaction ---------------------------------------------------

        let drag = GestureDrag::new();
        let drag_base = Rc::new(Cell::new((RotationRad::ZERO, RotationRad::ZERO)));
        {
            let camera = Rc::clone(&camera);
            let animating = Rc::clone(&animating);
            let drag_base = Rc::clone(&drag_base);
            drag.connect_drag_begin(move |_, _, _| {
                animating.set(false);
                let cam = camera.get();
                drag_base.set((cam.yaw, cam.pitch));
            });
        }
        {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            let sync_sliders = sync_sliders.clone();
            drag.connect_drag_update(move |_, dx, dy| {
                let (base_yaw, base_pitch) = drag_base.get();
                let mut cam = camera.get();
                cam.yaw = base_yaw.rotated(RotationRad::new(dx * 0.008));
                cam.pitch = RotationRad::new((base_pitch.as_f64() + dy * 0.008).clamp(-FRAC_PI_2, FRAC_PI_2));
                camera.set(cam);
                sync_sliders(cam);
                canvas.queue_draw();
            });
        }
        canvas.add_controller(drag);

        let motion = EventControllerMotion::new();
        {
            let hovered_motion = Rc::clone(&hovered);
            let canvas_motion = canvas.clone();
            let dots = Rc::clone(&dots);
            let update_motion = update_detail.clone();
            motion.connect_motion(move |_, x, y| {
                let index = pick(&dots.borrow(), Vec2::<Pixels>::new(x, y));
                if index != hovered_motion.get() {
                    hovered_motion.set(index);
                    canvas_motion.queue_draw();
                    update_motion();
                }
            });
            let hovered_leave = Rc::clone(&hovered);
            let canvas_leave = canvas.clone();
            let update_leave = update_detail.clone();
            motion.connect_leave(move |_| {
                hovered_leave.set(None);
                canvas_leave.queue_draw();
                update_leave();
            });
            canvas.add_controller(motion);
        }

        let click = GestureClick::new();
        {
            let selected = Rc::clone(&selected);
            let canvas_click = canvas.clone();
            let dots = Rc::clone(&dots);
            let update_detail = update_detail.clone();
            click.connect_released(move |_, _, x, y| {
                if let Some(index) = pick(&dots.borrow(), Vec2::<Pixels>::new(x, y)) {
                    selected.set(Some(index));
                    canvas_click.queue_draw();
                    update_detail();
                }
            });
            canvas.add_controller(click);
        }

        // Intro: the lattice spins, the pitch swings within +/-30 degrees.
        {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            let animating = Rc::clone(&animating);
            let sync_sliders = sync_sliders.clone();
            glib::timeout_add_local(Duration::from_millis(16), move || {
                if !animating.get() {
                    return glib::ControlFlow::Break;
                }
                let mut cam = camera.get();
                cam.yaw = cam.yaw.rotated(RotationRad::new(INTRO_YAW_STEP));
                cam.pitch = RotationRad::new(INTRO_PITCH_LIMIT * (cam.yaw.as_f64() * 1.5).sin());
                camera.set(cam);
                sync_sliders(cam);
                canvas.queue_draw();
                glib::ControlFlow::Continue
            });
        }

        // A grid aligns the pitch trough with the canvas height and
        // the yaw trough with the canvas width automatically.
        let view = gtk4::Grid::new();
        view.attach(&pitch_slider, 0, 0, 1, 1);
        view.attach(&canvas, 1, 0, 1, 1);
        view.attach(&sidebar, 2, 0, 1, 1);
        view.attach(&yaw_slider, 1, 1, 1, 1);

        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content.append(&view);
        content.append(&info_bar(ABOUT));
        let window = ApplicationWindow::builder()
            .application(app)
            .title("OSA-UCS color lattice (approximate sRGB)")
            .default_width(1000)
            .default_height(880)
            .child(&content)
            .build();
        window.present();
    });
    app.run()
}
