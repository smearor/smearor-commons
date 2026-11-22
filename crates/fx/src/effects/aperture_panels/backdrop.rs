use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2DFractal;

/// "Star Field" backdrop constants.
const STAR_LAYERS: usize = 3;
const STAR_CELL_PX: f64 = 22.0;
const STAR_CELL_STEP: f64 = 28.0;
const STAR_DENSITY: f64 = 0.45;
const STAR_DENSITY_STEP: f64 = 0.1;
const STAR_TWINKLE: f64 = 0.4;
const STAR_GLOW: f64 = 0.15;

/// "Nebula" backdrop constants.
const NEBULA_SCALE: f64 = 520.0;
const NEBULA_WARP: f64 = 1.6;
const NEBULA_BRIGHT: f64 = 1.5;
const NEBULA_DRIFT: f64 = 0.04;
const NEBULA_COL_A: (f64, f64, f64) = (0.16, 0.06, 0.38);
const NEBULA_COL_B: (f64, f64, f64) = (0.62, 0.14, 0.52);
const NEBULA_COL_C: (f64, f64, f64) = (0.10, 0.50, 0.66);
const NEBULA_CORE: (f64, f64, f64) = (1.00, 0.86, 0.80);

/// `starHash2` - the shader's sin-hash, two components.
fn star_hash2(p: Vec2<()>) -> Vec2<()> {
    let a = (p.x * 127.1 + p.y * 311.7).sin() * 43758.5453;
    let b = (p.x * 269.5 + p.y * 183.3).sin() * 43758.5453;
    Vec2::<()>::new(crate::fract(a), crate::fract(b))
}

/// `starField` - stars at layer pixel `p`, fixed to the screen via
/// `screen_origin`; `seed` reshuffles the sky per animation.
pub(crate) fn star_field(p: Vec2<()>, screen_origin: Vec2<()>, seed: f64, t: f64) -> (f64, f64, f64) {
    let p = p + screen_origin;
    let mut col = (0.0, 0.0, 0.0);
    for i in 0..STAR_LAYERS {
        let fi = i as f64;
        let cell_px = STAR_CELL_PX + fi * STAR_CELL_STEP;
        let sp = Vec2::<()>::new(p.x / cell_px + seed * 97.3 + fi * 17.7, p.y / cell_px + seed * 53.1 - fi * 11.3);
        let cell = Vec2::<()>::new(sp.x.floor(), sp.y.floor());
        let h1 = star_hash2(cell);
        let h2 = star_hash2(cell + Vec2::<()>::new(5.17, 9.73));
        if h2.x < STAR_DENSITY - fi * STAR_DENSITY_STEP {
            let star = cell + Vec2::<()>::new(0.15 + 0.7 * h1.x, 0.15 + 0.7 * h1.y);
            let d = ((sp.x - star.x) * (sp.x - star.x) + (sp.y - star.y) * (sp.y - star.y)).sqrt() * cell_px;
            let radius = crate::mix(0.6, 0.9 + fi * 0.5, h2.y);
            let bright = crate::mix(0.5, 1.0, crate::fract(h2.y * 7.31)) * (0.75 + 0.15 * fi);
            let twinkle = 1.0 - STAR_TWINKLE * (0.5 + 0.5 * (t * (2.0 + 5.0 * h1.x) + std::f64::consts::TAU * h1.y).sin());
            let tint = {
                let f = crate::fract(h1.x * 13.7);
                (crate::mix(0.72, 1.0, f), crate::mix(0.82, 0.88, f), crate::mix(1.0, 0.72, f))
            };
            let core = (-(d * d) / (radius * radius)).exp();
            let halo = if i == STAR_LAYERS - 1 { STAR_GLOW * (-d / (radius * 3.0)).exp() } else { 0.0 };
            let k = bright * twinkle * (core + halo);
            col.0 += tint.0 * k;
            col.1 += tint.1 * k;
            col.2 += tint.2 * k;
        }
    }
    (col.0.min(1.0), col.1.min(1.0), col.2.min(1.0))
}

/// `nebula` - rgb plus density in alpha, at layer pixel `p`.
pub(crate) fn nebula(p: Vec2<()>, screen_origin: Vec2<()>, seed: f64, t: f64) -> Color<f64> {
    let p = p + screen_origin;
    let u = Vec2::<()>::new(p.x / NEBULA_SCALE + seed * 41.7, p.y / NEBULA_SCALE - seed * 23.3);

    let big = Simplex2DFractal.at(Vec2::<()>::new(u.x * 0.45 + 11.3, u.y * 0.45 - 4.1));
    let mask = crate::smoothstep(0.33, 0.60, big);

    let q = Vec2::<()>::new(Simplex2DFractal.at(u), Simplex2DFractal.at(u + Vec2::<()>::new(5.2, 1.3)));
    let f = Simplex2DFractal.at(u + q * NEBULA_WARP + Vec2::<()>::new(NEBULA_DRIFT * t, 0.0));
    let dens = mask * crate::smoothstep(0.25, 0.75, f);

    let k1 = crate::smoothstep(0.35, 0.65, q.x);
    let mut col = (
        crate::mix(NEBULA_COL_A.0, NEBULA_COL_B.0, k1),
        crate::mix(NEBULA_COL_A.1, NEBULA_COL_B.1, k1),
        crate::mix(NEBULA_COL_A.2, NEBULA_COL_B.2, k1),
    );
    let k2 = crate::smoothstep(0.40, 0.65, big) * crate::smoothstep(0.45, 0.60, q.y);
    col = (
        crate::mix(col.0, NEBULA_COL_C.0, k2),
        crate::mix(col.1, NEBULA_COL_C.1, k2),
        crate::mix(col.2, NEBULA_COL_C.2, k2),
    );
    let core = crate::smoothstep(0.6, 0.95, dens);
    let k3 = core * core * 0.5;
    col = (
        crate::mix(col.0, NEBULA_CORE.0, k3),
        crate::mix(col.1, NEBULA_CORE.1, k3),
        crate::mix(col.2, NEBULA_CORE.2, k3),
    );

    let dust = crate::smoothstep(0.45, 0.60, Simplex2DFractal.at(Vec2::<()>::new(u.x * 2.1 + 3.1, u.y * 2.1 - 7.7)));
    let lum = dens.powf(1.05) * NEBULA_BRIGHT * crate::mix(0.45, 1.0, dust);

    Color::<f64>::new_rgba(col.0 * lum, col.1 * lum, col.2 * lum, dens)
}
