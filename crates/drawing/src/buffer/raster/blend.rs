//! Shared pixel compositing for the raster backend.

use smearor_commons_buffer::PixelFormat;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;

/// Blend `color` onto the pixel at byte `idx` with source-over
/// compositing; `alpha` composes with the color's own alpha channel.
/// Shared by the primitive-shape scans in `primitive_ops`/`progress_ops`.
pub(in crate::buffer) fn blend_pixel(pixels: &mut [u8], idx: usize, color: Color<u8>, alpha: Alpha) {
    let alpha = Alpha::from(color.a) * f64::from(alpha);
    if !alpha.is_transparent() {
        let pixel = &mut pixels[idx..idx + Rgba::BYTES_PER_PIXEL];
        let [r, g, b, a] = Rgba::read_pixel(pixel);
        let blended = color.alpha_over(&Color::<u8>::new_rgba(r, g, b, a), alpha);
        Rgba::write_pixel(pixel, blended.r, blended.g, blended.b, blended.a);
    }
}
