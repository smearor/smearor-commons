//! `sampling` - texel reads through [`BufferView`].
//!
//! The sampling contract effect-style consumers rely on (mirroring
//! GLSL texture reads):
//!
//! - **uv space**: origin top-left, y down, `0..1` spans the texture -
//!   the GNOME `iTexCoord` convention.
//! - **clamp-to-edge**: out-of-range uv coordinates read the nearest
//!   edge texel (`clamp(p, 0, size-1)`); no wrapping, no border color.
//! - **alpha**: pixel formats decode to straight-alpha RGBA through
//!   [`PixelFormat::read_pixel`] - premultiplied formats are
//!   unpremultiplied on read, so samplers always see straight alpha.
//!
//! Samples return normalized [`Color<f64>`] channels (`0.0..=1.0`).
//! The hot paths work on raw `[f64; 4]` channels with view
//! dimensions hoisted out of the per-tap loop - `sample_cone` taps
//! are only a clamp, an offset and a weighted sum each.

use crate::BufferView;
use crate::PixelFormat;
use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

/// Normalized-channel scale - `u8 -> 0..=1`.
const INV_BYTE: f64 = 1.0 / 255.0;

/// Transparent black - the empty-view read.
const EMPTY: [f64; 4] = [0.0, 0.0, 0.0, 0.0];

