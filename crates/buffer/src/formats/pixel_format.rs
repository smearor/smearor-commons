use crate::PixelFormatId;

/// Premultiply one color channel by alpha, rounding to nearest.
pub(super) fn premultiply(channel: u8, alpha: u8) -> u8 {
    ((u16::from(channel) * u16::from(alpha) + 127) / 255) as u8
}

/// Undo premultiplication on one color channel; a zero alpha yields a
/// zero channel (premultiplied `a=0` carries no color information).
pub(super) fn unpremultiply(channel: u8, alpha: u8) -> u8 {
    if alpha == 0 {
        0
    } else {
        ((u32::from(channel) * 255 + u32::from(alpha) / 2) / u32::from(alpha)).min(255) as u8
    }
}

/// Rec.601 luma for the `Gray` format: `0.299·r + 0.587·g + 0.114·b`
/// as a fixed-point byte computation.
pub(super) fn luma(r: u8, g: u8, b: u8) -> u8 {
    ((77 * u16::from(r) + 150 * u16::from(g) + 29 * u16::from(b) + 128) >> 8) as u8
}

/// The byte layout of a pixel format - implemented by marker types.
///
/// Implementors are zero-sized tags carried as `PhantomData` on
/// [`crate::PixelBuffer`] and the buffer views; the associated
/// constants supply the byte arithmetic and a runtime identifier.
///
/// `pixel_bytes`/`read_pixel` anchor the channel-order knowledge on the
/// format itself: callers always think in straight-alpha RGBA channels,
/// and each format maps them to its byte order - including premultiply/
/// unpremultiply for premultiplied formats and channel expansion for
/// alpha-free layouts.
pub trait PixelFormat {
    /// Bytes per pixel.
    const BYTES_PER_PIXEL: usize;

    /// Whether the format carries an alpha channel - `false` for
    /// formats without alpha and for padding-channel formats like
    /// [`crate::Rgbx`], where the fourth byte is ignored.
    const HAS_ALPHA: bool;

    /// The runtime identifier for diagnostics and wire formats.
    const ID: PixelFormatId;

    /// Encode straight-alpha RGBA channels into `dst` in this format's
    /// byte order (`dst.len() == Self::BYTES_PER_PIXEL`). Premultiplied
    /// formats multiply the color channels by `a`; alpha-free formats
    /// drop `a` (padding bytes read as opaque); single-channel formats
    /// reduce to luminance.
    fn write_pixel(dst: &mut [u8], r: u8, g: u8, b: u8, a: u8);

    /// Decode a pixel back into straight-alpha RGBA channels
    /// (`src.len() == Self::BYTES_PER_PIXEL`). Alpha-free formats
    /// report `a = 255`; premultiplied formats unpremultiply
    /// (`a = 0` decodes to all-zero channels).
    fn read_pixel(src: &[u8]) -> [u8; 4];
}

#[cfg(test)]
mod tests {
    use crate::Bgr;
    use crate::Bgra;
    use crate::BgraPremultiplied;
    use crate::Bgrx;
    use crate::Gray;
    use crate::PixelFormat;
    use crate::Rgb;
    use crate::Rgba;
    use crate::RgbaPremultiplied;
    use crate::Rgbx;

    fn write<F: PixelFormat>(r: u8, g: u8, b: u8, a: u8) -> Vec<u8> {
        let mut dst = vec![0; F::BYTES_PER_PIXEL];
        F::write_pixel(&mut dst, r, g, b, a);
        dst
    }

    #[test]
    fn rgba_roundtrips_channels() {
        let px = write::<Rgba>(10, 20, 30, 40);
        assert_eq!(px, [10, 20, 30, 40]);
        assert_eq!(Rgba::read_pixel(&px), [10, 20, 30, 40]);
    }

    #[test]
    fn bgra_swaps_red_and_blue() {
        let px = write::<Bgra>(10, 20, 30, 40);
        assert_eq!(px, [30, 20, 10, 40]);
        assert_eq!(Bgra::read_pixel(&px), [10, 20, 30, 40]);
    }

    #[test]
    fn rgba_premultiplied_multiplies_channels() {
        let px = write::<RgbaPremultiplied>(200, 100, 50, 128);
        // 200·128/255 ≈ 100, 100·128/255 ≈ 50, 50·128/255 ≈ 25
        assert_eq!(px[3], 128);
        assert!((px[0] as i32 - 100).abs() <= 1);
        assert!((px[1] as i32 - 50).abs() <= 1);
        assert!((px[2] as i32 - 25).abs() <= 1);
        let [r, _, _, a] = RgbaPremultiplied::read_pixel(&px);
        assert_eq!(a, 128);
        assert!((r as i32 - 200).abs() <= 2, "unpremultiplied red should recover ~200, got {r}");
    }

    #[test]
    fn bgra_premultiplied_zero_alpha_decodes_to_zero() {
        let px = write::<BgraPremultiplied>(200, 100, 50, 0);
        assert_eq!(px, [0, 0, 0, 0]);
        assert_eq!(BgraPremultiplied::read_pixel(&px), [0, 0, 0, 0]);
    }

    #[test]
    fn rgb_drops_alpha() {
        let px = write::<Rgb>(10, 20, 30, 40);
        assert_eq!(px, [10, 20, 30]);
        assert_eq!(Rgb::read_pixel(&px), [10, 20, 30, 255]);
    }

    #[test]
    fn bgr_drops_alpha_and_swaps() {
        let px = write::<Bgr>(10, 20, 30, 40);
        assert_eq!(px, [30, 20, 10]);
        assert_eq!(Bgr::read_pixel(&px), [10, 20, 30, 255]);
    }

    #[test]
    fn rgbx_writes_opaque_padding() {
        let px = write::<Rgbx>(10, 20, 30, 40);
        assert_eq!(px, [10, 20, 30, 255]);
        assert_eq!(Rgbx::read_pixel(&px), [10, 20, 30, 255]);
    }

    #[test]
    fn bgrx_writes_opaque_padding_swapped() {
        let px = write::<Bgrx>(10, 20, 30, 40);
        assert_eq!(px, [30, 20, 10, 255]);
        assert_eq!(Bgrx::read_pixel(&px), [10, 20, 30, 255]);
    }

    #[test]
    fn gray_reduces_to_luma() {
        // White → 255; pure red → 0.299·255 ≈ 76
        assert_eq!(write::<Gray>(255, 255, 255, 0), [255]);
        let red = write::<Gray>(255, 0, 0, 255);
        assert!((red[0] as i32 - 76).abs() <= 1, "Rec.601 red luma should be ~76, got {}", red[0]);
        assert_eq!(Gray::read_pixel(&red), [red[0], red[0], red[0], 255]);
    }
}
