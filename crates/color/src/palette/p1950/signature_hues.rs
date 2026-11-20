use crate::Color;
use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1950s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1950Color {
        /// The lead swatch of the "Happy at Home" palette - PANTONE 12-0736 TCX `Lemon Drop`.
        HappyAtHome => "happy-at-home",
        /// The lead swatch of the "Teen Angels" palette - Pantone 7471 C.
        TeenAngels => "teen-angels",
        /// The lead swatch of the "Mid-Century Modernists" palette - PANTONE 14-0848 TCX `Mimosa`.
        MidCenturyModernists => "mid-century-modernists",
        /// The lead swatch of the "Movie Goddesses" palette - Pantone 9286 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        MovieGoddesses => "movie-goddesses",
        /// The lead swatch of the "Cosmetic Superstars" palette - PANTONE 11-1404 TCX `Powder Puff`.
        CosmeticSuperstars => "cosmetic-superstars",
        /// The lead swatch of the "Coast-to-Coast Woolens" palette - PANTONE 19-1759 TCX `American Beauty`.
        CoastToCoastWoolens => "coast-to-coast-woolens",
        /// The lead swatch of the "Fantasyland" palette - Pantone 211 C.
        Fantasyland => "fantasyland",
        /// The lead swatch of the "Abstract Expressionists" palette - Pantone 7579 C.
        AbstractExpressionists => "abstract-expressionists"
    }
}

/// The sRGB values of the [`SignatureHues1950Color`] slots.
pub static SIGNATURE_HUES_1950: StaticPalette<SignatureHues1950Color, u8> = static_palette! {
    SignatureHues1950Color::HappyAtHome => PantoneFhiNamedColor::LemonDrop.to_color(),
    SignatureHues1950Color::TeenAngels => PantoneNamedColor::Pms7471C.to_color(),
    SignatureHues1950Color::MidCenturyModernists => PantoneFhiNamedColor::Mimosa.to_color(),
    SignatureHues1950Color::MovieGoddesses => Color::from_rgb_u32(0xFCDCDD),
    SignatureHues1950Color::CosmeticSuperstars => PantoneFhiNamedColor::PowderPuff.to_color(),
    SignatureHues1950Color::CoastToCoastWoolens => PantoneFhiNamedColor::AmericanBeauty.to_color(),
    SignatureHues1950Color::Fantasyland => PantoneNamedColor::Pms211C.to_color(),
    SignatureHues1950Color::AbstractExpressionists => PantoneNamedColor::Pms7579C.to_color(),
};
