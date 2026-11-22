//! `Simplex2DFractal` - the fixed four-octave variant of
//! [`Simplex2D`].
//!
//! Based on the MIT-licensed variant at
//! <https://www.shadertoy.com/view/Msf3WH> (Copyright (c) 2013 Inigo
//! Quilez): the domain is rotated-and-scaled by `mat2(1.6, 1.2, -1.2,
//! 1.6)` between octaves to break up directional artifacts, weights
//! `0.5/0.25/0.125/0.0625`.

use super::Simplex2D;
use crate::Field;
use smearor_commons_geometry::Vec2;

/// The fixed four-octave fractal variant of [`Simplex2D`] - rotates and
/// scales the domain by `mat2(1.6, 1.2, -1.2, 1.6)` between octaves to
/// break up directional artifacts, weights `0.5/0.25/0.125/0.0625`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Simplex2DFractal;

impl Field<Vec2<()>> for Simplex2DFractal {
    fn at(&self, p: Vec2<()>) -> f64 {
        // mat2(1.6, 1.2, -1.2, 1.6) column-major: rotate-and-scale.
        let rotate = |v: Vec2<()>| v.component_mul(&Vec2::<()>::new(1.6, 1.6)) + Vec2::<()>::new(-v.y * 1.2, v.x * 1.2);
        let mut f = 0.5 * Simplex2D.at(p);
        let mut p = rotate(p);
        f += 0.25 * Simplex2D.at(p);
        p = rotate(p);
        f += 0.125 * Simplex2D.at(p);
        p = rotate(p);
        f += 0.0625 * Simplex2D.at(p);
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractal_varies_and_stays_bounded() {
        let a = Simplex2DFractal.at(Vec2::<()>::new(0.31, -0.7));
        let b = Simplex2DFractal.at(Vec2::<()>::new(0.32, -0.7));
        assert_ne!(a, Simplex2D.at(Vec2::<()>::new(0.31, -0.7)), "fractal should differ from base");
        assert!(a != b, "still continuous input variation");
        for i in -20..20 {
            let v = Simplex2DFractal.at(Vec2::<()>::new(i as f64 * 0.37, 0.5));
            assert!((-0.1..=1.1).contains(&v), "fractal roughly bounded: {v}");
        }
    }
}
