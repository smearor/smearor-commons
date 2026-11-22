//! `Hash<Vec2<()>>` impls - the GLSL `hash21`/`hash22`/`hash23` recipes.
//!
//! MIT License, <https://www.shadertoy.com/view/4djSRW> - Copyright (c)
//! 2014 David Hoskins.

use crate::Hash;
use crate::hash::fract;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;

/// Shared `hash2x` tail: `(p3.xx + p3.yz) * p3.zy`, componentwise fract.
fn fold2(p3: Vec3<()>) -> Vec2<()> {
    Vec2::new(fract((p3.x + p3.y) * p3.z), fract((p3.x + p3.z) * p3.y))
}

/// Shared `hash2x` mix: `p3 += dot(p3, p3.yzx + 33.33)`.
fn mix2(p3: Vec3<()>) -> Vec3<()> {
    let dot = p3.dot(&p3.yzx().map(|c| c + 33.33));
    p3.map(|c| c + dot)
}

/// GLSL `hash21` - scalar broadcast with `(.1030, .1030, .0973)`.
impl Hash<Vec2<()>> for f64 {
    fn hash(&self) -> Vec2<()> {
        let p3 = Vec3::new(*self, *self, *self).component_mul(&Vec3::new(0.1030, 0.1030, 0.0973)).map(fract);
        fold2(mix2(p3))
    }
}

/// GLSL `hash22` - `p.xyx` with `(.1031, .1030, .0973)`.
impl Hash<Vec2<()>> for Vec2<()> {
    fn hash(&self) -> Vec2<()> {
        let p3 = self.xyx().component_mul(&Vec3::new(0.1031, 0.1030, 0.0973)).map(fract);
        fold2(mix2(p3))
    }
}

/// GLSL `hash23` - full `p3` with `(.1031, .1030, .0973)`.
impl Hash<Vec2<()>> for Vec3<()> {
    fn hash(&self) -> Vec2<()> {
        let p3 = self.component_mul(&Vec3::new(0.1031, 0.1030, 0.0973)).map(fract);
        fold2(mix2(p3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash2x_outputs_are_in_unit_range() {
        for i in -100..100 {
            let f = i as f64 * 0.731;
            let a: Vec2<()> = f.hash();
            assert!((0.0..1.0).contains(&a.x) && (0.0..1.0).contains(&a.y));
            let b: Vec2<()> = Vec2::new(f, -f).hash();
            assert!((0.0..1.0).contains(&b.x) && (0.0..1.0).contains(&b.y));
            let c: Vec2<()> = Vec3::new(f, f * 0.5, -f).hash();
            assert!((0.0..1.0).contains(&c.x) && (0.0..1.0).contains(&c.y));
        }
    }
}
