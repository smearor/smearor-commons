//! `Simplex2D` - the Quilez hash-simplex field.
//!
//! Based on the MIT-licensed variant at
//! <https://www.shadertoy.com/view/Msf3WH> (Copyright (c) 2013 Inigo
//! Quilez), whose gradients come from `hash22` - fully deterministic,
//! no permutation table, no trig.

mod fractal;

pub use fractal::Simplex2DFractal;

use crate::Field;
use crate::Hash;
use smearor_commons_geometry::Vec2;

/// 2D hash-simplex field - output in `[0, 1]`
/// (`0.5 + 0.5 * dot(n, 70.0)` semantics).
#[derive(Debug, Clone, Copy, Default)]
pub struct Simplex2D;

/// Gradient at lattice point `i`: `-1 + 2 * hash22(i)`.
fn gradient(i: Vec2<()>) -> Vec2<()> {
    let h: Vec2<()> = i.hash();
    h * 2.0 - Vec2::<()>::new(1.0, 1.0)
}

impl Field<Vec2<()>> for Simplex2D {
    /// Simplex noise in `[0, 1]` - continuous, deterministic.
    fn at(&self, p: Vec2<()>) -> f64 {
        const K1: f64 = 0.366025404; // (sqrt(3) - 1) / 2
        const K2: f64 = 0.211324865; // (3 - sqrt(3)) / 6

        let i = Vec2::<()>::new((p.x + (p.x + p.y) * K1).floor(), (p.y + (p.x + p.y) * K1).floor());
        let a = Vec2::<()>::new(p.x - i.x + (i.x + i.y) * K2, p.y - i.y + (i.x + i.y) * K2);
        // step(a.y, a.x): 1.0 where a.x >= a.y - the triangle edge.
        let m = if a.x >= a.y { 1.0 } else { 0.0 };
        let o = Vec2::<()>::new(m, 1.0 - m);
        let b = a - o + Vec2::<()>::new(K2, K2);
        let c = a - Vec2::<()>::new(1.0, 1.0) + Vec2::<()>::new(2.0 * K2, 2.0 * K2);

        // Surflet weights: h = max(0.5 - dot(corner, corner), 0)^4.
        let ha = (0.5 - a.dot::<f64>(&a)).max(0.0);
        let hb = (0.5 - b.dot::<f64>(&b)).max(0.0);
        let hc = (0.5 - c.dot::<f64>(&c)).max(0.0);

        let n = ha * ha * ha * ha * a.dot::<f64>(&gradient(i))
            + hb * hb * hb * hb * b.dot::<f64>(&gradient(i + o))
            + hc * hc * hc * hc * c.dot::<f64>(&gradient(i + Vec2::<()>::new(1.0, 1.0)));

        0.5 + 0.5 * n * 70.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_in_unit_range() {
        for i in -50..50 {
            for j in -50..50 {
                let p = Vec2::<()>::new(i as f64 * 0.173, j as f64 * 0.291);
                let v = Simplex2D.at(p);
                assert!((0.0..=1.0).contains(&v), "simplex2D({p}) = {v}");
            }
        }
    }

    #[test]
    fn field_is_continuous_between_cells() {
        // A fine walk must not jump - no hash popping at cell borders.
        let mut prev = Simplex2D.at(Vec2::<()>::new(0.0, 0.25));
        for i in 1..2000 {
            let v = Simplex2D.at(Vec2::<()>::new(i as f64 * 0.005, 0.25));
            assert!((v - prev).abs() < 0.15, "discontinuity at step {i}: {prev} -> {v}");
            prev = v;
        }
    }
}
