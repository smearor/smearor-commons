//! GTK example: the CIE 1931 `xy` chromaticity diagram
//! (`Normfarbtafel`) - the horseshoe of all perceivable colors.
//!
//! The fill raster is rendered once into an offscreen surface: every
//! pixel inside the spectral locus gets the maximum-luminance sRGB
//! color of its chromaticity (colors outside the sRGB gamut are
//! clipped, like in every printed diagram). Overlaid are the
//! spectral locus outline, the sRGB gamut triangle, the D65 white
//! point and wavelength ticks. Moving the pointer shows the
//! chromaticity under the cursor.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example cie_xy`.

mod common;

use common::info_bar;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::DrawingArea;
use gtk4::EventControllerMotion;
use gtk4::GestureClick;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::ToggleButton;
use gtk4::cairo::ImageSurface;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_color::Lab;
use smearor_commons_color::PLANCKIAN_LOCUS;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_color::SPECTRAL_LOCUS;
use smearor_commons_color::Xyz;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_drawing::DrawTextOps;
use smearor_commons_drawing::FillCircle;
use smearor_commons_drawing::StrokePolyline;
use smearor_commons_drawing::image_surface_from_buffer;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;
use smearor_commons_geometry::Polyline;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::Cell;
use std::f64::consts::TAU;
use std::rc::Rc;

const APP_ID: &str = "io.smearor.commons_color.cie_xy";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "In 1931 the CIE turned three imaginary primaries (X, Y, Z) into the first device-independent \
    color space: any color reduces to a point in this chromaticity diagram. \
    The horseshoe boundary is the spectral locus - pure wavelengths; the closing straight edge is the line of purples \
    (no single wavelength produces them). Every real device gamut is a polygon inside the horseshoe, \
    which is why the diagram became the universal map of colorimetry: it answers 'which colors exist at all' \
    before asking what a screen or printer can do with them.";

/// Chromaticity domain of the diagram.
const DOMAIN: Rect = Rect::from_xywh(0.0, 0.0, 0.8, 0.9);
/// Grid resolution of the offscreen fill raster (chromaticity
/// steps).
const GRID: Size<usize> = Size::new(340, 380);
/// Vertex stride used for the inside-polygon test (the locus is
/// smooth enough at this rate).
const LOCUS_STRIDE: usize = 4;

/// A device gamut drawn as a chromaticity polygon.
struct Gamut {
    /// The display name.
    name: &'static str,
    /// The gamut boundary vertices in `xy` (closed polygon).
    vertices: Vec<Vec2>,
    /// The white point of the gamut, if defined.
    white: Option<Vec2>,
    /// The outline color.
    color: Color<f64>,
}

/// The standard illuminant white points drawn by the `D` toggle:
/// `(name, x, y)`.
const D_ILLUMINANTS: [(&str, f64, f64); 5] = [
    ("D50", 0.3457, 0.3585),
    ("D55", 0.3324, 0.3474),
    ("D65", 0.3127, 0.3290),
    ("D75", 0.2990, 0.3149),
    ("D9300", 0.2848, 0.2932),
];

/// The sRGB gamut triangle - the selectable domain (monitor colors).
fn srgb_gamut() -> Polygon<()> {
    Polygon::new(vec![Vec2::new(0.64, 0.33), Vec2::new(0.30, 0.60), Vec2::new(0.15, 0.06)])
}

/// Kelvin labels on the Planckian locus.
const KELVIN_TICKS: [u16; 10] = [1000, 1500, 2000, 2500, 3000, 4000, 5000, 6500, 10000, 20000];

/// Labeled wavelength ticks (nm).
const TICKS: [u16; 11] = [380, 420, 460, 480, 500, 520, 540, 560, 580, 600, 700];

/// `xy` of a FOGRA39 nominal Lab patch (CIE D50 white point).
fn lab_d50_to_xy(l: f64, a: f64, b: f64) -> (f64, f64) {
    let (x, y, _) = Lab::new(l, a, b).to_xyz(Xyz::WHITE_D50).to_xyy();
    (x, y)
}

