//! GTK example: Philipp Otto Runge's color sphere (`Farben-Kugel`,
//! 1810) - the first systematic 3D color-order model.
//!
//! The pure hues sit on the equator; moving poleward mixes each hue
//! with white (north pole) or black (south pole). The sphere is
//! rendered as a latitude/longitude quad mesh in orthographic
//! projection and can be rotated by dragging or with the yaw/pitch
//! sliders. Hovering or clicking a surface point shows its color in
//! the detail card.
//!
//! The surface shading is the idealized model (hue -> white/black
//! mixing in sRGB), following Runge's own construction rule.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example runge`.

mod common;

use common::Camera;
use common::Vec3;
use common::colored_dot;
use common::colored_quad;
use common::hsv_color;
use common::info_bar;
use common::mix;
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
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Sphere;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::f64::consts::FRAC_PI_2;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "io.smearor.commons_color.runge";

/// The faint graticule on the sphere.
const GRID: Color<u8> = Color::<u8>::new(80, 80, 80);

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "Philipp Otto Runge - Romantic painter and self-taught color theorist - published Die Farben-Kugel in 1810, \
    the first systematic three-dimensional color order. He reasoned that every color sits between three poles: \
    the pure hues on the equator, white at the north pole and black at the south pole. \
    The sphere lets a painter read off mixing directions: moving poleward is adding white or black, moving around is changing hue. \
    It was the ancestor of all later color solids - Ostwald's double cone, Munsell's tree and the gamut bodies of colorimetry.";

/// Longitudinal meridians of the sphere mesh.
const MERIDIANS: usize = 36;
/// Latitude bands of the sphere mesh (excluding the poles).
const BANDS: usize = 16;

/// A projected surface quad: screen corners, fill color and mean
/// depth for painter sorting.
type PaintedQuad = ([(f64, f64); 4], Color<u8>, f64);

const WHITE: Color<u8> = Color::<u8>::new(255, 255, 255);
const BLACK: Color<u8> = Color::<u8>::new(0, 0, 0);

/// The unit sphere - Runge's model maps colors onto its surface.
const UNIT_SPHERE: Sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

/// Maximum tilt of the intro animation (degrees).
const INTRO_PITCH_LIMIT: f64 = 30.0;
/// Yaw speed of the intro animation (rad/frame).
const INTRO_YAW_STEP: f64 = 0.012;
/// Intro animation frame interval.
const INTRO_FRAME: Duration = Duration::from_millis(16);

/// A surface point: latitude `lat` (-90..90) and longitude `lon`
/// (degrees) on the unit sphere.
fn sphere_point(lat: f64, lon: f64) -> Vec3 {
    UNIT_SPHERE.point_at(Degrees::new(lat), Degrees::new(lon))
}

