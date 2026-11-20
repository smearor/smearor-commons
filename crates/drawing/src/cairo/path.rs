//! Generic cairo path building blocks harvested from consumer code.
//!
//! These helpers append segments to the current path of a [`Context`];
//! the caller decides whether to fill or stroke the result. They are
//! cairo-only because they emit cairo path commands — the geometry they
//! trace (polyline, spline, wave, band, rounded rectangle) is
//! backend-neutral, but the raster backend has no consumer for it.
//!
//! - [`polyline_path`]: straight segments through the points (qdo36
//!   `px_polyline_path` lineage — road and rail edges).
//! - [`smooth_path`]: Catmull-Rom→cubic-Bézier spline through the
//!   points (qdo36 `smooth_path` — watercourse curves).
//! - [`wavy_path`]: arc-length-sampled sine wave along the polyline
//!   (qdo36 `wavy_path` — boat wakes; `phase` lets shared edges agree).
//! - [`band_path`]: closed polygon between two bank lines (qdo36
//!   `band_path` — river bodies).
//! - [`rounded_rect_path`]: the canonical four-arc rounded rectangle
//!   (osk-rs `draw_rounded_rect` lineage) — cairo has no built-in one.
//! - [`tile_fill`]: repeat a tile surface over a rect (qdo36
//!   `tile_fill` — ground textures panning with the map).

use cairo::Context;
use cairo::ImageSurface;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// Straight segments through `points`, starting a new sub-path.
pub fn polyline_path(ctx: &Context, points: &[Vec2<Pixels>]) {
    ctx.new_path();
    for (index, point) in points.iter().enumerate() {
        if index == 0 {
            ctx.move_to(point.x, point.y);
        } else {
            ctx.line_to(point.x, point.y);
        }
    }
}

/// A smooth spline through `points` (Catmull-Rom → cubic Bézier).
///
/// The first and last hop draw straight: endpoints are the short stubs
/// that must not pull spline tangents (qdo36 pier stubs), so segment
/// tangents clamp to the second and second-to-last control points.
pub fn smooth_path(ctx: &Context, points: &[Vec2<Pixels>]) {
    ctx.new_path();
    let Some(first) = points.first() else { return };
    ctx.move_to(first.x, first.y);
    if points.len() < 4 {
        for point in &points[1..] {
            ctx.line_to(point.x, point.y);
        }
        return;
    }
    ctx.line_to(points[1].x, points[1].y);
    let last = points.len() - 2;
    let at = |index: isize| points[index.clamp(1, last as isize) as usize];
    for index in 1..last {
        let (p0, p1, p2, p3) = (at(index as isize - 1), at(index as isize), at(index as isize + 1), at(index as isize + 2));
        ctx.curve_to(
            p1.x + (p2.x - p0.x) / 6.0,
            p1.y + (p2.y - p0.y) / 6.0,
            p2.x - (p3.x - p1.x) / 6.0,
            p2.y - (p3.y - p1.y) / 6.0,
            p2.x,
            p2.y,
        );
    }
    ctx.line_to(points[last + 1].x, points[last + 1].y);
}

/// A sine wave traced along `points`: the polyline is sampled at fine
/// arc-length steps and each sample shifts perpendicular to the local
/// segment direction. `phase` offsets the wave's arc position, so
/// polylines sharing a stretch trace the same wave (qdo36 `wavy_path`).
pub fn wavy_path(ctx: &Context, points: &[Vec2<Pixels>], wavelength: Pixels<f64>, amplitude: Pixels<f64>, phase: Pixels<f64>) {
    let (wavelength, amplitude, phase) = (wavelength.as_f64(), amplitude.as_f64(), phase.as_f64());
    ctx.new_path();
    let Some(first) = points.first() else { return };
    ctx.move_to(first.x, first.y);
    let step = (wavelength / 16.0).max(0.15);
    let mut sample_arc = step;
    let mut seg_start_arc = 0.0;
    for pair in points.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let seg_len = dx.hypot(dy);
        if seg_len < 1e-6 {
            continue;
        }
        let (ux, uy) = (dx / seg_len, dy / seg_len);
        while sample_arc < seg_start_arc + seg_len {
            let t = sample_arc - seg_start_arc;
            let wave = amplitude * ((sample_arc + phase) / wavelength * std::f64::consts::TAU).sin();
            ctx.line_to(a.x + ux * t - uy * wave, a.y + uy * t + ux * wave);
            sample_arc += step;
        }
        seg_start_arc += seg_len;
    }
    if let Some(last) = points.last() {
        ctx.line_to(last.x, last.y);
    }
}

/// The closed band between two bank lines: `left_bank` traced forward,
/// `right_bank` reversed — the filled body between them (qdo36
/// `band_path` — river bodies).
pub fn band_path(ctx: &Context, left_bank: &[Vec2<Pixels>], right_bank: &[Vec2<Pixels>]) {
    ctx.new_path();
    let Some(first) = left_bank.first() else { return };
    ctx.move_to(first.x, first.y);
    for point in &left_bank[1..] {
        ctx.line_to(point.x, point.y);
    }
    for point in right_bank.iter().rev() {
        ctx.line_to(point.x, point.y);
    }
    ctx.close_path();
}

