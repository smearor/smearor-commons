//! `Simplex3DFractal` - the fixed four-octave variant of
//! [`Simplex3D`].
//!
//! Based on the MIT-licensed variant at
//! <https://www.shadertoy.com/view/XsX3zB> (Copyright (c) 2013 Nikita
//! Miropolskiy): the domain is rotated by three fixed `mat3` matrices
//! between octaves to break up directional artifacts, weights
//! `0.5333/0.2667/0.1333/0.0667`.

use super::Simplex3D;
use crate::Field;
use smearor_commons_geometry::Vec3;

/// The fixed four-octave fractal variant of [`Simplex3D`] - rotates the
/// domain by three fixed `mat3` matrices between octaves to break up
/// directional artifacts, weights `0.5333/0.2667/0.1333/0.0667`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Simplex3DFractal;

/// mat3-by-column rotation: `cols[0]*v.x + cols[1]*v.y + cols[2]*v.z`.
fn rotate(cols: [Vec3<()>; 3], v: Vec3<()>) -> Vec3<()> {
    cols[0] * v.x + cols[1] * v.y + cols[2] * v.z
}

/// The three octave rotations (GLSL mat3 constructors are
/// column-major).
const ROT1: [Vec3<()>; 3] = [
    Vec3::<()>::new(-0.37, 0.36, 0.85),
    Vec3::<()>::new(-0.14, -0.93, 0.34),
    Vec3::<()>::new(0.92, 0.01, 0.4),
];
const ROT2: [Vec3<()>; 3] = [
    Vec3::<()>::new(-0.55, -0.39, 0.74),
    Vec3::<()>::new(0.33, -0.91, -0.24),
    Vec3::<()>::new(0.77, 0.12, 0.63),
];
const ROT3: [Vec3<()>; 3] = [
    Vec3::<()>::new(-0.71, 0.52, -0.47),
    Vec3::<()>::new(-0.08, -0.72, -0.68),
    Vec3::<()>::new(-0.7, -0.45, 0.56),
];

impl Field<Vec3<()>> for Simplex3DFractal {
    fn at(&self, m: Vec3<()>) -> f64 {
        0.5333333 * Simplex3D.at(rotate(ROT1, m))
            + 0.2666667 * Simplex3D.at(rotate(ROT2, m * 2.0))
            + 0.1333333 * Simplex3D.at(rotate(ROT3, m * 4.0))
            + 0.0666667 * Simplex3D.at(m * 8.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractal_rotates_octaves() {
        let a = Simplex3DFractal.at(Vec3::<()>::new(0.31, -0.7, 0.2));
        assert_ne!(a, Simplex3D.at(Vec3::<()>::new(0.31, -0.7, 0.2)));
        assert!(a.is_finite());
    }
}
