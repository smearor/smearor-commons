//! GTK example: a synthetic gamut body - the sRGB color cube sampled
//! on a regular grid and plotted in CIE L\*a\*b\* (D65), rotatable
//! by dragging.
//!
//! This is *not* a real ICC profile gamut; it visualizes where the
//! sRGB primaries land in perceptual space. The eight cube corners
//! (black, the primaries/secondaries, white) are connected by
//! wireframe edges for orientation.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example gamut`.

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
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_color.gamut";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "A gamut is the set of colors a device can actually reproduce - every monitor and printer has a different one. \
    The International Color Consortium (founded 1993) standardized ICC profiles that describe each device's gamut, \
    so images can be converted between devices without losing colors silently. \
    Plotting a gamut inside CIE L*a*b* - a perceptually motivated space - makes the mismatch visible: \
    points of the source outside the target body must be clipped or compressed (gamut mapping). \
    This demo uses the sRGB cube as a stand-in for a real profile, which is exactly the shape an ICC 'monitor' profile describes.";

/// Grid steps per sRGB channel.
const STEPS: usize = 10;

/// Screen radius for picking a gamut point.
const PICK_RADIUS: f64 = 9.0;

/// The faint reference grid on the L* = 50 midplane.
const GRID: Color<u8> = Color::<u8>::new(110, 110, 110);

/// Intro animation: yaw step per tick and the pitch pendulum limit.
const INTRO_YAW_STEP: f64 = 0.006;
const INTRO_PITCH_LIMIT: f64 = 30.0_f64.to_radians();

/// A sample point: position in Lab space and the sRGB value itself.
#[derive(Clone, Copy)]
struct GamutPoint {
    /// The Lab position scaled to world units (L -> Y, a -> X, b -> Z).
    position: Vec3,
    /// The sRGB color of the sample.
    color: Color<u8>,
}

/// Maps a Lab value into the 3D world: `a* -> x`, `b* -> z`,
/// `L* - 50 -> y` (centered).
fn lab_to_vec(lab: Lab) -> Vec3 {
    Vec3::new(lab.a, lab.l.as_f64() - 50.0, lab.b)
}

/// Samples the sRGB cube on a regular `STEPS`^3 grid.
fn gamut_points() -> Vec<GamutPoint> {
    let mut points = Vec::with_capacity((STEPS + 1).pow(3));
    for r in 0..=STEPS {
        for g in 0..=STEPS {
            for b in 0..=STEPS {
                let color = Color::<u8>::new((r * 255 / STEPS) as u8, (g * 255 / STEPS) as u8, (b * 255 / STEPS) as u8);
                points.push(GamutPoint {
                    position: lab_to_vec(Lab::from(color)),
                    color,
                });
            }
        }
    }
    points
}

/// The Lab positions of the eight cube corners, for the wireframe.
fn cube_corners() -> [Vec3; 8] {
    [
        lab_to_vec(Lab::from(Color::<u8>::new(0, 0, 0))),
        lab_to_vec(Lab::from(Color::<u8>::new(255, 0, 0))),
        lab_to_vec(Lab::from(Color::<u8>::new(0, 255, 0))),
        lab_to_vec(Lab::from(Color::<u8>::new(0, 0, 255))),
        lab_to_vec(Lab::from(Color::<u8>::new(255, 255, 0))),
        lab_to_vec(Lab::from(Color::<u8>::new(255, 0, 255))),
        lab_to_vec(Lab::from(Color::<u8>::new(0, 255, 255))),
        lab_to_vec(Lab::from(Color::<u8>::new(255, 255, 255))),
    ]
}

