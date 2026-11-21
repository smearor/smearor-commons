//! GTK example: Wilhelm Ostwald's color-order system.
//!
//! Left: the hue circle (`Farbkreis`) with 24 equidistant full colors
//! (`Vollfarben`). Right: the color triangle (`Farbdreieck`) of the
//! selected hue - every Ostwald color is a mixture
//! `v * Vollfarbe + w * Weiss + s * Schwarz` with `v + w + s = 1`,
//! drawn here as the triangular grid of mixture steps.
//!
//! The mixing is the idealized Ostwald model computed in sRGB; the
//! historically printed chips were pigment mixtures.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example ostwald`.

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
use smearor_commons_color::Hue;
use smearor_commons_color::Ostwald;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_drawing::DrawTextOps;
use smearor_commons_geometry::Barycentric;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::f64::consts::TAU;
use std::rc::Rc;

const APP_ID: &str = "io.smearor.commons_color.ostwald";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "Wilhelm Ostwald - chemist and Nobel laureate - published his color system in Die Farbenfibel (1916/17). \
    His idea: every color is a recipe - a share of pure full color plus white plus black, always summing to one. \
    The hue circle (Farbkreis) and the per-hue mixture triangle (Farbdreieck) turn color selection into arithmetic: \
    craftsmen and printers could name and reproduce shades without a sample book. \
    It was one of the first attempts at a systematic, numerical color notation and directly influenced later color-order systems.";

/// The hues of the Ostwald hue circle.
const HUES: usize = 24;
/// Grid subdivisions of the color triangle.
const STEPS: usize = 9;

/// The Ostwald mixture weights `(v, w, s)` of a grid cell:
/// `v` full color, `w` white, `s` black, `v + w + s = 1`.
fn cell_weights(white_step: usize, black_step: usize) -> Barycentric {
    let (w, s) = (white_step as f64 / STEPS as f64, black_step as f64 / STEPS as f64);
    Barycentric::new(1.0 - w - s, w, s)
}

/// The center of the hue circle for a canvas `size`.
fn ring_center(size: Vec2<Pixels>) -> Vec2<Pixels> {
    Vec2::<Pixels>::new(size.x * 0.27, size.y * 0.48)
}

/// The radius of the hue circle for a canvas `size`.
fn ring_radius(size: Vec2<Pixels>) -> f64 {
    size.x.min(size.y) * 0.3
}

/// The center of hue `index` on the circle widget area.
fn ring_point(size: Vec2<Pixels>, index: usize) -> Vec2<Pixels> {
    let center = ring_center(size);
    let (sin, cos) = Degrees::new(index as f64 * 360.0 / HUES as f64 - 90.0).to_radians().sin_cos();
    center + Vec2::<Pixels>::new(ring_radius(size) * cos, ring_radius(size) * sin)
}

/// The triangle vertices `(pure, white, black)` in widget space.
fn triangle(size: Vec2<Pixels>) -> [Vec2<Pixels>; 3] {
    let (origin_x, cy, side) = (size.x * 0.56, size.y * 0.48, size.x * 0.38);
    [
        Vec2::<Pixels>::new(origin_x + side * 0.5, cy - side * 0.45),
        Vec2::<Pixels>::new(origin_x, cy + side * 0.4),
        Vec2::<Pixels>::new(origin_x + side, cy + side * 0.4),
    ]
}

/// The center of triangle cell `(white_step, black_step)`.
fn cell_point(triangle: &[Vec2<Pixels>; 3], white_step: usize, black_step: usize) -> Vec2<Pixels> {
    cell_weights(white_step, black_step).point(triangle)
}

/// What the pointer position hits.
enum Hit {
    /// A hue of the circle.
    Hue(usize),
    /// A cell of the color triangle `(white_step, black_step)`.
    Cell(usize, usize),
    /// Nothing.
    None,
}

fn hit_at(size: Vec2<Pixels>, point: Vec2<Pixels>) -> Hit {
    let chip_r = ring_radius(size) * 0.14;
    for index in 0..HUES {
        if Circle::new(ring_point(size, index), chip_r * 1.25).contains(&point) {
            return Hit::Hue(index);
        }
    }
    let tri = triangle(size);
    if let Some(weights) = Barycentric::from_point(&point, &tri)
        && weights.is_inside()
    {
        let i = (weights.b * STEPS as f64).round() as usize;
        let j = (weights.c * STEPS as f64).round() as usize;
        if i + j <= STEPS {
            return Hit::Cell(i, j);
        }
    }
    Hit::None
}

