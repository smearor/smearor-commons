//! `Hash<Vec3<()>>` impls - the GLSL `hash31`/`hash32`/`hash33` recipes.
//!
//! MIT License, <https://www.shadertoy.com/view/4djSRW> - Copyright (c)
//! 2014 David Hoskins.

use crate::Hash;
use crate::hash::fract;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;

/// GLSL `hash31` - scalar broadcast, `yzx` mix, `(xxy + yzz) * zyx` fold.
impl Hash<Vec3<()>> for f64 {
    fn hash(&self) -> Vec3<()> {
        let mut p3 = Vec3::<()>::new(*self, *self, *self)
            .component_mul(&Vec3::new(0.1031, 0.1030, 0.0973))
            .map(fract);
        let dot = p3.dot(&p3.yzx().map(|c| c + 33.33));

        p3 = p3.map(|c| c + dot);
        Vec3::new(fract((p3.x + p3.y) * p3.z), fract((p3.x + p3.z) * p3.y), fract((p3.y + p3.z) * p3.x))
    }
}

/// GLSL `hash32` - `p.xyx`, `yxz` mix, `(xxy + yzz) * zyx` fold.
impl Hash<Vec3<()>> for Vec2<()> {
    fn hash(&self) -> Vec3<()> {
        let mut p3 = self.xyx().component_mul(&Vec3::new(0.1031, 0.1030, 0.0973)).map(fract);
        let dot = p3.dot(&p3.yxz().map(|c| c + 33.33));

        p3 = p3.map(|c| c + dot);
        Vec3::new(fract((p3.x + p3.y) * p3.z), fract((p3.x + p3.z) * p3.y), fract((p3.y + p3.z) * p3.x))
    }
}

/// GLSL `hash33` - `yxz` mix, `(xxy + yxx) * zyx` fold.
impl Hash<Vec3<()>> for Vec3<()> {
    fn hash(&self) -> Vec3<()> {
        let mut p3 = self.component_mul(&Vec3::new(0.1031, 0.1030, 0.0973)).map(fract);
        let dot = p3.dot(&p3.yxz().map(|c| c + 33.33));

        p3 = p3.map(|c| c + dot);
        Vec3::new(fract((p3.x + p3.y) * p3.z), fract((p3.x + p3.x) * p3.y), fract((p3.y + p3.x) * p3.x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash3x_outputs_are_in_unit_range() {
        for i in -100..100 {
            let f = i as f64 * 0.731;
            let a: Vec3<()> = f.hash();
            assert!((0.0..1.0).contains(&a.x) && (0.0..1.0).contains(&a.y) && (0.0..1.0).contains(&a.z));
            let b: Vec3<()> = Vec2::new(f, -f).hash();
            assert!((0.0..1.0).contains(&b.x) && (0.0..1.0).contains(&b.y) && (0.0..1.0).contains(&b.z));
            let c: Vec3<()> = Vec3::new(f, f * 0.5, -f).hash();
            assert!((0.0..1.0).contains(&c.x) && (0.0..1.0).contains(&c.y) && (0.0..1.0).contains(&c.z));
        }
    }
}