const EDGES: [(usize, usize); 12] = [(0, 1), (0, 2), (0, 4), (1, 4), (1, 5), (2, 4), (2, 6), (3, 5), (3, 6), (4, 7), (5, 7), (6, 7)];

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let points = Rc::new(gamut_points());
        let corners = cube_corners();
        let camera = Rc::new(Cell::new(Camera {
            yaw: RotationRad::new(0.6),
            pitch: RotationRad::new(-0.2),
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
            let points = Rc::clone(&points);
            canvas.set_draw_func(move |_, context, width, height| {
                let mut cam = camera.get();
                cam.scale = ScaleFactor::new(width.min(height) as f64 / 320.0);
                let viewport = Vec2::<Pixels>::new(width as f64, height as f64);
                // The a*/b* reference grid on the L* = 50 midplane and
                // the neutral lightness axis.
                for t in (-128..=128).step_by(32) {
                    let t = t as f64;
                    for (a, b) in [
                        (Vec3::<()>::new(t, 0.0, -128.0), Vec3::new(t, 0.0, 128.0)),
                        (Vec3::new(-128.0, 0.0, t), Vec3::new(128.0, 0.0, t)),
                    ] {
                        let line = [a, b].iter().map(|point| cam.project(*point, viewport).0).collect::<Vec<_>>();
                        common::stroke_polyline(context, &line, 0.6, &GRID);
                    }
                }
                let axis = [Vec3::<()>::new(0.0, -50.0, 0.0), Vec3::new(0.0, 50.0, 0.0)]
                    .iter()
                    .map(|point| cam.project(*point, viewport).0)
                    .collect::<Vec<_>>();
                common::stroke_polyline(context, &axis, 0.6, &GRID);
                let mut projected: Vec<(Vec2<Pixels>, f64, usize)> = Vec::with_capacity(points.len());
                for (index, point) in points.iter().enumerate() {
                    let (p, depth) = cam.project(point.position, viewport);
                    projected.push((p, depth, index));
                }
                projected.sort_by(|a, b| b.1.total_cmp(&a.1));
                for (p, _, index) in &projected {
                    colored_dot(context, p.x, p.y, 2.2, &points[*index].color);
                }
                for (a, b) in EDGES {
                    let edge = [corners[a], corners[b]]
                        .iter()
                        .map(|corner| cam.project(*corner, viewport).0)
                        .collect::<Vec<_>>();
                    common::stroke_polyline(context, &edge, 1.0, &Color::<u8>::new(102, 102, 102));
                }
                // Selection and hover markers on top.
                for (index, ring) in [(selected.get(), 255u8), (hovered.get(), 0u8)] {
                    if let Some(index) = index {
                        let (p, _) = cam.project(points[index].position, viewport);
                        colored_dot(context, p.x, p.y, 6.0, &Color::<u8>::new(ring, ring, ring));
                        colored_dot(context, p.x, p.y, 4.0, &points[index].color);
                    }
                }
                *dots.borrow_mut() = projected.iter().map(|(p, d, index)| (*p, *d, *index)).collect();
            });
        }

        /// The index of the gamut point nearest a screen position -
        /// the frontmost dot within `PICK_RADIUS`.
        fn pick(dots: &[(Vec2<Pixels>, f64, usize)], position: Vec2<Pixels>) -> Option<usize> {
            dots.iter()
                .filter(|(p, _, _)| p.distance_f64(&position) < PICK_RADIUS)
                .min_by(|(_, da, _), (_, db, _)| da.total_cmp(db))
                .map(|(_, _, index)| *index)
        }

        // --- detail card -------------------------------------------------

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
            let points = Rc::clone(&points);
            preview.set_draw_func(move |_, context, width, height| {
                if let Some(index) = hovered.get().or(selected.get()) {
                    let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                    let _ = target.fill_background(points[index].color);
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
        detail.append(&detail_lab);
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_hsv);
        detail.append(&detail_pantone);

        let update_detail = {
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let detail_lab = detail_lab.clone();
            let detail_hex = detail_hex.clone();
            let detail_rgb = detail_rgb.clone();
            let detail_hsv = detail_hsv.clone();
            let detail_pantone = detail_pantone.clone();
            let preview = preview.clone();
            let points = Rc::clone(&points);
            move || {
                let Some(index) = hovered.get().or(selected.get()) else {
                    return;
                };
                let point = &points[index];
                let lab = Lab::from(point.color);
                let hsv = Hsv::from(point.color);
                detail_lab.set_text(&format!("L* {:.1}  a* {:.1}  b* {:.1}", lab.l.as_f64(), lab.a, lab.b));
                detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", point.color.r, point.color.g, point.color.b));
                detail_rgb.set_text(&format!("RGB {}/{}/{}", point.color.r, point.color.g, point.color.b));
                detail_hsv.set_text(&format!("HSV {:.0}° {:.0}% {:.0}%", hsv.h.as_f64(), hsv.s.as_f64() * 100.0, hsv.v.as_f64() * 100.0));
                detail_pantone.set_text(&format!("~ {}", PantoneNamedColor::nearest(&point.color).as_str()));
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

        // Intro: the body spins, the pitch swings within +/-30 degrees.
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
            .title("sRGB gamut in CIE L*a*b* (D65)")
            .default_width(1000)
            .default_height(880)
            .child(&content)
            .build();
        window.present();
    });
    app.run()
}
