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

use crate::BufferView;
use crate::PixelFormat;
use smearor_commons_color::Color;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use std::f64::consts::TAU;

impl<F: PixelFormat> BufferView<'_, F> {
    /// The texel at integer pixel coordinates as normalized color -
    /// `None` outside the view.
    fn texel(&self, x: i64, y: i64) -> Option<Color<f64>> {
        let size = self.size();
        if x < 0 || y < 0 || x >= size.x as i64 || y >= size.y as i64 {
            return None;
        }
        let offset = self.pixel_offset(Pixels::new(x as u32), Pixels::new(y as u32))?;
        let [r, g, b, a] = F::read_pixel(&self.data()[offset..offset + F::BYTES_PER_PIXEL]);
        Some(Color::<f64>::new_rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0))
    }

    /// The clamped texel read - out-of-range coordinates read the
    /// nearest edge texel. Only defined for non-empty views; empty
    /// views yield transparent black.
    fn texel_clamped(&self, x: i64, y: i64) -> Color<f64> {
        let size = self.size();
        let x = x.clamp(0, size.x as i64 - 1);
        let y = y.clamp(0, size.y as i64 - 1);
        self.texel(x, y).unwrap_or(Color::<f64>::BLACK.with_alpha(0.0))
    }

    /// Nearest-texel sampling - the texel whose center is closest to
    /// `uv` (pixelate-style, no interpolation).
    pub fn sample_nearest(&self, uv: Vec2<()>) -> Color<f64> {
        let size = self.size();
        self.texel_clamped((uv.x * size.x as f64 - 0.5).round() as i64, (uv.y * size.y as f64 - 0.5).round() as i64)
    }

    /// Bilinear sampling - the weighted mean of the four surrounding
    /// texels, clamp-to-edge on every tap.
    pub fn sample_bilinear(&self, uv: Vec2<()>) -> Color<f64> {
        let size = self.size();
        let x = uv.x * size.x as f64 - 0.5;
        let y = uv.y * size.y as f64 - 0.5;
        let (x0, y0) = (x.floor() as i64, y.floor() as i64);
        let (tx, ty) = (x - x0 as f64, y - y0 as f64);
        let top = self.texel_clamped(x0, y0).lerp(&self.texel_clamped(x0 + 1, y0), tx);
        let bottom = self.texel_clamped(x0, y0 + 1).lerp(&self.texel_clamped(x0 + 1, y0 + 1), tx);
        top.lerp(&bottom, ty)
    }

    /// Radial-blur sampling - the mean of `taps` bilinear samples
    /// along `directions` evenly spaced rays of `radius` uv units
    /// around `uv`. A `radius` of `0.0` degenerates to the center
    /// texel.
    pub fn sample_radial_blur(&self, uv: Vec2<()>, radius: f64, directions: u32, taps: u32) -> Color<f64> {
        if radius <= 0.0 || directions == 0 || taps == 0 {
            return self.sample_bilinear(uv);
        }
        let mut acc = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
        for direction in 0..directions {
            let angle = direction as f64 / directions as f64 * TAU;
            for tap in 1..=taps {
                let r = radius * tap as f64 / taps as f64;
                let c = self.sample_bilinear(Vec2::<()>::new(uv.x + angle.cos() * r, uv.y + angle.sin() * r));
                acc = Color::<f64>::new_rgba(acc.r + c.r, acc.g + c.g, acc.b + c.b, acc.a + c.a);
            }
        }
        let count = (directions * taps) as f64;
        Color::<f64>::new_rgba(acc.r / count, acc.g / count, acc.b / count, acc.a / count)
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
        let v = view.sample_radial_blur(Vec2::<()>::new(0.5, 0.5), 0.3, 8, 2);
        // Mean of an all-four-texel neighborhood in a checker is ~0.5 per channel.
        assert!((v.r - 0.5).abs() < 0.3, "radial r = {}", v.r);
        assert_eq!(view.sample_radial_blur(Vec2::<()>::new(0.25, 0.25), 0.0, 8, 2), Color::<f64>::new_rgba(0.0, 0.0, 0.0, 1.0));
    }
}
