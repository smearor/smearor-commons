use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;

/// The Portal easter egg - "The cake is a lie!". `CAKE_PX` fits in
/// `[-0.8, 0.8]` of the cake's own units.
pub(crate) const CAKE_PX: f64 = 128.0;
pub(crate) const CAKE_MIN_SCALE: f64 = 3.0;
/// `CAKE_CHANCE` from `AperturePanels.js` - per-animation probability.
pub(crate) const CAKE_CHANCE: f64 = 0.05;

/// `sdRoundBox` - signed distance to a rounded box centred on the
/// origin (negative inside).
fn sd_round_box(p: Vec2<()>, b: Vec2<()>, r: f64) -> f64 {
    let d = Vec2::<()>::new(p.x.abs() - b.x + r, p.y.abs() - b.y + r);
    let mx = d.x.max(0.0);
    let my = d.y.max(0.0);
    (mx * mx + my * my).sqrt() + d.x.max(d.y).min(0.0) - r
}

/// `drawCake` - the cake in straight alpha. `p` is centred on the
/// cake (y up), `aa` is one pixel in cake units, `t` is seconds.
pub(crate) fn draw_cake(p: Vec2<()>, aa: f64, t: f64) -> Color<f64> {
    let mut col = (0.0, 0.0, 0.0);
    let mut a = 0.0;
    let layer = |d: f64, c: (f64, f64, f64), col: &mut (f64, f64, f64), a: &mut f64| {
        let m = crate::smoothstep(aa, -aa, d);
        col.0 = crate::mix(col.0, c.0, m);
        col.1 = crate::mix(col.1, c.1, m);
        col.2 = crate::mix(col.2, c.2, m);
        *a = a.max(m);
    };

    let sponge = (0.26, 0.13, 0.07);
    let filling = (0.45, 0.26, 0.15);
    let icing = (0.17, 0.08, 0.04);
    let cream = (0.98, 0.95, 0.90);
    layer(
        sd_round_box(p - Vec2::<()>::new(0.0, -0.58), Vec2::<()>::new(0.78, 0.04), 0.04),
        (0.78, 0.80, 0.82),
        &mut col,
        &mut a,
    );
    layer(sd_round_box(p - Vec2::<()>::new(0.0, -0.26), Vec2::<()>::new(0.56, 0.29), 0.06), sponge, &mut col, &mut a);
    layer(sd_round_box(p - Vec2::<()>::new(0.0, -0.26), Vec2::<()>::new(0.56, 0.03), 0.02), filling, &mut col, &mut a);
    layer(sd_round_box(p - Vec2::<()>::new(0.0, 0.06), Vec2::<()>::new(0.58, 0.07), 0.07), icing, &mut col, &mut a);
    for i in 0..6 {
        let fi = i as f64;
        let x = -0.45 + fi * 0.18;
        let len = 0.04 + 0.05 * crate::fract((fi * 12.9898).sin() * 43758.5453);
        layer(
            sd_round_box(p - Vec2::<()>::new(x, -0.02 - len * 0.5), Vec2::<()>::new(0.035, len), 0.035),
            icing,
            &mut col,
            &mut a,
        );
    }
    layer(
        sd_round_box(p - Vec2::<()>::new(-0.12, 0.10), Vec2::<()>::new(0.30, 0.012), 0.012),
        (0.42, 0.25, 0.15),
        &mut col,
        &mut a,
    );
    for i in 0..4 {
        let x = if i < 2 { -0.42 + i as f64 * 0.2 } else { 0.22 + (i - 2) as f64 * 0.2 };
        layer((p - Vec2::<()>::new(x, 0.15)).magnitude_f64() - 0.06, cream, &mut col, &mut a);
        layer((p - Vec2::<()>::new(x, 0.21)).magnitude_f64() - 0.055, (0.70, 0.05, 0.08), &mut col, &mut a);
    }
    let stripe_k = if crate::fract((p.y + p.x) * 14.0) >= 0.5 {
        (0.85, 0.15, 0.2)
    } else {
        (1.0, 1.0, 1.0)
    };
    layer(sd_round_box(p - Vec2::<()>::new(0.0, 0.30), Vec2::<()>::new(0.03, 0.17), 0.01), stripe_k, &mut col, &mut a);
    let fp = p - Vec2::<()>::new(0.01 * (t * 31.0).sin(), 0.56);
    let stretch = if fp.y > 0.0 { 0.70 } else { 0.55 };
    let flame = (fp.x * fp.x + fp.y * fp.y * stretch * stretch).sqrt() - 0.045;
    // Flame colour: mix(orange, cream) by flame depth.
    let k = crate::smoothstep(0.0, -0.04, flame);
    layer(flame, (crate::mix(1.0, 0.95, k), crate::mix(0.55, 0.95, k), crate::mix(0.1, 0.6, k)), &mut col, &mut a);

    Color::<f64>::new_rgba(col.0, col.1, col.2, a)
}
