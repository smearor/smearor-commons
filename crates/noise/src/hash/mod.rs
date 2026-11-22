//! `Hash` - the Hoskins "hash without sine" family as a generic trait.
//!
//! MIT License, <https://www.shadertoy.com/view/4djSRW> - Copyright (c)
//! 2014 David Hoskins. The recipe is `fract`/multiply/add only: no trig,
//! no tables - bitwise identical output on every platform.

/// The Hoskins fract-hash family.
///
/// `p.hash()` returns the `Out`-shaped hash of `p` with every component
/// in `[0, 1)`. The 13 impls mirror the GLSL `hashNN` naming (input
/// dimension + output dimension); each impl's rustdoc names its
/// `hashNN` designation. This is *not* [`std::hash::Hash`] - the
/// no-arg `Out`-returning signature never collides at call sites.
pub trait Hash<Out> {
    /// The deterministic hash value in `[0, 1)` per component.
    fn hash(&self) -> Out;
}

/// GLSL `fract(x) = x - x.floor()` - exact for negatives (unlike
/// `rem_euclid` variants that behave differently around integers).
pub(crate) fn fract(x: f64) -> f64 {
    x - x.floor()
}

mod scalar;
mod vec2;
mod vec3;
mod vec4;

#[cfg(test)]
mod tests {

    use super::*;
    use smearor_commons_geometry::Vec2;
    use smearor_commons_geometry::Vec3;
    use smearor_commons_geometry::Vec4;

    /// Regression pins against the ported Hoskins formula - if any of
    /// these drift, the recipe constants changed.
    #[test]
    fn known_values() {
        let h11: f64 = 1.0f64.hash();
        assert!((h11 - 0.7629645911716167).abs() < 1e-12);
        let h12: f64 = Vec2::<()>::new(0.5, -1.25).hash();
        assert!((h12 - 0.1989645971389109).abs() < 1e-12);
        let h13: f64 = Vec3::<()>::new(1.0, 2.0, 3.0).hash();
        assert!((h13 - 0.17883083253718723).abs() < 1e-12);
        let h22: Vec2<()> = Vec2::<()>::new(0.5, -1.25).hash();
        assert!((h22.x - 0.538394836124553).abs() < 1e-12 && (h22.y - 0.28917051798407556).abs() < 1e-12);
        let h44: Vec4<()> = Vec4::<()>::new(1.0, 2.0, 3.0, 4.0).hash();
        assert!((h44.x - 0.5798467050076397).abs() < 1e-12 && (h44.w - 0.12897491332751088).abs() < 1e-12);
    }

    /// Rough uniformity: mean ~ 0.5, variance ~ 1/12 over a 10k grid.
    #[test]
    fn hash_is_roughly_uniform() {
        let mut sum = 0.0;
        let mut sq = 0.0;
        let n = 10000.0;
        for i in 0..10000 {
            let v: f64 = (i as f64 * 0.173).hash();
            sum += v;
            sq += v * v;
        }
        let mean = sum / n;
        let variance = sq / n - mean * mean;
        assert!((mean - 0.5).abs() < 0.02, "mean {mean}");
        assert!((variance - 1.0 / 12.0).abs() < 0.01, "variance {variance}");
    }
}
