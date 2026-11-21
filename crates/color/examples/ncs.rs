//! GTK example: the Natural Colour System (NCS) - hue circle and
//! color triangle.
//!
//! Left: the NCS hue circle with the four elementary chromatic
//! colors (`Y`ellow, `R`ed, `B`lue, `G`reen) and the 100-step
//! resemblance between them (`Y90R` = yellow that resembles red to
//! 90%). Click a hue to select it. Right: the NCS color triangle of
//! that hue: every color is described by blackness `s`,
//! chromaticness `c` and whiteness `w` with `s + c + w = 100`,
//! giving the familiar `S sccc-HUE` notation (e.g. `S 2030-Y90R`).
//!
//! The colors are a *documented approximation*: the official NCS
//! atlas values are proprietary, so hue resemblance is mapped onto
//! CIELAB hue angles between the four elementary anchors and
//! chromaticness/blackness onto `C*`/`L*` ranges. The geometry and
//! the notation are the real NCS ones.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example ncs`.

mod common;

use common::colored_dot;
use common::info_bar;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::DrawingArea;
use gtk4::EventControllerMotion;
use gtk4::GestureClick;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::cairo::Context;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_color::Color;
use smearor_commons_color::Hsv;
use smearor_commons_color::Ncs;
use smearor_commons_color::NcsHue;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_drawing::DrawTextOps;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Barycentric;
use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::f64::consts::TAU;
use std::rc::Rc;

const APP_ID: &str = "io.smearor.commons_color.ncs";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "The Natural Colour System was developed in Sweden (Anders Hard, Lars Sivik et al., 1970s) \
    from a different premise than colorimetry: not what color is made of, but what it looks like. \
    Everyone can judge how much a shade resembles the six elementary colors (white, black, yellow, red, blue, green) - \
    and resemblance is enough to name a color: S 2030-Y90R means 20% blackness, 30% chromaticness, \
    a hue of yellow that resembles red to 90%. \
    Standardized by the Swedish Standards Institute (1979) and later an international standard, \
    it became the working notation of architecture and product design - and a model for later \
    appearance-based systems like CIECAM.";

/// Subdivisions of the color triangle (10% steps).
const STEPS: usize = 10;

/// The mixture weights `(c, w, s)` of a grid cell on the
/// `(chroma, white, black)` triangle, `c + w + s = 1`.
fn cell_weights(blackness: usize, chromaticness: usize) -> Barycentric {
    Barycentric::new(
        chromaticness as f64 / STEPS as f64,
        (STEPS - blackness - chromaticness) as f64 / STEPS as f64,
        blackness as f64 / STEPS as f64,
    )
}

/// The center of the hue circle for a canvas `size`.
fn ring_center(size: Vec2<Pixels>) -> Vec2<Pixels> {
    Vec2::<Pixels>::new(size.x * 0.27, size.y * 0.45)
}

/// The radius of the hue circle for a canvas `size`.
fn ring_radius(size: Vec2<Pixels>) -> f64 {
    size.x.min(size.y) * 0.3
}

/// The point of circle `position` on the ring (`position * 0.9 - 90`
/// degrees, so `Y` sits on top).
fn ring_point(size: Vec2<Pixels>, position: f64) -> Vec2<Pixels> {
    let center = ring_center(size);
    let (sin, cos) = Degrees::new(position * 0.9 - 90.0).to_radians().sin_cos();
    center + Vec2::<Pixels>::new(ring_radius(size) * cos, ring_radius(size) * sin)
}

/// Triangle vertices `(chroma top, white left, black right)`.
fn triangle(size: Vec2<Pixels>) -> [Vec2<Pixels>; 3] {
    let (origin_x, cy, side) = (size.x * 0.58, size.y * 0.45, size.x * 0.36);
    [
        Vec2::<Pixels>::new(origin_x + side * 0.5, cy - side * 0.5),
        Vec2::<Pixels>::new(origin_x, cy + side * 0.45),
        Vec2::<Pixels>::new(origin_x + side, cy + side * 0.45),
    ]
}

/// Cell `(blackness_step, chromaticness_step)` center inside the
/// triangle.
fn cell_point(triangle: &[Vec2<Pixels>; 3], s: usize, c: usize) -> Vec2<Pixels> {
    cell_weights(s, c).point(triangle)
}

enum Hit {
    /// A circle position.
    Hue(f64),
    /// A triangle cell `(blackness, chromaticness)`.
    Cell(usize, usize),
    /// Nothing.
    None,
}