/// The toggleable device gamuts of the diagram.
///
/// Chromaticities: CIE RGB 1931, Adobe Wide Gamut RGB, Adobe RGB
/// 1998, NTSC 1953, EBU (PAL/SECAM), sRGB. `CMYK` is the FOGRA39
/// characterization target (ISO 12647-2 nominal Lab patches, D50).
fn gamuts() -> Vec<Gamut> {
    let gamut = |name, vertices: &[(f64, f64)], white: Option<(f64, f64)>, color: (f64, f64, f64)| Gamut {
        name,
        vertices: vertices.iter().map(|&(x, y)| Vec2::new(x, y)).collect(),
        white: white.map(|(x, y)| Vec2::new(x, y)),
        color: Color::<f64>::new(color.0, color.1, color.2),
    };
    vec![
        gamut("CIE RGB", &[(0.7347, 0.2653), (0.2738, 0.7174), (0.1665, 0.0089)], Some((0.3333, 0.3333)), (0.45, 0.45, 0.45)),
        gamut(
            "Wide Gamut",
            &[(0.7347, 0.2653), (0.1152, 0.8264), (0.1566, 0.0177)],
            Some((0.3457, 0.3585)),
            (0.0, 0.55, 0.25),
        ),
        gamut("Adobe RGB", &[(0.64, 0.33), (0.21, 0.71), (0.15, 0.06)], Some((0.3127, 0.3290)), (0.15, 0.4, 0.85)),
        gamut("NTSC", &[(0.67, 0.33), (0.21, 0.71), (0.14, 0.08)], Some((0.3101, 0.3162)), (0.75, 0.15, 0.55)),
        gamut("PAL/SECAM", &[(0.64, 0.33), (0.29, 0.60), (0.15, 0.06)], Some((0.3127, 0.3290)), (0.85, 0.5, 0.05)),
        gamut("sRGB", &[(0.64, 0.33), (0.30, 0.60), (0.15, 0.06)], Some((0.3127, 0.3290)), (0.85, 0.1, 0.15)),
        gamut(
            "CMYK",
            &[
                lab_d50_to_xy(89.0, -5.0, 93.0),   // Y
                lab_d50_to_xy(47.0, 65.0, 47.0),   // R
                lab_d50_to_xy(48.0, 75.0, -3.0),   // M
                lab_d50_to_xy(24.0, 22.0, -46.0),  // B
                lab_d50_to_xy(55.0, -37.0, -50.0), // C
                lab_d50_to_xy(50.0, -65.0, 27.0),  // G
            ],
            Some((0.3457, 0.3585)),
            (0.1, 0.55, 0.6),
        ),
    ]
}

/// The spectral-locus polygon in `xy`, closed by the purple line.
fn locus_polygon() -> Polygon<()> {
    Polygon::new(
        SPECTRAL_LOCUS
            .iter()
            .step_by(LOCUS_STRIDE)
            .map(|(_, x, y)| Vec2::new(*x, *y))
            .chain(SPECTRAL_LOCUS.last().map(|(_, x, y)| Vec2::new(*x, *y)))
            .collect(),
    )
}

/// The brightest in-gamut sRGB color for a chromaticity: `xyY` with
/// `Y = 1`, normalized to the largest channel.
fn chromaticity_color(xy: Vec2) -> Color<u8> {
    Color::<u8>::from(Color::<f64>::from(Xyz::from_xyy(xy.x, xy.y, 1.0)).normalized())
}

/// Renders the horseshoe fill into an offscreen surface (once), via a
/// `PixelBuffer` of the buffer crate uploaded through the drawing
/// crate's cairo bridge.
fn fill_surface() -> Option<ImageSurface> {
    let polygon = locus_polygon();
    let mut buffer = PixelBuffer::<Rgba>::new(Size::new(GRID.x as u32, GRID.y as u32));
    let cell_w = DOMAIN.size.x / GRID.x as f64;
    let cell_h = DOMAIN.size.y / GRID.y as f64;
    for gy in 0..GRID.y {
        for gx in 0..GRID.x {
            let xy = Vec2::new((gx as f64 + 0.5) * cell_w, DOMAIN.size.y - (gy as f64 + 0.5) * cell_h);
            if !polygon.contains(&xy) {
                continue;
            }
            let color = chromaticity_color(xy);
            let offset = (gy * GRID.x + gx) * 4;
            buffer.pixels_mut()[offset..offset + 4].copy_from_slice(&[color.r, color.g, color.b, 255]);
        }
    }
    image_surface_from_buffer(&buffer).ok()
}

