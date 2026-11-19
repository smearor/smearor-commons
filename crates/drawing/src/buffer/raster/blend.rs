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
        let dst = Color::<u8>::new_rgba(pixels[idx], pixels[idx + 1], pixels[idx + 2], pixels[idx + 3]);
        let blended: [u8; 4] = color.alpha_over(&dst, alpha).into();
        pixels[idx..idx + Rgba::BYTES_PER_PIXEL].copy_from_slice(&blended);
    }
}
