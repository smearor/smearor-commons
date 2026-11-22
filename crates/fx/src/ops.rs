//! `ops` - the `common.glsl` helper family effects actually use.
//!
//! Easing is deliberately absent: `animation::EasingFunction` covers
//! `easeOutQuad`/`easeInSine`/etc., and `fract`'s noise-specific form
//! lives in `smearor-commons-noise` - these are the remaining
//! mask/blend primitives.

use smearor_commons_color::Color;
use smearor_commons_color::Hsv;
use smearor_commons_color::Hue;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;

/// GLSL `fract(x) = x - x.floor()` - exact for negatives.
pub fn fract(x: f64) -> f64 {
    x - x.floor()
}

/// GLSL `mix(a, b, t)` - linear blend (`t` unclamped).
pub fn mix(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// GLSL `smoothstep(edge0, edge1, x)` - hermite ramp `t²(3-2t)`
/// clamped to `[0, 1]`.
pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// BMW `remap` - `x` from `[old_min, old_max]` into `[new_min,
/// new_max]`, result clamped to the new range.
pub fn remap(x: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    let t = ((x - old_min) / (old_max - old_min)).clamp(0.0, 1.0);
    new_min + (new_max - new_min) * t
}

/// BMW `getAbsoluteEdgeMask` - the context-aware edge fade: fades
/// `uv` (pixel space) inside the layer rect shrunk by the effective
/// padding `max(0, padding - fade * offset)`; fullscreen content is
/// never faded.
///
/// `offset` positions the fade zone relative to the window border:
/// `0` fully inside, `0.5` centered on the border (the value every
/// shipped effect uses), `1` fully outside.
///
/// Delegates to [`Rect::edge_fade`] for the pure rect math - this
/// wrapper only knows where the window sits inside the padded layer.
pub fn absolute_edge_mask(uv: Vec2<()>, size: Vec2<()>, padding: Pixels<f64>, fade: Pixels<f64>, offset: f64, fullscreen: bool) -> f64 {
    if fullscreen {
        return 1.0;
    }
    let p = (padding.0 - fade.0 * offset).max(0.0);
    Rect::new(Vec2::<()>::new(p, p), Vec2::<()>::new(size.x - 2.0 * p, size.y - 2.0 * p)).edge_fade(uv, fade.0)
}

/// BMW `offsetHue` - rotates the color's hue by `offset` circle
/// fractions (`0.5` = complementary), saturation and alpha preserved.
/// `offset` is a `[0, 1)` fraction, not degrees - matching BMW's
/// `hueOffset` uniform.
pub fn offset_hue(color: Color<f64>, offset: f64) -> Color<f64> {
    let mut hsv = Hsv::from(color);
    hsv.h = Hue::new(hsv.h.as_f64() + offset * 360.0);
    Color::<f64>::from(hsv)
}

/// BMW `whirl` - swirl distortion: rotates `coords` around the
/// origin by an angle which falls off quadratically with distance
/// (`warping` scales the falloff, `rotation` adds a constant spin).
/// The geeks3d swirl-post-processing filter.
pub fn whirl(coords: Vec2<()>, warping: f64, rotation: f64) -> Vec2<()> {
    // (1-len)^2 stays positive outside the radius - powi(2), not max(0).
    let angle = (1.0 - coords.x.mul_add(coords.x, coords.y * coords.y).sqrt()).powi(2) * warping + rotation;
    let (sin, cos) = angle.sin_cos();
    Vec2::<()>::new(coords.x * cos - coords.y * sin, coords.x * sin + coords.y * cos)
}

/// BMW `getRelativeEdgeMask` - fades `uv` (unit space) to zero at the
/// actor boundaries over `fade` relative units; pure uv math, no
/// pixel context.
pub fn relative_edge_mask(uv: Vec2<()>, fade: f64) -> f64 {
    Rect::new(Vec2::<()>::new(0.0, 0.0), Vec2::<()>::new(1.0, 1.0)).edge_fade(uv, fade)
}

/// BMW `darken` - scales color toward black by `fac`.
pub fn darken(color: Color<f64>, fac: f64) -> Color<f64> {
    let t = 1.0 - fac;
    Color::<f64>::new_rgba(color.r * t, color.g * t, color.b * t, color.a)
}

/// BMW `lighten` - lerps color toward white by `fac`.
pub fn lighten(color: Color<f64>, fac: f64) -> Color<f64> {
    Color::<f64>::new_rgba(color.r + (1.0 - color.r) * fac, color.g + (1.0 - color.g) * fac, color.b + (1.0 - color.b) * fac, color.a)
}

/// BMW `tritone` - maps `val` through a three-stop gradient:
/// shadows → midtones over the lower half, midtones → highlights
/// over the upper half, each leg smoothed. Used by `incinerate`
/// and `wisps`.
pub fn tritone(val: f64, shadows: Color<f64>, midtones: Color<f64>, highlights: Color<f64>) -> Color<f64> {
    let mix_rgb = |a: Color<f64>, b: Color<f64>, t: f64| Color::<f64>::new(mix(a.r, b.r, t), mix(a.g, b.g, t), mix(a.b, b.b, t));
    if val < 0.5 {
        mix_rgb(shadows, midtones, smoothstep(0.0, 1.0, val * 2.0))
    } else {
        mix_rgb(midtones, highlights, smoothstep(0.0, 1.0, val * 2.0 - 1.0))
    }
}

/// BMW `alphaOver` - source-over compositing: `src` over `dst`,
/// straight-alpha semantics.
pub fn alpha_over(dst: Color<f64>, src: Color<f64>) -> Color<f64> {
    let a = src.a + dst.a * (1.0 - src.a);
    if a <= 0.0 {
        return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    }
    let over = |s: f64, d: f64| (s * src.a + d * dst.a * (1.0 - src.a)) / a;
    Color::<f64>::new_rgba(over(src.r, dst.r), over(src.g, dst.g), over(src.b, dst.b), a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remap_matches_the_glsl_formula() {
        assert_eq!(remap(0.5, 0.0, 1.0, 0.0, 10.0), 5.0);
        assert_eq!(remap(2.0, 0.0, 1.0, 0.0, 10.0), 10.0, "clamps above");
        assert_eq!(remap(-1.0, 0.0, 1.0, 0.0, 10.0), 0.0, "clamps below");
        assert_eq!(remap(0.5, 1.0, 0.0, 0.0, 10.0), 5.0, "inverted range");
    }

    #[test]
    fn smoothstep_matches_glsl_on_a_grid() {
        for i in -10..=20 {
            let x = i as f64 / 10.0;
            let t = x.clamp(0.0, 1.0);
            let expected = t * t * (3.0 - 2.0 * t);
            assert!((smoothstep(0.0, 1.0, x) - expected).abs() < 1e-12);
        }
    }

    #[test]
    fn absolute_edge_mask_respects_padding_and_fullscreen() {
        let size = Vec2::<()>::new(100.0, 100.0);
        // Inside the padded rect -> 1.0; at the outer edge -> faded.
        assert_eq!(absolute_edge_mask(Vec2::<()>::new(50.0, 50.0), size, Pixels::new(10.0), Pixels::new(5.0), 0.5, false), 1.0);
        assert!(absolute_edge_mask(Vec2::<()>::new(0.0, 50.0), size, Pixels::new(10.0), Pixels::new(5.0), 0.5, false) < 0.5);
        assert_eq!(absolute_edge_mask(Vec2::<()>::new(0.0, 0.0), size, Pixels::new(10.0), Pixels::new(5.0), 0.5, true), 1.0);
        // A bigger offset pushes the fade zone outward: at uv 8 the
        // offset-1.0 mask (fade starts at padding 5) is well inside
        // while offset-0.0 (fade starts at padding 10) is still 0.
        let outward = absolute_edge_mask(Vec2::<()>::new(8.0, 50.0), size, Pixels::new(10.0), Pixels::new(5.0), 1.0, false);
        let inward = absolute_edge_mask(Vec2::<()>::new(8.0, 50.0), size, Pixels::new(10.0), Pixels::new(5.0), 0.0, false);
        assert!(outward > inward, "offset 1.0 keeps more mask inside the window");
    }
}
