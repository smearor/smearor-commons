use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Cosmetic Superstars" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CosmeticSuperstarsColor {
        /// PANTONE 11-1404 TCX `Powder Puff`.
        PowderPuff => "powder-puff",
        /// PANTONE 14-1511 TCX `Powder Pink`.
        PowderPink => "powder-pink",
        /// PANTONE 15-1614 TCX `Blush`.
        Blush => "blush",
        /// PANTONE 17-1740 TCX `Claret Red`.
        ClaretRed => "claret-red",
        /// PANTONE 18-1755 TCX `Rouge Red`.
        RougeRed => "rouge-red",
        /// PANTONE 19-1764 TCX `Lipstick Red`.
        LipstickRed => "lipstick-red"
    }
}

/// The sRGB values of the [`CosmeticSuperstarsColor`] slots.
pub static COSMETIC_SUPERSTARS: StaticPalette<CosmeticSuperstarsColor, u8> = static_palette! {
    CosmeticSuperstarsColor::PowderPuff => PantoneFhiNamedColor::PowderPuff.to_color(),
    CosmeticSuperstarsColor::PowderPink => PantoneFhiNamedColor::PowderPink.to_color(),
    CosmeticSuperstarsColor::Blush => PantoneFhiNamedColor::Blush.to_color(),
    CosmeticSuperstarsColor::ClaretRed => PantoneFhiNamedColor::ClaretRed.to_color(),
    CosmeticSuperstarsColor::RougeRed => PantoneFhiNamedColor::RougeRed.to_color(),
    CosmeticSuperstarsColor::LipstickRed => PantoneFhiNamedColor::LipstickRed.to_color(),
};
