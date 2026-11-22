//! `Simplex3D` - the Miropolskiy hash-simplex.
//!
//! Based on the MIT-licensed variant at
//! <https://www.shadertoy.com/view/XsX3zB> (Copyright (c) 2013 Nikita
//! Miropolskiy), whose gradients come from `hash33` - table-free and
//! fully deterministic.

mod fractal;

pub use fractal::Simplex3DFractal;

use crate::Field;
use crate::Hash;
use smearor_commons_geometry::Vec3;
use smearor_commons_geometry::Vec4;

/// 3D hash-simplex field - output in `[0, 1]`
/// (`dot(d, 52.0) * 0.5 + 0.5` semantics, the Burn-My-Windows
/// renormalized variant).
#[derive(Debug, Clone, Copy, Default)]
pub struct Simplex3D;

/// `1.0 - v` componentwise.
fn one_minus(v: Vec3<()>) -> Vec3<()> {
    Vec3::<()>::new(1.0 - v.x, 1.0 - v.y, 1.0 - v.z)
}

/// `step(vec3(0), v)` componentwise: 1.0 where `v >= 0`.
fn step3(v: Vec3<()>) -> Vec3<()> {
    v.map(|c| if c >= 0.0 { 1.0 } else { 0.0 })
}

/// The gradient contribution of lattice vertex `s`: `-0.5 + hash33(s)`.
fn gradient(s: Vec3<()>) -> Vec3<()> {
    let h: Vec3<()> = s.hash();
    h - Vec3::<()>::new(0.5, 0.5, 0.5)
}

impl Field<Vec3<()>> for Simplex3D {
    fn at(&self, p: Vec3<()>) -> f64 {
        const F3: f64 = 0.3333333; // skew to the simplex grid
        const G3: f64 = 0.1666667; // unskew

        // 1. find the containing tetrahedron's base vertex `s` and the
        // unskewed offsets `x`, `x1`, `x2`, `x3` to its four corners.
        let dot_p = p.x + p.y + p.z;
        let s = Vec3::<()>::new((p.x + dot_p * F3).floor(), (p.y + dot_p * F3).floor(), (p.z + dot_p * F3).floor());
        let dot_s = s.x + s.y + s.z;
        let x = p - s + Vec3::<()>::new(dot_s * G3, dot_s * G3, dot_s * G3);

        // e = step(vec3(0), x - x.yzx) - per-axis comparison.
        let e = step3(x - x.yzx());
        let i1 = e.component_mul(&one_minus(e.zxy()));
        let i2 = one_minus(e.zxy().component_mul(&one_minus(e)));

        let x1 = x - i1 + Vec3::<()>::new(G3, G3, G3);
        let x2 = x - i2 + Vec3::<()>::new(2.0 * G3, 2.0 * G3, 2.0 * G3);
        let x3 = x - Vec3::<()>::new(1.0, 1.0, 1.0) + Vec3::<()>::new(3.0 * G3, 3.0 * G3, 3.0 * G3);

        // 2. surflet weights fade 0.6 -> 0.0 at the margin, raised to
        // the fourth power; contributions are corner-dot products.
        let mut w = Vec4::<()>::new(0.6 - x.dot(&x), 0.6 - x1.dot(&x1), 0.6 - x2.dot(&x2), 0.6 - x3.dot(&x3)).map(|c| c.max(0.0));

        let d = Vec4::<()>::new(
            gradient(s).dot(&x),
            gradient(s + i1).dot(&x1),
            gradient(s + i2).dot(&x2),
            gradient(s + Vec3::<()>::new(1.0, 1.0, 1.0)).dot(&x3),
        );

        w = w.map(|c| c * c);
        w = w.map(|c| c * c);
        let d = Vec4::<()>::new(d.x * w.x, d.y * w.y, d.z * w.z, d.w * w.w);

        // 3. sum of the four surflets, renormalized to [0, 1].
        d.dot(&Vec4::new(52.0, 52.0, 52.0, 52.0)) * 0.5 + 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_in_unit_range() {
        for i in -30..30 {
            let p = Vec3::<()>::new(i as f64 * 0.173, -i as f64 * 0.291, i as f64 * 0.117);
            let v = Simplex3D.at(p);
            assert!((0.0..=1.0).contains(&v), "simplex3D({p}) = {v}");
        }
    }

    #[test]
    fn field_is_continuous() {
        let mut prev = Simplex3D.at(Vec3::<()>::new(0.0, 0.25, -0.1));
        for i in 1..2000 {
            let v = Simplex3D.at(Vec3::<()>::new(i as f64 * 0.005, 0.25, -0.1));
            assert!((v - prev).abs() < 0.2, "discontinuity at step {i}: {prev} -> {v}");
            prev = v;
        }
    }
}