fn draw(context: &Context, hue: usize, marker: Option<(usize, usize)>, selected: Option<(usize, usize)>, size: Vec2<Pixels>) {
    let center = ring_center(size);
    let radius = ring_radius(size);
    let chip_r = radius * 0.14;
    for index in 0..HUES {
        let p = ring_point(size, index);
        colored_dot(context, p.x, p.y, chip_r, &Ostwald::full_color(Hue::new(index as f64 * 360.0 / HUES as f64)));
        if index == hue {
            context.arc(p.x, p.y, chip_r + 2.5, 0.0, TAU);
            context.set_source_rgb(0.85, 0.85, 0.85);
            context.set_line_width(2.0);
            let _ = context.stroke();
        }
    }
    let mut target = CairoTarget::new(context, Size::new(size.x.max(0.0) as u32, size.y.max(0.0) as u32));
    let _ = target.draw_text(
        None,
        Vec2::<Pixels, f32>::new((center.x - 40.0) as f32, (center.y + radius + 30.0) as f32),
        "Farbkreis - click a hue",
        Pixels::new(13.0),
        Color::<u8>::new(140, 140, 140),
    );

    let hue_angle = Hue::new(hue as f64 * 360.0 / HUES as f64);
    let tri = triangle(size);
    let spacing = (tri[2].x - tri[1].x) / STEPS as f64;
    common::stroke_polygon(context, &Polygon::new(tri.to_vec()), 1.5, &Color::<u8>::new(160, 160, 160));
    for i in 0..=STEPS {
        for j in 0..=STEPS - i {
            let p = cell_point(&tri, i, j);
            let weights = cell_weights(i, j);
            colored_dot(context, p.x, p.y, spacing * 0.4, &Ostwald::new(hue_angle, weights.a, weights.b, weights.c).color());
            for (hit, ring) in [(selected, 0.9), (marker, 0.15)] {
                if hit == Some((i, j)) {
                    context.arc(p.x, p.y, spacing * 0.46, 0.0, TAU);
                    context.set_source_rgb(ring, ring, ring);
                    context.set_line_width(1.5);
                    let _ = context.stroke();
                }
            }
        }
    }
    for (vertex, label, dx, dy) in [
        (tri[1], "Weiss", -10.0, 20.0),
        (tri[0], "Vollfarbe", -14.0, -14.0),
        (tri[2], "Schwarz", -15.0, 20.0),
    ] {
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
        let hue = Rc::new(Cell::new(0usize));
        let hovered = Rc::new(Cell::new(None::<(usize, usize)>));
        let selected = Rc::new(Cell::new(None::<(usize, usize)>));
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
        {
            let hue = Rc::clone(&hue);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            canvas.set_draw_func(move |_, context, width, height| {
                draw(context, hue.get(), hovered.get(), selected.get(), Vec2::<Pixels>::new(width as f64, height as f64));
            });
        }

        // --- detail card -------------------------------------------------

        let detail_recipe = Label::new(Some("v - w - s -"));
        detail_recipe.set_selectable(true);
        let detail_hue = Label::new(Some("Vollfarbe -"));
        detail_hue.set_selectable(true);
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
            let hue = Rc::clone(&hue);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            preview.set_draw_func(move |_, context, width, height| {
                let hue_angle = Hue::new(hue.get() as f64 * 360.0 / HUES as f64);
                let color = hovered
                    .get()
                    .or(selected.get())
                    .map(|(i, j)| Ostwald::new(hue_angle, 1.0 - (i + j) as f64 / STEPS as f64, i as f64 / STEPS as f64, j as f64 / STEPS as f64).color())
                    .unwrap_or_else(|| Ostwald::full(hue_angle).color());
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
        detail.append(&detail_recipe);
        detail.append(&detail_hue);
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_hsv);
        detail.append(&detail_pantone);

        let update_detail = {
            let hue = Rc::clone(&hue);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let detail_recipe = detail_recipe.clone();
            let detail_hue = detail_hue.clone();
            let detail_hex = detail_hex.clone();
            let detail_rgb = detail_rgb.clone();
            let detail_hsv = detail_hsv.clone();
            let detail_pantone = detail_pantone.clone();
            let preview = preview.clone();
            move || {
                let index = hue.get();
                let hue_angle = Hue::new(index as f64 * 360.0 / HUES as f64);
                let full = Ostwald::full(hue_angle).color();
                detail_hue.set_text(&format!("Vollfarbe {index}/24  #{:02X}{:02X}{:02X}", full.r, full.g, full.b));
                let color = hovered
                    .get()
                    .or(selected.get())
                    .map(|(i, j)| {
                        let weights = cell_weights(i, j);
                        detail_recipe.set_text(&format!("v {:.0}%  w {:.0}%  s {:.0}%", weights.a * 100.0, weights.b * 100.0, weights.c * 100.0));
                        Ostwald::new(hue_angle, weights.a, weights.b, weights.c).color()
                    })
                    .unwrap_or_else(|| {
                        detail_recipe.set_text("v 100%  w 0%  s 0%");
                        full
                    });
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
            let hue = Rc::clone(&hue);
            let hovered = Rc::clone(&hovered);
            let selected = Rc::clone(&selected);
            let canvas = canvas.clone();
            let update_detail = update_detail.clone();
            move |x: f64, y: f64, click: bool| {
                let size = Vec2::<Pixels>::new(canvas.width() as f64, canvas.height() as f64);
                let mut changed = false;
                match hit_at(size, Vec2::<Pixels>::new(x, y)) {
                    Hit::Hue(index) => {
                        if click && hue.get() != index {
                            hue.set(index);
                            changed = true;
                        }
                        if hovered.take().is_some() {
                            changed = true;
                        }
                    }
                    Hit::Cell(i, j) => {
                        if hovered.get() != Some((i, j)) {
                            hovered.set(Some((i, j)));
                            changed = true;
                        }
                        if click {
                            selected.set(Some((i, j)));
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
            .title("Ostwald - Farbkreis & Farbdreieck")
            .default_width(1100)
            .default_height(780)
            .child(&content)
            .build();
        update_detail();
        window.present();
    });
    app.run()
}