fn hit_at(size: Vec2<Pixels>, point: Vec2<Pixels>) -> Hit {
    let radius = ring_radius(size);
    let annulus = Annulus::new(ring_center(size), radius * 0.6, radius * 1.3);
    if annulus.contains(&point) {
        let direction = (point - ring_center(size)).with_unit::<()>().direction();
        // `ring_point` draws at `position * 0.9 - 90` degrees, so the
        // inverse adds the quarter turn back.
        let position = (direction + RotationRad::QUARTER_TURN).normalized().as_f64() / TAU * 400.0;
        return Hit::Hue(position);
    }
    let tri = triangle(size);
    if let Some(weights) = Barycentric::from_point(&point, &tri)
        && weights.is_inside()
    {
        let c = (weights.a * STEPS as f64).round() as usize;
        let s = (weights.c * STEPS as f64).round() as usize;
        if s + c <= STEPS {
            return Hit::Cell(s, c);
        }
    }
    Hit::None
}

fn draw(context: &Context, position: f64, marker: Option<(usize, usize)>, selected: Option<(usize, usize)>, size: Vec2<Pixels>) {
    let center = ring_center(size);
    let radius = ring_radius(size);
    let chip_r = radius * 0.12;
    // The hue ring sampled at 100 circle positions.
    for step in 0..100 {
        let step_position = step as f64 * 4.0;
        let p = ring_point(size, step_position);
        colored_dot(context, p.x, p.y, chip_r, &NcsHue::new(step_position).color(0.0, 100.0));
        if (step_position - position).abs() < 2.0 {
            context.arc(p.x, p.y, chip_r + 3.0, 0.0, TAU);
            context.set_source_rgb(0.85, 0.85, 0.85);
            context.set_line_width(2.0);
            let _ = context.stroke();
        }
    }
    // Elementary hue labels.
    let mut target = CairoTarget::new(context, Size::new(size.x.max(0.0) as u32, size.y.max(0.0) as u32));
    for (pos, label) in [(0.0, "Y"), (100.0, "R"), (200.0, "B"), (300.0, "G")] {
        let (sin, cos) = Degrees::new(pos * 0.9 - 90.0).to_radians().sin_cos();
        let p = center + Vec2::<Pixels>::new(radius * 1.28 * cos, radius * 1.28 * sin);
        let _ = target.draw_text(
            None,
            Vec2::<Pixels, f32>::new((p.x - 4.0) as f32, (p.y + 5.0) as f32),
            label,
            Pixels::new(14.0),
            Color::<u8>::new(140, 140, 140),
        );
    }

    let hue = NcsHue::new(position);
    let tri = triangle(size);
    let spacing = (tri[2].x - tri[1].x) / STEPS as f64;
    common::stroke_polygon(context, &Polygon::new(tri.to_vec()), 1.5, &Color::<u8>::new(160, 160, 160));
    for s in 0..=STEPS {
        for c in 0..=STEPS - s {
            let p = cell_point(&tri, s, c);
            colored_dot(context, p.x, p.y, spacing * 0.42, &hue.color(s as f64 * 10.0, c as f64 * 10.0));
            for (hit, ring) in [(selected, 0.9), (marker, 0.15)] {
                if hit == Some((s, c)) {
                    context.arc(p.x, p.y, spacing * 0.5, 0.0, TAU);
                    context.set_source_rgb(ring, ring, ring);
                    context.set_line_width(1.5);
                    let _ = context.stroke();
                }
            }
        }
    }
    for (vertex, label, dx, dy) in [(tri[1], "W", -14.0, 20.0), (tri[0], "C", -8.0, -12.0), (tri[2], "S", -10.0, 20.0)] {
        let _ = target.draw_text(
            None,
            Vec2::<Pixels, f32>::new((vertex.x + dx) as f32, (vertex.y + dy) as f32),
            label,
            Pixels::new(13.0),
            Color::<u8>::new(140, 140, 140),
        );
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let position = Rc::new(Cell::new(0.0f64));
        let hovered = Rc::new(Cell::new(None::<(usize, usize)>));
        let selected = Rc::new(Cell::new(None::<(usize, usize)>));
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
        {
            let position = Rc::clone(&position);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            canvas.set_draw_func(move |_, context, width, height| {
                draw(context, position.get(), hovered.get(), selected.get(), Vec2::<Pixels>::new(width as f64, height as f64));
            });
        }

        // --- detail card -------------------------------------------------

        let detail_notation = Label::new(Some("S ----Y"));
        detail_notation.set_selectable(true);
        let detail_recipe = Label::new(Some("s - c - w -"));
        detail_recipe.set_selectable(true);
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
            let position = Rc::clone(&position);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            preview.set_draw_func(move |_, context, width, height| {
                let hue = NcsHue::new(position.get());
                let color = hovered
                    .get()
                    .or(selected.get())
                    .map(|(s, c)| hue.color(s as f64 * 10.0, c as f64 * 10.0))
                    .unwrap_or_else(|| hue.color(0.0, 100.0));
                let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                let _ = target.fill_background(color);
            });
        }

        let detail = Box::new(Orientation::Vertical, 6);
        detail.set_margin_top(12);
        detail.set_margin_bottom(12);
        detail.set_margin_start(12);
        detail.set_margin_end(12);
        detail.set_width_request(220);
        detail.append(&preview);
        detail.append(&detail_notation);
        detail.append(&detail_recipe);
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_hsv);
        detail.append(&detail_pantone);

        let update_detail = {
            let position = Rc::clone(&position);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let detail_notation = detail_notation.clone();
            let detail_recipe = detail_recipe.clone();
            let detail_hex = detail_hex.clone();
            let detail_rgb = detail_rgb.clone();
            let detail_hsv = detail_hsv.clone();
            let detail_pantone = detail_pantone.clone();
            let preview = preview.clone();
            move || {
                let hue = NcsHue::new(position.get());
                let (color, weights) = match hovered.get().or(selected.get()) {
                    Some((s, c)) => (hue.color(s as f64 * 10.0, c as f64 * 10.0), Some(cell_weights(s, c))),
                    None => (hue.color(0.0, 100.0), None),
                };
                let (s, c) = match weights {
                    Some(w) => (w.c * 100.0, w.a * 100.0),
                    None => (0.0, 100.0),
                };
                let ncs = Ncs::new(hue, s, c);
                detail_notation.set_text(&ncs.to_string());
                detail_recipe.set_text(&format!("s {}%  c {}%  w {}%", s, c, ncs.whiteness()));
                let hsv = Hsv::from(color);
                detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b));
                detail_rgb.set_text(&format!("RGB {}/{}/{}", color.r, color.g, color.b));
                detail_hsv.set_text(&format!("HSV {:.0}° {:.0}% {:.0}%", hsv.h.as_f64(), hsv.s.as_f64() * 100.0, hsv.v.as_f64() * 100.0));
                detail_pantone.set_text(&format!("~ {}", PantoneNamedColor::nearest(&color).as_str()));
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

        // --- interaction ---------------------------------------------------

        let pick = {
            let position = Rc::clone(&position);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let canvas = canvas.clone();
            let update_detail = update_detail.clone();
            move |x: f64, y: f64, click: bool| {
                let size = Vec2::<Pixels>::new(canvas.width() as f64, canvas.height() as f64);
                let mut changed = false;
                match hit_at(size, Vec2::<Pixels>::new(x, y)) {
                    Hit::Hue(pos) => {
                        if click && (position.get() - pos).abs() > 1.0 {
                            position.set(pos);
                            changed = true;
                        }
                        if hovered.take().is_some() {
                            changed = true;
                        }
                    }
                    Hit::Cell(s, c) => {
                        if hovered.get() != Some((s, c)) {
                            hovered.set(Some((s, c)));
                            changed = true;
                        }
                        if click {
                            selected.set(Some((s, c)));
                            changed = true;
                        }
                    }
                    Hit::None => {
                        if hovered.take().is_some() {
                            changed = true;
                        }
                    }
                }
                if changed {
                    canvas.queue_draw();
                    update_detail();
                }
            }
        };
        let motion = EventControllerMotion::new();
        let pick_motion = pick.clone();
        motion.connect_motion(move |_, x, y| pick_motion(x, y, false));
        {
            let hovered = Rc::clone(&hovered);
            let canvas = canvas.clone();
            let update_detail = update_detail.clone();
            motion.connect_leave(move |_| {
                hovered.set(None);
                canvas.queue_draw();
                update_detail();
            });
        }
        canvas.add_controller(motion);
        let click = GestureClick::new();
        click.connect_released(move |_, _, x, y| pick(x, y, true));
        canvas.add_controller(click);

        let view = Box::new(Orientation::Horizontal, 0);
        view.append(&canvas);
        view.append(&sidebar);

        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content.append(&view);
        content.append(&info_bar(ABOUT));
        let window = ApplicationWindow::builder()
            .application(app)
            .title("NCS - Natural Colour System")
            .default_width(1100)
            .default_height(780)
            .child(&content)
            .build();
        update_detail();
        window.present();
    });
    app.run()
}