/// The diagram frame margin inside the canvas.
const MARGIN: f64 = 40.0;

/// Maps a chromaticity to widget coordinates (y flipped).
fn map_xy(point: Vec2, viewport: Vec2<Pixels>) -> Vec2<Pixels> {
    let margin = MARGIN;
    let scale = ((viewport.x - 2.0 * margin) / DOMAIN.size.x).min((viewport.y - 2.0 * margin) / DOMAIN.size.y);
    Vec2::<Pixels>::new(margin + point.x * scale, viewport.y - margin - point.y * scale)
}

/// Black text with a white drop shadow, readable on any fill.
fn shadow_text(target: &mut CairoTarget, position: Vec2<Pixels>, text: &str, font_size: f32) {
    for (dx, dy) in [(-1.5f32, 0.0), (1.5, 0.0), (0.0, -1.5), (0.0, 1.5), (1.5, 1.5)] {
        let _ = target.draw_text(
            None,
            Vec2::<Pixels, f32>::new(position.x as f32 + dx, position.y as f32 + dy),
            text,
            Pixels::new(font_size),
            Color::<u8>::new(255, 255, 255),
        );
    }
    let _ = target.draw_text(
        None,
        Vec2::<Pixels, f32>::new(position.x as f32, position.y as f32),
        text,
        Pixels::new(font_size),
        Color::<u8>::new(0, 0, 0),
    );
}

