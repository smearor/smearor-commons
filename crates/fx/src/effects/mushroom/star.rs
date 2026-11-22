use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;

use super::scale::zero_start_end;

/// BMW `getStar` - a five-point star SDF-ish mask: `uv` relative
/// pixel position, `center` the star center, `size` the radius, all
/// in the same (aspect-corrected) space. Returns 0/1 coverage via
/// the cross-product edge test.
fn get_star(uv: Vec2<()>, center: Vec2<()>, npoints: f64, radius_ratio: f64, size: f64, rotation: f64) -> f64 {
    let starangle = std::f64::consts::TAU / npoints;
    // One point always up at rotation = 0.
    let rotation = rotation + std::f64::consts::FRAC_PI_2 - starangle;
    let p0 = Vec2::<()>::new(size * rotation.cos(), size * rotation.sin());
    let p1 = Vec2::<()>::new(size * radius_ratio * (starangle + rotation).cos(), size * radius_ratio * (starangle + rotation).sin());
    let cur = Vec2::<()>::new(uv.x - center.x, uv.y - center.y);
    let cur_radius = cur.x.mul_add(cur.x, cur.y * cur.y).sqrt();
    let cur_angle = cur.y.atan2(cur.x) - rotation;
    let mut a = crate::fract(cur_angle / starangle);
    if a >= 0.5 {
        a = 1.0 - a;
    }
    let a = a * starangle + rotation;
    let cur_pos = Vec2::<()>::new(cur_radius * a.cos(), cur_radius * a.sin());
    // Cross product of (p1 - p0) x (curPos - p0).
    let dir0 = Vec2::<()>::new(p1.x - p0.x, p1.y - p0.y);
    let dir1 = Vec2::<()>::new(cur_pos.x - p0.x, cur_pos.y - p0.y);
    if dir0.x * dir1.y - dir0.y * dir1.x >= 0.0 { 1.0 } else { 0.0 }
}

/// BMW `getStarColor` - interpolates `v` over the six-stop ramp
/// (`STEPS`), scaling each stop's alpha by `alpha`.
fn star_color(colors: &[Color<f64>; 6], v: f64, alpha: f64) -> Color<f64> {
    const STEPS: [f64; 6] = [0.0, 0.1666, 0.3332, 0.4998, 0.6664, 0.8330];
    let at = |i: usize| colors[i];
    let v = v.clamp(0.0, 1.0);
    if v >= STEPS[5] {
        let c = at(5);
        return Color::<f64>::new_rgba(c.r, c.g, c.b, alpha * c.a);
    }
    for i in 0..5 {
        if v <= STEPS[i + 1] {
            let t = (v - STEPS[i]) / (STEPS[i + 1] - STEPS[i]);
            let (a, b) = (at(i), at(i + 1));
            return Color::<f64>::new_rgba(crate::mix(a.r, b.r, t), crate::mix(a.g, b.g, t), crate::mix(a.b, b.b, t), alpha * crate::mix(a.a, b.a, t));
        }
    }
    Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0)
}

/// BMW `getStars` - `ring_count` rings of `star_count` stars
/// circling the window. Ring `r` lags by `spread = r/ring_count`;
/// each star orbits `sin/cos(rotation*τ + s/star_count*τ)` on the
/// `aspect*0.33` ellipse. `depth < 0` puts a star behind the window
/// (composited under, scaled by `1 - oColorAlpha`).
#[allow(clippy::too_many_arguments)]
pub(super) fn stars(
    star_uv: Vec2<()>,
    aspect: f64,
    progress: f64,
    window_alpha: f64,
    ring_count: u32,
    ring_rotation: f64,
    star_count: u32,
    colors: &[Color<f64>; 6],
) -> Color<f64> {
    let mut result = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    for r in 0..ring_count {
        let spread = r as f64 / ring_count.max(1) as f64;
        let y = crate::mix(0.0 - spread, 1.0 + (1.0 - spread), 1.0 - progress).clamp(0.00001, 0.99999);
        for s in 0..star_count {
            let phase = progress * ring_rotation * std::f64::consts::TAU + s as f64 * (std::f64::consts::TAU / star_count.max(1) as f64);
            let center = Vec2::<()>::new(phase.sin() * aspect * 0.33, y);
            let mask = get_star(star_uv, center, 5.0, 0.5, zero_start_end(y, 0.1, 2.0), 0.0).clamp(0.0, 1.0);
            let star = star_color(colors, y, mask);
            // Behind vs in front of the window.
            if phase.cos() < 0.0 {
                result = crate::alpha_over(result, star);
            } else {
                result = crate::alpha_over(
                    Color::<f64>::new_rgba(
                        star.r * (1.0 - window_alpha),
                        star.g * (1.0 - window_alpha),
                        star.b * (1.0 - window_alpha),
                        star.a * (1.0 - window_alpha),
                    ),
                    result,
                );
            }
        }
    }
    result
}