impl<F: PixelFormat> BufferView<'_, F> {
    /// The clamped texel at integer coordinates as normalized
    /// channels - the hot-path read behind every sampler. `width`
    /// and `height` must be at least 1 (the public front-ends guard
    /// the empty view).
    #[inline]
    fn texel_clamped(&self, width: i64, height: i64, x: i64, y: i64) -> [f64; 4] {
        let x = x.clamp(0, width - 1) as usize;
        let y = y.clamp(0, height - 1) as usize;
        let offset = y * self.metadata.stride.as_usize() + x * F::BYTES_PER_PIXEL;
        let [r, g, b, a] = F::read_pixel(&self.data[offset..offset + F::BYTES_PER_PIXEL]);
        [r as f64 * INV_BYTE, g as f64 * INV_BYTE, b as f64 * INV_BYTE, a as f64 * INV_BYTE]
    }

    /// The bilinear tap at `uv` as normalized channels, clamp-to-edge
    /// on every texel - the shared inner loop of `sample_bilinear`
    /// and `sample_cone`. Same `width`/`height >= 1` contract as
    /// [`texel_clamped`](Self::texel_clamped).
    #[inline]
    fn bilinear(&self, width: i64, height: i64, uv: Vec2<()>) -> [f64; 4] {
        let x = uv.x * width as f64 - 0.5;
        let y = uv.y * height as f64 - 0.5;
        let (x0, y0) = (x.floor() as i64, y.floor() as i64);
        let (tx, ty) = (x - x0 as f64, y - y0 as f64);
        let tl = self.texel_clamped(width, height, x0, y0);
        let tr = self.texel_clamped(width, height, x0 + 1, y0);
        let bl = self.texel_clamped(width, height, x0, y0 + 1);
        let br = self.texel_clamped(width, height, x0 + 1, y0 + 1);
        let lerp = |a: [f64; 4], b: [f64; 4], t: f64| {
            [
                a[0] + (b[0] - a[0]) * t,
                a[1] + (b[1] - a[1]) * t,
                a[2] + (b[2] - a[2]) * t,
                a[3] + (b[3] - a[3]) * t,
            ]
        };
        lerp(lerp(tl, tr, tx), lerp(bl, br, tx), ty)
    }

    /// View dimensions for the samplers - `None` on empty views.
    #[inline]
    fn dims(&self) -> Option<(i64, i64)> {
        let size = self.size();
        (size.x > 0 && size.y > 0).then_some((size.x as i64, size.y as i64))
    }

    /// Nearest-texel sampling - the texel whose center is closest to
    /// `uv` (pixelate-style, no interpolation). Empty views yield
    /// transparent black.
    pub fn sample_nearest(&self, uv: Vec2<()>) -> Color<f64> {
        let Some((width, height)) = self.dims() else {
            return Color::<f64>::BLACK.with_alpha(0.0);
        };
        let [r, g, b, a] = self.texel_clamped(width, height, (uv.x * width as f64 - 0.5).round() as i64, (uv.y * height as f64 - 0.5).round() as i64);
        Color::<f64>::new_rgba(r, g, b, a)
    }

    /// Bilinear sampling - the weighted mean of the four surrounding
    /// texels, clamp-to-edge on every tap. Empty views and non-finite
    /// uv coordinates (warped effect coords can go `inf`/`NaN`) yield
    /// transparent black.
    pub fn sample_bilinear(&self, uv: Vec2<()>) -> Color<f64> {
        if !uv.x.is_finite() || !uv.y.is_finite() {
            return Color::<f64>::BLACK.with_alpha(0.0);
        }
        let Some((width, height)) = self.dims() else {
            return Color::<f64>::BLACK.with_alpha(0.0);
        };
        let [r, g, b, a] = self.bilinear(width, height, uv);
        Color::<f64>::new_rgba(r, g, b, a)
    }

    /// The ray offsets of a radial (cone) blur: `taps` samples along
    /// each of `directions` evenly spaced rays of `radius` uv units
    /// around the sample point. The radius is a `Vec2`, so
    /// anisotropic (elliptical) blurs like `radius_px / size` are
    /// expressible. Compute once per frame - the same offsets serve
    /// every pixel's [`sample_cone`](Self::sample_cone) call.
    pub fn cone_offsets(radius: Vec2<()>, directions: u32, taps: u32) -> Vec<Vec2<()>> {
        if directions == 0 || taps == 0 || (radius.x <= 0.0 && radius.y <= 0.0) {
            return Vec::new();
        }
        let mut offsets = Vec::with_capacity((directions * taps) as usize);
        for direction in 0..directions {
            let angle = direction as f64 / directions as f64 * TAU;
            let (cos, sin) = (angle.cos(), angle.sin());
            for tap in 1..=taps {
                let t = tap as f64 / taps as f64;
                offsets.push(Vec2::<()>::new(cos * radius.x * t, sin * radius.y * t));
            }
        }
        offsets
    }

    /// Cone-blur sampling - the mean of bilinear taps at `uv` plus
    /// each precomputed `offsets` entry (see
    /// [`cone_offsets`](Self::cone_offsets)). An empty offset list
    /// degenerates to the plain [`sample_bilinear`](Self::sample_bilinear)
    /// tap.
    pub fn sample_cone(&self, uv: Vec2<()>, offsets: &[Vec2<()>]) -> Color<f64> {
        let Some((width, height)) = self.dims() else {
            return Color::<f64>::BLACK.with_alpha(0.0);
        };
        if offsets.is_empty() {
            let [r, g, b, a] = self.bilinear(width, height, uv);
            return Color::<f64>::new_rgba(r, g, b, a);
        }
        let mut acc = EMPTY;
        for offset in offsets {
            let c = self.bilinear(width, height, uv + *offset);
            acc[0] += c[0];
            acc[1] += c[1];
            acc[2] += c[2];
            acc[3] += c[3];
        }
        let count = offsets.len() as f64;
        Color::<f64>::new_rgba(acc[0] / count, acc[1] / count, acc[2] / count, acc[3] / count)
    }

    /// Radial-blur sampling - the mean of `taps` bilinear samples
    /// along `directions` evenly spaced rays of `radius` uv units
    /// around `uv`. A zero component degenerates to the center texel.
    /// For per-pixel use prefer [`cone_offsets`](Self::cone_offsets)
    /// + [`sample_cone`](Self::sample_cone): this convenience
    ///   recomputes the ray table on every call.
    pub fn sample_radial_blur(&self, uv: Vec2<()>, radius: Vec2<()>, directions: u32, taps: u32) -> Color<f64> {
        self.sample_cone(uv, &Self::cone_offsets(radius, directions, taps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgba;
    use smearor_commons_geometry::Size;

    /// A 2x2 checker: black, red, green, white (row-major, opaque).
    fn checker() -> (Vec<u8>, BufferView<'static, Rgba>) {
        let data: &[u8] = &[0, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 255, 255, 255, 255];
        (data.to_vec(), BufferView::<Rgba>::from_packed(data, Size::new(2, 2)).unwrap())
    }

    #[test]
    fn bilinear_samples_exact_corners_and_center_mean() {
        let (_data, view) = checker();
        // Texel centers sit at (0.25, 0.25) etc. in uv.
        assert_eq!(view.sample_bilinear(Vec2::<()>::new(0.25, 0.25)), Color::<f64>::new_rgba(0.0, 0.0, 0.0, 1.0));
        assert_eq!(view.sample_bilinear(Vec2::<()>::new(0.75, 0.25)), Color::<f64>::new_rgba(1.0, 0.0, 0.0, 1.0));
        let center = view.sample_bilinear(Vec2::<()>::new(0.5, 0.5));
        // Mean of black, red, green, white: (0.5, 0.5, 0.25, 1.0).
        assert!((center.r - 0.5).abs() < 1e-9, "center r = {}", center.r);
        assert!((center.g - 0.5).abs() < 1e-9, "center g = {}", center.g);
        assert!((center.b - 0.25).abs() < 1e-9, "center b = {}", center.b);
        assert!((center.a - 1.0).abs() < 1e-9);
    }

    #[test]
    fn out_of_range_uv_clamps_to_edge() {
        let (_data, view) = checker();
        assert_eq!(view.sample_bilinear(Vec2::<()>::new(-0.5, -0.5)), Color::<f64>::new_rgba(0.0, 0.0, 0.0, 1.0));
        assert_eq!(view.sample_bilinear(Vec2::<()>::new(1.5, 1.5)), Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn nearest_picks_the_cell_texel() {
        let (_data, view) = checker();
        assert_eq!(view.sample_nearest(Vec2::<()>::new(0.49, 0.5)), Color::<f64>::new_rgba(0.0, 1.0, 0.0, 1.0));
        assert_eq!(view.sample_nearest(Vec2::<()>::new(0.51, 0.5)), Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn radial_blur_averages_the_neighborhood() {
        let (_data, view) = checker();
        let offsets = BufferView::<Rgba>::cone_offsets(Vec2::<()>::new(0.3, 0.3), 8, 2);
        let v = view.sample_cone(Vec2::<()>::new(0.5, 0.5), &offsets);
        // Mean of an all-four-texel neighborhood in a checker is ~0.5 per channel.
        assert!((v.r - 0.5).abs() < 0.3, "radial r = {}", v.r);
        assert_eq!(view.sample_cone(Vec2::<()>::new(0.25, 0.25), &[]), Color::<f64>::new_rgba(0.0, 0.0, 0.0, 1.0));
    }
}
