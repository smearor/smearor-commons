//! `Hash<Vec4<()>>` impls - the GLSL `hash41`/`hash42`/`hash43`/`hash44`
//! recipes.
//!
//! MIT License, <https://www.shadertoy.com/view/4djSRW> - Copyright (c)
//! 2014 David Hoskins.

use crate::Hash;
use crate::hash::fract;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use smearor_commons_geometry::Vec4;

const RECIPE: Vec4<()> = Vec4::<()>::new(0.1031, 0.1030, 0.0973, 0.1099);

/// `vec4 * vec4` componentwise multiply.
/// Shared `hash4x` fold: `(xxyz + yzzw) * zywx`, componentwise fract.
fn fold4(p4: Vec4<()>) -> Vec4<()> {
    Vec4::<()>::new(
        fract((p4.x + p4.y) * p4.z),
        fract((p4.x + p4.z) * p4.y),
        fract((p4.y + p4.z) * p4.w),
        fract((p4.z + p4.w) * p4.x),
    )
}

/// Shared `hash4x` recipe: fract-multipliers, `wzxy` dot mix, fold.
fn hash4(p4: Vec4<()>) -> Vec4<()> {
    let mut p4 = p4.component_mul(&RECIPE).map(fract);
    p4 = p4 + Vec4::<()>::new(1.0, 1.0, 1.0, 1.0) * p4.dot(&p4.wzxy().map(|c| c + 33.33));
    fold4(p4)
}

/// GLSL `hash41` - scalar broadcast to four components.
impl Hash<Vec4<()>> for f64 {
    fn hash(&self) -> Vec4<()> {
        hash4(Vec4::<()>::new(*self, *self, *self, *self))
    }
}

/// GLSL `hash42` - `p.xyxy` broadcast to four components.
impl Hash<Vec4<()>> for Vec2<()> {
    fn hash(&self) -> Vec4<()> {
        hash4(Vec4::<()>::new(self.x, self.y, self.x, self.y))
    }
}

/// GLSL `hash43` - `p.xyzx` broadcast to four components.
impl Hash<Vec4<()>> for Vec3<()> {
    fn hash(&self) -> Vec4<()> {
        hash4(Vec4::<()>::new(self.x, self.y, self.z, self.x))
    }
}

/// GLSL `hash44` - full `p4` input.
impl Hash<Vec4<()>> for Vec4<()> {
    fn hash(&self) -> Vec4<()> {
        hash4(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_range(v: Vec4<()>) {
        for c in [v.x, v.y, v.z, v.w] {
            assert!((0.0..1.0).contains(&c), "component {c} out of range");
        }
    }

    #[test]
    fn hash4x_outputs_are_in_unit_range() {
        for i in -100..100 {
            let f = i as f64 * 0.731;
            in_range(Hash::<Vec4<()>>::hash(&f));
            in_range(Hash::<Vec4<()>>::hash(&Vec2::new(f, -f)));
            in_range(Hash::<Vec4<()>>::hash(&Vec3::new(f, f * 0.5, -f)));
            in_range(Hash::<Vec4<()>>::hash(&Vec4::<()>::new(f, -f, 0.25, f * 2.0)));
        }
    }
}
