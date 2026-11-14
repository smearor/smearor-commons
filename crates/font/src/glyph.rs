//! Glyph-level metrics: scale fitting and bounds-centered placement.

use ab_glyph::Font;
use ab_glyph::Glyph;
use ab_glyph::GlyphId;
use ab_glyph::PxScale;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// The em-scale that makes the glyph's largest pixel dimension equal
/// `target_px`.
///
/// `PxScale::from(x)` sets the **em** size, not the glyph size; icon
/// fonts typically occupy only a small fraction of the em square. The
/// glyph is probed at 100 px em (large enough for integer-`px_bounds`
/// precision), its pixel bounds measured, and the scale solved for
/// `target_px`. `glyph_bounds` (design space, not pixel-accurate) is
/// the fallback when the glyph has no outline.
pub fn glyph_fit_scale<F: Font>(font: &F, glyph_id: GlyphId, target_px: Pixels<f32>) -> Pixels<f32> {
    let probe_scale = Pixels::new(100.0_f32);
    let probe = glyph_id.with_scale(PxScale::from(probe_scale.as_raw()));
    if let Some(outline) = font.outline_glyph(probe) {
        let bounds: Rect<Pixels, f32> = outline.px_bounds().into();
        let max_g = Pixels::new(bounds.size.x.max(bounds.size.y).max(0.0));
        if max_g > Pixels::new(0.01) {
            // max_g is the pixel size at probe_scale em.
            return Pixels::new(target_px.as_raw() * probe_scale.as_raw() / max_g.as_raw());
        }
    }
    let probe = glyph_id.with_scale(1.0);
    let bounds: Rect<Pixels, f32> = font.glyph_bounds(&probe).into();
    let max_g = Pixels::new(bounds.size.x.max(bounds.size.y).max(0.0));
    if max_g > Pixels::new(0.01) {
        Pixels::new(target_px.as_raw() / max_g.as_raw())
    } else {
        target_px
    }
}

/// A glyph scaled to `scale` and positioned so that the center of its
/// pixel bounds lands on `target` — outline `px_bounds` when available
/// (precise for centering), `glyph_bounds` as fallback.
pub fn centered_glyph<F: Font>(font: &F, glyph_id: GlyphId, scale: PxScale, target: Vec2<Pixels, f32>) -> Glyph {
    let probe = glyph_id.with_scale(scale);
    let bounds: Rect<Pixels, f32> = font
        .outline_glyph(probe.clone())
        .map(|outline| outline.px_bounds().into())
        .unwrap_or_else(|| font.glyph_bounds(&probe).into());
    glyph_id.with_scale_and_position(scale, target - bounds.center())
}