/// Maps widget coordinates back to a chromaticity.
fn unmap_xy(point: Vec2<Pixels>, viewport: Vec2<Pixels>) -> Vec2 {
    let margin = MARGIN;
    let scale = ((viewport.x - 2.0 * margin) / DOMAIN.size.x).min((viewport.y - 2.0 * margin) / DOMAIN.size.y);
    Vec2::new((point.x - margin) / scale, (viewport.y - margin - point.y) / scale)
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let surface = fill_surface();
        let gamuts = gamuts();
        let hover = Rc::new(Cell::new(None::<Vec2>));
        let selected = Rc::new(Cell::new(None::<Vec2>));
        // Guards the two-way slider sync against feedback loops.
        let syncing = Rc::new(Cell::new(false));
        let locus = locus_polygon();
        let srgb_gamut = srgb_gamut();
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
        let buttons: Rc<Vec<ToggleButton>> = Rc::new(
            [
                "CIE RGB",
                "Wide Gamut",
                "Adobe RGB",
                "NTSC",
                "PAL/SECAM",
                "sRGB",
                "CMYK",
                "Kelvin",
                "D illuminants",
            ]
            .iter()
            .map(|label| ToggleButton::with_label(label))
            .collect(),
        );
        for (index, button) in buttons.iter().enumerate() {
            // sRGB, Kelvin and the illuminant points start enabled.
            button.set_active(matches!(index, 5 | 7 | 8));
            let canvas_clone = canvas.clone();
            button.connect_toggled(move |_| canvas_clone.queue_draw());
        }
        {
            let hover = Rc::clone(&hover);
            let selected = Rc::clone(&selected);
            let buttons = Rc::clone(&buttons);
            canvas.set_draw_func(move |_, context, width, height| {
                let (w, h) = (width as f64, height as f64);
                let viewport = Vec2::<Pixels>::new(w, h);
                let (x0, y0) = {
                    let p = map_xy(Vec2::new(0.0, DOMAIN.size.y), viewport);
                    (p.x, p.y)
                };
                let (x1, y1) = {
                    let p = map_xy(Vec2::new(DOMAIN.size.x, 0.0), viewport);
                    (p.x, p.y)
                };
                if let Some(surface) = &surface {
                    let _ = context.save();
                    context.translate(x0, y0);
                    context.scale((x1 - x0) / GRID.x as f64, (y1 - y0) / GRID.y as f64);
                    let _ = context.set_source_surface(surface, 0.0, 0.0);
                    let _ = context.paint();
                    let _ = context.restore();
                }

                let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));

                // Axis frame, a light 0.1-step grid and tick labels.
                {
                    let corner = |x, y| map_xy(Vec2::new(x, y), viewport);
                    let frame = Polyline::new(vec![
                        corner(0.0, 0.0),
                        corner(DOMAIN.size.x, 0.0),
                        corner(DOMAIN.size.x, DOMAIN.size.y),
                        corner(0.0, DOMAIN.size.y),
                        corner(0.0, 0.0),
                    ]);
                    let gray = Color::<f64>::new(0.45, 0.45, 0.45);
                    let _ = target.stroke_polyline(StrokePolyline::new(frame, gray, Pixels::new(1.0), Alpha::OPAQUE));
                    let grid = Color::<f64>::new(1.0, 1.0, 1.0);
                    for i in 1..9 {
                        if (i as f64 * 0.1) < DOMAIN.size.x {
                            let line = Polyline::new(vec![corner(i as f64 * 0.1, 0.0), corner(i as f64 * 0.1, DOMAIN.size.y)]);
                            let _ = target.stroke_polyline(StrokePolyline::new(line, grid, Pixels::new(0.6), Alpha::new(0.4)));
                        }
                        if (i as f64 * 0.1) < DOMAIN.size.y {
                            let line = Polyline::new(vec![corner(0.0, i as f64 * 0.1), corner(DOMAIN.size.x, i as f64 * 0.1)]);
                            let _ = target.stroke_polyline(StrokePolyline::new(line, grid, Pixels::new(0.6), Alpha::new(0.4)));
                        }
                    }
                    for i in 1..9 {
                        let value = i as f64 * 0.1;
                        if value < DOMAIN.size.x {
                            let p = corner(value, 0.0);
                            shadow_text(&mut target, Vec2::<Pixels>::new(p.x - 8.0, p.y + 14.0), &format!("{value:.1}"), 11.0);
                        }
                        if value < DOMAIN.size.y {
                            let p = corner(0.0, value);
                            shadow_text(&mut target, Vec2::<Pixels>::new(p.x - 26.0, p.y + 3.0), &format!("{value:.1}"), 11.0);
                        }
                    }
                }

                let mut locus: Vec<Vec2<Pixels>> = SPECTRAL_LOCUS.iter().map(|(_, x, y)| map_xy(Vec2::new(*x, *y), viewport)).collect();
                locus.push(locus[0]);
                let _ = target.stroke_polyline(StrokePolyline::new(Polyline::new(locus), Color::<u8>::new(64, 64, 64), Pixels::new(1.2), Alpha::OPAQUE));

                // The Planckian locus and its Kelvin labels.
                if buttons[7].is_active() {
                    let curve: Vec<Vec2<Pixels>> = PLANCKIAN_LOCUS.iter().map(|(_, x, y)| map_xy(Vec2::new(*x, *y), viewport)).collect();
                    let _ = target.stroke_polyline(StrokePolyline::new(Polyline::new(curve), Color::<u8>::new(38, 38, 38), Pixels::new(1.4), Alpha::OPAQUE));
                    for kelvin in KELVIN_TICKS {
                        if let Some((_, x, y)) = PLANCKIAN_LOCUS.iter().find(|(k, _, _)| *k == kelvin) {
                            let t = map_xy(Vec2::new(*x, *y), viewport);
                            let _ = target.fill_circle(FillCircle::new(Circle::new(t, 2.2), Color::<f64>::new(0.15, 0.15, 0.15), Alpha::OPAQUE));
                            shadow_text(&mut target, Vec2::<Pixels>::new(t.x - 10.0, t.y + 14.0), &format!("{kelvin}K"), 11.0);
                        }
                    }
                }

                // The standard illuminant white points.
                if buttons[8].is_active() {
                    for (name, x, y) in D_ILLUMINANTS {
                        let t = map_xy(Vec2::new(x, y), viewport);
                        let _ = target.fill_circle(FillCircle::new(Circle::new(t, 3.0), Color::<f64>::new(1.0, 1.0, 1.0), Alpha::OPAQUE));
                        context.set_source_rgb(0.1, 0.1, 0.1);
                        context.arc(t.x, t.y, 3.0, 0.0, TAU);
                        let _ = context.stroke();
                        shadow_text(&mut target, Vec2::<Pixels>::new(t.x + 6.0, t.y - 5.0), name, 12.0);
                    }
                }

                // The device gamut polygons.
                let mut labels: Vec<(Vec2<Pixels>, &'static str)> = Vec::new();
                for (index, gamut) in gamuts.iter().enumerate() {
                    if !buttons[index].is_active() {
                        continue;
                    }
                    let color = gamut.color;
                    let mut vertices: Vec<Vec2<Pixels>> = gamut.vertices.iter().map(|v| map_xy(*v, viewport)).collect();
                    vertices.push(vertices[0]);
                    let _ = target.stroke_polyline(StrokePolyline::new(Polyline::new(vertices.clone()), color, Pixels::new(1.6), Alpha::OPAQUE));
                    // The label anchors at the midpoint of the highest
                    // chromaticity edge, so it sits on the boundary it
                    // names instead of floating in the centroid.
                    let ring = &gamut.vertices;
                    let top_edge = (0..ring.len())
                        .max_by(|&a, &b| ((ring[a].y + ring[(a + 1) % ring.len()].y) / 2.0).total_cmp(&((ring[b].y + ring[(b + 1) % ring.len()].y) / 2.0)));
                    let top_edge = top_edge.unwrap_or(0);
                    let (a, b) = (ring[top_edge], ring[(top_edge + 1) % ring.len()]);
                    let mid = map_xy(Vec2::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0), viewport);
                    labels.push((Vec2::<Pixels>::new(mid.x - 14.0, mid.y - 8.0), gamut.name));
                    if let Some(white) = gamut.white {
                        let _ = target.fill_circle(FillCircle::new(Circle::new(map_xy(white, viewport), 2.4), color, Alpha::OPAQUE));
                    }
                }
                // Stagger labels that would overlap (sRGB and
                // PAL/SECAM share nearly the same triangle).
                labels.sort_by(|a, b| a.0.y.total_cmp(&b.0.y));
                let mut last_y = f64::MIN;
                for (pos, _) in labels.iter_mut() {
                    if pos.y - last_y < 14.0 {
                        pos.y = last_y + 14.0;
                    }
                    last_y = pos.y;
                }
                for (pos, name) in labels {
                    shadow_text(&mut target, pos, name, 12.0);
                }

                let tick_color = Color::<f64>::new(0.3, 0.3, 0.3);
                for nm in TICKS {
                    if let Some((_, x, y)) = SPECTRAL_LOCUS.iter().find(|(w, _, _)| *w == nm) {
                        let t = map_xy(Vec2::new(*x, *y), viewport);
                        let _ = target.fill_circle(FillCircle::new(Circle::new(t, 2.0), tick_color, Alpha::OPAQUE));
                        shadow_text(&mut target, Vec2::<Pixels>::new(t.x + 5.0, t.y - 4.0), &nm.to_string(), 12.0);
                    }
                }

                if let Some(xy) = selected.get() {
                    let t = map_xy(xy, viewport);
                    let _ = target.fill_circle(FillCircle::new(Circle::new(t, 6.5), Color::<f64>::new(1.0, 1.0, 1.0), Alpha::OPAQUE));
                    let inner = if locus_polygon().contains(&xy) {
                        chromaticity_color(xy)
                    } else {
                        // Outside the spectral locus there is no real
                        // color - the marker stays hollow gray.
                        Color::<u8>::new(60, 60, 60)
                    };
                    let _ = target.fill_circle(FillCircle::new(Circle::new(t, 4.5), inner, Alpha::OPAQUE));
                }
                if let Some(xy) = hover.get() {
                    let _ = target.draw_text(
                        None,
                        Vec2::<Pixels, f32>::new(16.0, 24.0),
                        &format!("x = {:.4}   y = {:.4}", xy.x, xy.y),
                        Pixels::new(13.0),
                        Color::<u8>::new(38, 38, 38),
                    );
                }
            });
        }
        {
            let motion = EventControllerMotion::new();
            let hover = Rc::clone(&hover);
            let canvas_clone = canvas.clone();
            motion.connect_motion(move |_, x, y| {
                let viewport = Vec2::<Pixels>::new(canvas_clone.width() as f64, canvas_clone.height() as f64);
                let c = unmap_xy(Vec2::<Pixels>::new(x, y), viewport);
                let next = DOMAIN.contains(&c).then_some(c);
                if hover.get() != next {
                    hover.set(next);
                    canvas_clone.queue_draw();
                }
            });
            canvas.add_controller(motion);
        }

        // --- detail card -------------------------------------------------

        let detail_xy = Label::new(Some("x = -, y = -"));
        detail_xy.set_selectable(true);
        let detail_hex = Label::new(Some("#------"));
        detail_hex.set_selectable(true);
        let detail_rgb = Label::new(Some("RGB ---/---/---"));
        detail_rgb.set_selectable(true);
        let detail_pantone = Label::new(Some("Nearest Pantone: -"));
        detail_pantone.set_selectable(true);
        detail_pantone.set_wrap(true);
        detail_pantone.set_max_width_chars(24);
        let detail_gamut = Label::new(Some(""));
        detail_gamut.set_selectable(true);
        let detail_gamut_wrap = detail_gamut.clone();

        let preview = DrawingArea::builder().content_height(140).build();
        {
            let hover = Rc::clone(&hover);
            let selected = Rc::clone(&selected);
            preview.set_draw_func(move |_, context, width, height| {
                if let Some(xy) = hover.get().or(selected.get()) {
                    let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                    let color = if locus_polygon().contains(&xy) {
                        chromaticity_color(xy)
                    } else {
                        Color::<u8>::new(40, 40, 40)
                    };
                    let _ = target.fill_background(color);
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
        detail.append(&detail_xy);
        detail.append(&detail_hex);
        detail.append(&detail_rgb);
        detail.append(&detail_pantone);
        detail.append(&detail_gamut);

        let update_detail = {
            let hover = Rc::clone(&hover);
            let selected = Rc::clone(&selected);
            let srgb_gamut = srgb_gamut.clone();
            let locus = locus.clone();
            let preview = preview.clone();
            move || {
                let Some(xy) = hover.get().or(selected.get()) else {
                    return;
                };
                detail_xy.set_text(&format!("x = {:.4}, y = {:.4}", xy.x, xy.y));
                if locus.contains(&xy) {
                    let color = chromaticity_color(xy);
                    detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b));
                    detail_rgb.set_text(&format!("RGB {}/{}/{}", color.r, color.g, color.b));
                    detail_pantone.set_text(&format!("~ {}", PantoneNamedColor::nearest(&color).as_str()));
                    detail_gamut_wrap.set_text(if srgb_gamut.contains(&xy) { "in sRGB gamut" } else { "outside sRGB gamut" });
                } else {
                    detail_hex.set_text("-");
                    detail_rgb.set_text("-");
                    detail_pantone.set_text("-");
                    detail_gamut_wrap.set_text("no real color");
                }
                preview.queue_draw();
            }
        };

        // --- x / y sliders -------------------------------------------------

        let x_slider = Scale::with_range(Orientation::Horizontal, 0.0, DOMAIN.size.x, 0.005);
        x_slider.set_hexpand(true);
        let y_slider = Scale::with_range(Orientation::Vertical, 0.0, DOMAIN.size.y, 0.005);
        y_slider.set_vexpand(true);
        y_slider.set_inverted(true);

        let sync_sliders = {
            let syncing = Rc::clone(&syncing);
            let x_slider = x_slider.clone();
            let y_slider = y_slider.clone();
            move |xy: Vec2| {
                syncing.set(true);
                x_slider.set_value(xy.x);
                y_slider.set_value(xy.y);
                syncing.set(false);
            }
        };
        {
            let selected = Rc::clone(&selected);
            let canvas = canvas.clone();
            let syncing = Rc::clone(&syncing);
            let update_detail = update_detail.clone();
            x_slider.connect_value_changed(move |slider| {
                if syncing.get() {
                    return;
                }
                let xy = selected.get().unwrap_or(Vec2::new(0.0, 0.0));
                let xy = Vec2::new(slider.value(), xy.y);
                selected.set(Some(xy));
                canvas.queue_draw();
                update_detail();
            });
        }
        {
            let selected = Rc::clone(&selected);
            let canvas = canvas.clone();
            let syncing = Rc::clone(&syncing);
            let update_detail = update_detail.clone();
            y_slider.connect_value_changed(move |slider| {
                if syncing.get() {
                    return;
                }
                let xy = selected.get().unwrap_or(Vec2::new(0.0, 0.0));
                let xy = Vec2::new(xy.x, slider.value());
                selected.set(Some(xy));
                canvas.queue_draw();
                update_detail();
            });
        }

        // --- selection ------------------------------------------------------

        let select = {
            let selected = Rc::clone(&selected);
            let canvas = canvas.clone();
            let sync_sliders = sync_sliders.clone();
            let update_detail = update_detail.clone();
            move |xy: Vec2| {
                selected.set(Some(xy));
                sync_sliders(xy);
                canvas.queue_draw();
                update_detail();
            }
        };
        {
            let click = GestureClick::new();
            let canvas_click = canvas.clone();
            let select = select.clone();
            click.connect_released(move |_, _, x, y| {
                let viewport = Vec2::<Pixels>::new(canvas_click.width() as f64, canvas_click.height() as f64);
                let c = unmap_xy(Vec2::<Pixels>::new(x, y), viewport);
                if DOMAIN.contains(&c) && locus.contains(&c) {
                    select(c);
                }
            });
            canvas.add_controller(click);
        }
        {
            let leave = EventControllerMotion::new();
            let canvas_leave = canvas.clone();
            leave.connect_leave(move |_| {
                if hover.get().is_some() {
                    hover.set(None);
                    canvas_leave.queue_draw();
                    update_detail();
                }
            });
            canvas.add_controller(leave);
        }

        let toolbar = Box::new(Orientation::Horizontal, 6);
        toolbar.set_margin_top(8);
        toolbar.set_margin_start(8);
        toolbar.set_margin_end(8);
        for button in buttons.iter() {
            toolbar.append(button);
        }
        let top_spacer = Box::new(Orientation::Vertical, 0);
        top_spacer.set_vexpand(true);
        let bottom_spacer = Box::new(Orientation::Vertical, 0);
        bottom_spacer.set_vexpand(true);
        let sidebar = Box::new(Orientation::Vertical, 8);
        sidebar.append(&top_spacer);
        sidebar.append(&detail);
        sidebar.append(&bottom_spacer);

        let view = Box::new(Orientation::Horizontal, 0);
        view.append(&y_slider);
        view.append(&canvas);
        view.append(&sidebar);

        let sliders = Box::new(Orientation::Horizontal, 12);
        sliders.set_margin_top(6);
        sliders.set_margin_bottom(6);
        sliders.append(&x_slider);

        let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content.append(&toolbar);
        content.append(&view);
        content.append(&sliders);
        content.append(&info_bar(ABOUT));
        let window = ApplicationWindow::builder()
            .application(app)
            .title("CIE 1931 xy chromaticity diagram")
            .default_width(900)
            .default_height(880)
            .child(&content)
            .build();

        // Align the slider troughs with the plot rect: the diagram is
        // letterboxed inside the canvas, so the margins track its
        // actual pixel bounds on every resize.
        {
            let x_slider = x_slider.clone();
            let y_slider = y_slider.clone();
            let sidebar = sidebar.clone();
            canvas.connect_resize(move |_, width, height| {
                let (w, h) = (width as f64, height as f64);
                let scale = ((w - 2.0 * MARGIN) / DOMAIN.size.x).min((h - 2.0 * MARGIN) / DOMAIN.size.y);
                let (plot_w, plot_h) = (DOMAIN.size.x * scale, DOMAIN.size.y * scale);
                y_slider.set_margin_top((h - MARGIN - plot_h).max(0.0) as i32);
                y_slider.set_margin_bottom(MARGIN as i32);
                x_slider.set_margin_start(y_slider.width() + MARGIN as i32);
                x_slider.set_margin_end((sidebar.width() as f64 + (w - MARGIN - plot_w)).max(0.0) as i32);
            });
        }
        window.present();
    });
    app.run()
}
