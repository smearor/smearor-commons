//! Shared helpers for the color-space visualization examples:
//! orthographic 3D projection and Cairo drawing helpers on top of
//! commons-geometry types and commons-drawing ops.

#![allow(dead_code)]
#![allow(unused_imports)]

use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Separator;
use gtk4::cairo::Context;
use gtk4::prelude::*;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_color::Hsv;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_drawing::FillCircle;
use smearor_commons_drawing::FillPolygon;
use smearor_commons_drawing::StrokePolyline;
pub use smearor_commons_geometry::Camera;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;
use smearor_commons_geometry::Polyline;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
pub use smearor_commons_geometry::Vec3;

/// An sRGB value for a Cairo `set_source_rgb` call.
pub fn set_color(context: &Context, color: &Color<u8>) {
    context.set_source_rgb(f64::from(color.r) / 255.0, f64::from(color.g) / 255.0, f64::from(color.b) / 255.0);
}

/// A pixel-space position, for the geometry-crate helpers below.
pub fn px(x: f64, y: f64) -> Vec2<Pixels> {
    Vec2::new(x, y)
}

/// A filled circle.
pub fn dot(context: &Context, x: f64, y: f64, radius: f64) {
    context.arc(x, y, radius, 0.0, std::f64::consts::TAU);
    let _ = context.fill();
}

/// A colored filled circle, drawn via the `DrawPrimitiveOps`
/// capability of `CairoTarget`.
pub fn colored_dot(context: &Context, x: f64, y: f64, radius: f64, color: &Color<u8>) {
    let mut target = CairoTarget::new(context, Size::new(0, 0));
    let _ = target.fill_circle(FillCircle::new(Circle::new(px(x, y), radius), *color, Alpha::OPAQUE));
}

/// A stroked polyline through pixel positions, drawn via the
/// `stroke_polyline` op of `CairoTarget`.
pub fn stroke_polyline(context: &Context, points: &[Vec2<Pixels>], width: f32, color: &Color<u8>) {
    let mut target = CairoTarget::new(context, Size::new(0, 0));
    let _ = target.stroke_polyline(StrokePolyline::new(Polyline::new(points.to_vec()), *color, Pixels::new(width), Alpha::OPAQUE));
}

/// A stroked closed `Polygon` outline via the `stroke_polyline` op -
/// the first ring point is repeated to close the path.
pub fn stroke_polygon(context: &Context, polygon: &Polygon<Pixels>, width: f32, color: &Color<u8>) {
    let mut points = polygon.0.clone();
    if let Some(first) = points.first().copied() {
        points.push(first);
    }
    stroke_polyline(context, &points, width, color);
}

/// A colored filled quad (already projected to screen space), drawn
/// via the `fill_polygon` op of `CairoTarget`.
pub fn colored_quad(context: &Context, points: [(f64, f64); 4], color: &Color<u8>) {
    let mut target = CairoTarget::new(context, Size::new(0, 0));
    let polygon = Polygon::new(points.iter().map(|(x, y)| px(*x, *y)).collect());
    let _ = target.fill_polygon(FillPolygon::new(polygon, *color, Alpha::OPAQUE));
}

/// The sRGB value of an HSV color (`h` in degrees).
pub fn hsv_color(h: f64, s: f64, v: f64) -> Color<u8> {
    Color::<u8>::from(Hsv::new(h, s, v))
}

/// Mixes two sRGB colors channel-wise (`t = 0.0` -> `a`,
/// `t = 1.0` -> `b`).
pub fn mix(a: &Color<u8>, b: &Color<u8>, t: f64) -> Color<u8> {
    a.lerp(b, t)
}

/// A wrapped, dimmed explanation text for the bottom edge of a demo
/// window: what the visualization is for and where it came from.
pub fn info_bar(text: &'static str) -> Box {
    let bar = Box::new(Orientation::Vertical, 0);
    bar.append(&Separator::new(Orientation::Horizontal));
    let label = Label::new(Some(text));
    label.set_wrap(true);
    label.set_xalign(0.0);
    label.add_css_class("dim-label");
    label.set_margin_top(8);
    label.set_margin_bottom(10);
    label.set_margin_start(14);
    label.set_margin_end(14);
    bar.append(&label);
    bar
}
