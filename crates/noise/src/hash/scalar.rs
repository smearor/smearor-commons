//! `Hash<f64>` impls - the GLSL `hash11`/`hash12`/`hash13` recipes.
//!
//! MIT License, <https://www.shadertoy.com/view/4djSRW> - Copyright (c)
//! 2014 David Hoskins.

use crate::Hash;
use crate::hash::fract;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;

/// GLSL `hash11` - scalar in, scalar out.
impl Hash<f64> for f64 {
    fn hash(&self) -> f64 {
        let mut p = fract(self * 0.1031);
        p *= p + 33.33;
        p *= p + p;
        fract(p)
    }
}

/// GLSL `hash12` - `p3 = fract(p.xyx * .1031)`, `yzx` mix, `xy*z` fold.
impl Hash<f64> for Vec2<()> {
    fn hash(&self) -> f64 {
        let mut p3 = self.xyx().map(|c| fract(c * 0.1031));
        let dot = p3.dot(&p3.yzx().map(|c| c + 33.33));

        p3 = p3.map(|c| c + dot);
        fract((p3.x + p3.y) * p3.z)
    }
}

/// GLSL `hash13` - `p3.zyx + 31.32` mix, `xy*z` fold.
impl Hash<f64> for Vec3<()> {
    fn hash(&self) -> f64 {
        let mut p3 = self.map(|c| fract(c * 0.1031));
        let dot = p3.dot(&p3.zyx().map(|c| c + 31.32));

        p3 = p3.map(|c| c + dot);
        fract((p3.x + p3.y) * p3.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash1x_outputs_are_in_unit_range() {
        for i in -100..100 {
            let f = i as f64 * 0.731;
            let a: f64 = f.hash();
            assert!((0.0..1.0).contains(&a));
            let b: f64 = Vec2::new(f, -f).hash();
            assert!((0.0..1.0).contains(&b));
            let c: f64 = Vec3::new(f, f * 0.5, -f).hash();
            assert!((0.0..1.0).contains(&c));
        }
    }
}