/// Rounds `rect`'s corners into the current path with four 90° arcs —
/// the canonical rounded-rectangle path (osk-rs lineage). `radius`
/// clamps to half the shorter side so degenerate rects stay valid.
pub fn rounded_rect_path(ctx: &Context, rect: Rect<Pixels>, radius: Pixels<f64>) {
    let (x, y) = (rect.min.x, rect.min.y);
    let (width, height) = (rect.width(), rect.height());
    let r = radius.as_f64().min(width / 2.0).min(height / 2.0).max(0.0);
    ctx.new_path();
    ctx.arc(x + width - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    ctx.arc(x + width - r, y + height - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    ctx.arc(x + r, y + height - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    ctx.arc(x + r, y + r, r, std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2);
    ctx.close_path();
}

/// Fills `(0, 0, width, height)` with `tile` repeated, anchored so the
/// tile origin sits at `pin` — the texture repeats infinitely and pans
/// with the anchor point (qdo36 `tile_fill` — map ground textures).
pub fn tile_fill(ctx: &Context, tile: &ImageSurface, pin: Vec2<Pixels>, width: f64, height: f64) -> Result<(), cairo::Error> {
    let pattern = cairo::SurfacePattern::create(tile);
    pattern.set_extend(cairo::Extend::Repeat);
    pattern.set_matrix(cairo::Matrix::new(1.0, 0.0, 0.0, 1.0, -pin.x, -pin.y));
    ctx.set_source(&pattern)?;
    ctx.rectangle(0.0, 0.0, width, height);
    ctx.fill()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo::Format;
    use smearor_commons_geometry::Vec2;

    fn px(x: f64, y: f64) -> Vec2<Pixels> {
        Vec2::new(x, y)
    }

    fn surface_and_ctx() -> (ImageSurface, Context) {
        let surface = ImageSurface::create(Format::ARgb32, 32, 32).unwrap();
        let ctx = Context::new(&surface).unwrap();
        (surface, ctx)
    }

    #[test]
    fn polyline_path_traces_points() {
        let (_surface, ctx) = surface_and_ctx();
        polyline_path(&ctx, &[px(1.0, 1.0), px(10.0, 1.0), px(10.0, 10.0)]);
        assert_eq!(ctx.current_point().unwrap(), (10.0, 10.0));
    }

    #[test]
    fn polyline_path_empty_is_noop() {
        let (_surface, ctx) = surface_and_ctx();
        polyline_path(&ctx, &[]);
        // No current point: cairo reports the origin, nothing drawn.
        assert_eq!(ctx.current_point().unwrap(), (0.0, 0.0));
    }

    #[test]
    fn smooth_path_ends_at_last_point() {
        let (_surface, ctx) = surface_and_ctx();
        let points = [px(0.0, 16.0), px(4.0, 10.0), px(12.0, 22.0), px(20.0, 12.0), px(28.0, 18.0)];
        smooth_path(&ctx, &points);
        let (x, y) = ctx.current_point().unwrap();
        assert!((x - 28.0).abs() < 1e-9 && (y - 18.0).abs() < 1e-9);
    }

    #[test]
    fn smooth_path_degenerates_to_polyline_below_four_points() {
        let (_surface, ctx) = surface_and_ctx();
        smooth_path(&ctx, &[px(1.0, 1.0), px(9.0, 9.0), px(20.0, 5.0)]);
        let (x, y) = ctx.current_point().unwrap();
        assert!((x - 20.0).abs() < 1e-9 && (y - 5.0).abs() < 1e-9);
    }

    #[test]
    fn wavy_path_reaches_last_point() {
        let (_surface, ctx) = surface_and_ctx();
        wavy_path(&ctx, &[px(0.0, 16.0), px(30.0, 16.0)], Pixels::new(8.0), Pixels::new(2.0), Pixels::new(0.0));
        let (x, y) = ctx.current_point().unwrap();
        assert!((x - 30.0).abs() < 1e-9 && (y - 16.0).abs() < 1e-9);
    }

    #[test]
    fn band_path_closes_between_banks() {
        let (_surface, ctx) = surface_and_ctx();
        band_path(&ctx, &[px(0.0, 0.0), px(10.0, 0.0)], &[px(0.0, 5.0), px(10.0, 5.0)]);
        ctx.fill().unwrap();
        // The band filled the area between the banks.
        assert!(ctx.status().is_ok());
    }

    #[test]
    fn rounded_rect_path_stays_inside_rect() {
        let (surface, ctx) = surface_and_ctx();
        ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        rounded_rect_path(&ctx, Rect::from_xywh(4.0, 4.0, 20.0, 20.0), Pixels::new(6.0));
        ctx.fill().unwrap();
        let stride = surface.stride() as usize;
        drop(ctx);
        let mut surface = surface;
        let data = surface.data().unwrap();
        // Corners stay transparent — the arcs cut them.
        let corner = stride * 4 + 4 * 4;
        assert_eq!(data[corner + 3], 0, "corner alpha should be zero");
        // Center is filled.
        let center = stride * 12 + 12 * 4;
        assert_eq!(data[center + 3], 255, "center alpha should be 255");
    }

    #[test]
    fn tile_fill_repeats_tile() {
        let (surface, ctx) = surface_and_ctx();
        let tile = ImageSurface::create(Format::ARgb32, 2, 2).unwrap();
        {
            let tile_ctx = Context::new(&tile).unwrap();
            tile_ctx.set_source_rgba(1.0, 0.0, 0.0, 1.0);
            tile_ctx.paint().unwrap();
        }
        tile_fill(&ctx, &tile, px(0.0, 0.0), 32.0, 32.0).unwrap();
        drop(ctx);
        let mut surface = surface;
        let data = surface.data().unwrap();
        assert_eq!(data[2], 255, "tile color should repeat across the fill");
    }
}