/// The Runge surface color: hue at the equator, blended toward the
/// white north pole and the black south pole.
fn surface_color(lat: f64, lon: f64) -> Color<u8> {
    let pure = hsv_color(lon, 1.0, 1.0);
    let t = (lat.abs() / 90.0).min(1.0);
    if lat >= 0.0 { mix(&pure, &WHITE, t) } else { mix(&pure, &BLACK, t) }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let camera = Rc::new(Cell::new(Camera {
            yaw: RotationRad::new(-0.4),
            pitch: RotationRad::new(-0.35),
            scale: ScaleFactor::new(0.0),
        }));
        // Hovered and selected surface positions (lat, lon in degrees).
        let hovered = Rc::new(Cell::new(Option::<(f64, f64)>::None));
        let selected = Rc::new(Cell::new(Option::<(f64, f64)>::None));
        // The intro animation runs until the first user interaction.
        let animating = Rc::new(Cell::new(true));
        // Guards the two-way slider sync against feedback loops.
        let syncing = Rc::new(Cell::new(false));

        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
        {
            let camera = Rc::clone(&camera);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            canvas.set_draw_func(move |_, context, width, height| {
                let mut cam = camera.get();
                cam.scale = ScaleFactor::new((width.min(height) as f64) * 0.42);
                let viewport = Vec2::<Pixels>::new(width as f64, height as f64);
                let mut quads: Vec<PaintedQuad> = Vec::with_capacity(MERIDIANS * BANDS);
                for band in 0..BANDS {
                    let lat0 = -90.0 + band as f64 * (180.0 / BANDS as f64);
                    let lat1 = lat0 + 180.0 / BANDS as f64;
                    for meridian in 0..MERIDIANS {
                        let lon0 = meridian as f64 * (360.0 / MERIDIANS as f64);
                        let lon1 = lon0 + 360.0 / MERIDIANS as f64;
                        let corners = [
                            sphere_point(lat0, lon0),
                            sphere_point(lat0, lon1),
                            sphere_point(lat1, lon1),
                            sphere_point(lat1, lon0),
                        ];
                        let mut points = [(0.0, 0.0); 4];
                        let mut depth = 0.0;
                        for (i, corner) in corners.iter().enumerate() {
                            let (p, z) = cam.project(*corner, viewport);
                            points[i] = (p.x, p.y);
                            depth += z;
                        }
                        quads.push((points, surface_color((lat0 + lat1) / 2.0, (lon0 + lon1) / 2.0), depth));
                    }
                }
                quads.sort_by(|a, b| b.2.total_cmp(&a.2));
                for (points, color, _) in &quads {
                    colored_quad(context, *points, color);
                }
                // The lat/lon graticule, clipped to the visible
                // hemisphere: consecutive visible samples form a run.
                let stroke_visible = |points: &mut dyn Iterator<Item = Vec3>| {
                    let mut run: Vec<Vec2<Pixels>> = Vec::new();
                    for point in points {
                        let (p, depth) = cam.project(point, viewport);
                        if depth < 0.0 {
                            run.push(p);
                        } else {
                            if run.len() > 1 {
                                common::stroke_polyline(context, &run, 0.7, &GRID);
                            }
                            run.clear();
                        }
                    }
                    if run.len() > 1 {
                        common::stroke_polyline(context, &run, 0.7, &GRID);
                    }
                };
                for lon in (0..360).step_by(30) {
                    stroke_visible(&mut (-90..=90).step_by(5).map(|lat| sphere_point(lat as f64, lon as f64)));
                }
                for lat in (-60..=60).step_by(30) {
                    stroke_visible(&mut (0..360).step_by(5).map(|lon| sphere_point(lat as f64, lon as f64)));
                }
                // Markers: white ring for the selection, black for the
                // hovered point, filled with the surface color.
                for (position, ring) in [(selected.get(), 0.93), (hovered.get(), 0.2)] {
                    if let Some((lat, lon)) = position {
                        let (p, depth) = cam.project(sphere_point(lat, lon), viewport);
                        if depth < 0.0 {
                            colored_dot(context, p.x, p.y, 7.0, &Color::<u8>::from(Color::<f64>::new(ring, ring, ring)));
                            colored_dot(context, p.x, p.y, 5.0, &surface_color(lat, lon));
                        }
                    }
                }
            });
        }

        // The (lat, lon) surface position under a screen point, if it
        // is inside the projected sphere disc (front hemisphere).
        let sphere_pick = {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            move |x: f64, y: f64| -> Option<(f64, f64)> {
                let cam = camera.get();
                let (w, h) = (canvas.width() as f64, canvas.height() as f64);
                let scale = (w.min(h)) * 0.42;
                let (vx, vy) = ((x - w / 2.0) / scale, -(y - h / 2.0) / scale);
                let r2 = vx * vx + vy * vy;
                if r2 > 1.0 {
                    return None;
                }
                let world = cam.unview(Vec3::new(vx, vy, (1.0 - r2).sqrt()));
                let (lat, lon) = UNIT_SPHERE.angles_of(&world);
                Some((lat.as_f64(), lon.as_f64()))
            }
        };

        // --- detail card -------------------------------------------------

        let detail_pos = Label::new(Some("Position: -"));
        detail_pos.set_selectable(true);
        let detail_hex = Label::new(Some("#------"));
        detail_hex.set_selectable(true);
        let detail_rgb = Label::new(Some("RGB ---/---/---"));
        detail_rgb.set_selectable(true);
        let detail_hsv = Label::new(Some("HSV -/-/-"));
        detail_hsv.set_selectable(true);
        let detail_pantone = Label::new(Some("Nearest Pantone: -"));
        detail_pantone.set_selectable(true);
        detail_pantone.set_wrap(true);
        detail_pantone.set_max_width_chars(24);

        let preview = DrawingArea::builder().content_height(140).build();
        {
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            preview.set_draw_func(move |_, context, width, height| {
                if let Some((lat, lon)) = hovered.get().or(selected.get()) {
                    let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                    let _ = target.fill_background(surface_color(lat, lon));
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
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_hsv);
        detail.append(&detail_pantone);
        detail.append(&detail_pos);

        let update_detail = {
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let detail_pos = detail_pos.clone();
            let detail_hex = detail_hex.clone();
            let detail_rgb = detail_rgb.clone();
            let detail_hsv = detail_hsv.clone();
            let detail_pantone = detail_pantone.clone();
            let preview = preview.clone();
            move || {
                let Some((lat, lon)) = hovered.get().or(selected.get()) else {
                    return;
                };
                let color = surface_color(lat, lon);
                let hsv = Hsv::from(color);
                detail_pos.set_text(&format!("lat {:+.0}°, lon {:+.0}°", lat, lon));
                detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b));
                detail_rgb.set_text(&format!("RGB {}/{}/{}", color.r, color.g, color.b));
                detail_hsv.set_text(&format!("HSV {:.0}°/{:.0}%/{:.0}%", hsv.h.as_f64(), hsv.s.as_f64() * 100.0, hsv.v.as_f64() * 100.0));
                detail_pantone.set_text(&format!("~ {}", PantoneNamedColor::nearest(&color).as_str()));
                preview.queue_draw();
            }
        };

        // --- yaw / pitch sliders -----------------------------------------

        // Yaw is the horizontal axis below the rendering, pitch the
        // vertical axis on the left (matching dx/dy of the drag).
        let yaw_slider = Scale::with_range(Orientation::Horizontal, 0.0, 360.0, 1.0);
        yaw_slider.set_hexpand(true);
        let pitch_slider = Scale::with_range(Orientation::Vertical, -90.0, 90.0, 1.0);
        pitch_slider.set_vexpand(true);
        pitch_slider.set_inverted(true);

        // Pushes the camera angles into the sliders; `syncing` blocks
        // the change handlers so the push does not read back.
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
            let syncing = Rc::clone(&syncing);
            let animating = Rc::clone(&animating);
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
            let syncing = Rc::clone(&syncing);
            let animating = Rc::clone(&animating);
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
        sync_sliders(camera.get());

        // --- interaction -------------------------------------------------

        let drag = GestureDrag::new();
        let drag_base = Rc::new(Cell::new((RotationRad::ZERO, RotationRad::ZERO)));
        {
            let camera = Rc::clone(&camera);
            let drag_base = Rc::clone(&drag_base);
            let animating = Rc::clone(&animating);
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

        let hover = {
            let hovered = Rc::clone(&hovered);
            let canvas = canvas.clone();
            let update_detail = update_detail.clone();
            move |position: Option<(f64, f64)>| {
                if hovered.get() != position {
                    hovered.set(position);
                    canvas.queue_draw();
                    update_detail();
                }
            }
        };
        {
            let motion = EventControllerMotion::new();
            let sphere_pick = sphere_pick.clone();
            let hover_motion = hover.clone();
            motion.connect_motion(move |_, x, y| hover_motion(sphere_pick(x, y)));
            motion.connect_leave(move |_| hover(None));
            canvas.add_controller(motion);
        }
        {
            let click = GestureClick::new();
            let canvas_click = canvas.clone();
            click.connect_released(move |_, _, x, y| {
                if let Some(position) = sphere_pick(x, y) {
                    selected.set(Some(position));
                    canvas_click.queue_draw();
                    update_detail();
                }
            });
            canvas.add_controller(click);
        }

        // --- intro animation ----------------------------------------------
        {
            let camera = Rc::clone(&camera);
            let canvas = canvas.clone();
            let animating = Rc::clone(&animating);
            let sync_sliders = sync_sliders.clone();
            glib::timeout_add_local(INTRO_FRAME, move || {
                if !animating.get() {
                    return glib::ControlFlow::Break;
                }
                let mut cam = camera.get();
                cam.yaw = cam.yaw.rotated(RotationRad::new(INTRO_YAW_STEP));
                let phase = cam.yaw.normalized().as_f64();
                cam.pitch = RotationRad::new(INTRO_PITCH_LIMIT.to_radians() * (phase * 1.5).sin());
                camera.set(cam);
                sync_sliders(cam);
                canvas.queue_draw();
                glib::ControlFlow::Continue
            });
        }

        // --- layout -------------------------------------------------------

        let top_spacer = Box::new(Orientation::Vertical, 0);
        top_spacer.set_vexpand(true);
        let bottom_spacer = Box::new(Orientation::Vertical, 0);
        bottom_spacer.set_vexpand(true);
        let sidebar = Box::new(Orientation::Vertical, 8);
        sidebar.append(&top_spacer);
        sidebar.append(&detail);
        sidebar.append(&bottom_spacer);

        // A grid aligns the yaw trough with the canvas height and
        // the pitch trough with the canvas width automatically.
        let view = gtk4::Grid::new();
        view.attach(&pitch_slider, 0, 0, 1, 1);
        view.attach(&canvas, 1, 0, 1, 1);
        view.attach(&sidebar, 2, 0, 1, 1);
        view.attach(&yaw_slider, 1, 1, 1, 1);

        let content = Box::new(Orientation::Vertical, 0);
        content.append(&view);
        content.append(&info_bar(ABOUT));
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Runge - Farben-Kugel (1810)")
            .default_width(900)
            .default_height(880)
            .child(&content)
            .build();
        window.present();
    });
    app.run()
}
