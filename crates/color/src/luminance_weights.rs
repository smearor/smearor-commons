/// RGB channel weights for a perceived-luminance standard.
///
/// The luminance of a color is the weighted sum `r*w_r + g*w_g + b*w_b`.
/// The weights of a standard sum to 1.0; [`crate::Color::luminance`]
/// accepts any custom triple, the result is clamped to `0.0-1.0`
/// regardless.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LuminanceWeights {
    /// Weight of the red channel.
    pub r: f64,
    /// Weight of the green channel.
    pub g: f64,
    /// Weight of the blue channel.
    pub b: f64,
}

impl LuminanceWeights {
    /// ITU-R BT.709 weights (HDTV, sRGB default).
    pub const REC_709: Self = Self {
        r: 0.2126,
        g: 0.7152,
        b: 0.0722,
    };

    /// ITU-R BT.601 weights (SDTV, JPEG/MPEG luma).
    pub const REC_601: Self = Self { r: 0.299, g: 0.587, b: 0.114 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_weights_sum_to_one() {
        for weights in [LuminanceWeights::REC_709, LuminanceWeights::REC_601] {
            assert!((weights.r + weights.g + weights.b - 1.0).abs() < 1e-9);
        }
    }
}
