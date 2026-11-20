//! Replays `ab_glyph` outlines as cairo vector paths — the cairo twin of
//! the raster backend's coverage loop.
//!
//! `ab_glyph` keeps the raw curves private inside `OutlinedGlyph`, so
//! the unscaled [`Outline`] comes from [`Font::outline`] and this module
//! applies the same transform the rasterizer uses in
//! `OutlinedGlyph::draw`: font-unit points map to screen pixels as
//! `position + (x·horizontal, −y·vertical)` — the negated vertical
//! factor converts font y-up coordinates into cairo's y-down space.

use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::Glyph;
use ab_glyph::Outline;
use ab_glyph::OutlineCurve;
use ab_glyph::Point;
use ab_glyph::PxScaleFactor;
use ab_glyph::ScaleFont;
use cairo::Context;
use tracing::debug;

/// Outlines `glyph` (id + scale + pen position) into the current path
/// and fills it. Glyphs without outlines (spaces, missing codepoints)
/// are skipped with a `debug!` note — matching the raster backend,
/// which simply renders no coverage for them.
pub(crate) fn fill_glyph(ctx: &Context, font: &FontVec, glyph: &Glyph) -> Result<(), cairo::Error> {
    let Some(outline) = font.outline(glyph.id) else {
        debug!("glyph {:?} has no outline, skipping", glyph.id);
        return Ok(());
    };
    let scale_factor = font.as_scaled(glyph.scale).scale_factor();
    glyph_outline_path(ctx, &outline, scale_factor, glyph.position);
    ctx.fill()
}

/// Appends `outline`'s curves to the current cairo path, scaled by
/// `scale_factor` and shifted to `position` (the glyph pen position, in
/// surface pixels).
///
/// Quadratic Béziers are degree-elevated to cubics (`c1 = p0 + ⅔·(p1−p0)`,
/// `c2 = p2 + ⅔·(p1−p2)`) because cairo has no quadratic segments. Each
/// contour's first point is emitted via `move_to`; a curve whose start
/// does not continue the previous endpoint starts a new contour
/// (multi-contour glyphs like `i` or `ö`). Caller fills (or strokes)
/// the path afterwards.
pub(crate) fn glyph_outline_path(ctx: &Context, outline: &Outline, scale_factor: PxScaleFactor, position: Point) {
    let to_screen = |point: Point| {
        (
            f64::from(position.x + point.x * scale_factor.horizontal),
            f64::from(position.y - point.y * scale_factor.vertical),
        )
    };

    let mut previous_end: Option<Point> = None;
    for curve in &outline.curves {
        let start = match *curve {
            OutlineCurve::Line(p0, _) | OutlineCurve::Quad(p0, _, _) | OutlineCurve::Cubic(p0, _, _, _) => p0,
        };
        if previous_end != Some(start) {
            if previous_end.is_some() {
                ctx.close_path();
            }
            let (x, y) = to_screen(start);
            ctx.move_to(x, y);
        }
        match *curve {
            OutlineCurve::Line(_, p1) => {
                let (x, y) = to_screen(p1);
                ctx.line_to(x, y);
                previous_end = Some(p1);
            }
            OutlineCurve::Quad(p0, p1, p2) => {
                let c1 = Point {
                    x: p0.x + (p1.x - p0.x) * (2.0 / 3.0),
                    y: p0.y + (p1.y - p0.y) * (2.0 / 3.0),
                };
                let c2 = Point {
                    x: p2.x + (p1.x - p2.x) * (2.0 / 3.0),
                    y: p2.y + (p1.y - p2.y) * (2.0 / 3.0),
                };
                let (c1x, c1y) = to_screen(c1);
                let (c2x, c2y) = to_screen(c2);
                let (x, y) = to_screen(p2);
                ctx.curve_to(c1x, c1y, c2x, c2y, x, y);
                previous_end = Some(p2);
            }
            OutlineCurve::Cubic(_, p1, p2, p3) => {
                let (c1x, c1y) = to_screen(p1);
                let (c2x, c2y) = to_screen(p2);
                let (x, y) = to_screen(p3);
                ctx.curve_to(c1x, c1y, c2x, c2y, x, y);
                previous_end = Some(p3);
            }
        }
    }
    ctx.close_path();
}
