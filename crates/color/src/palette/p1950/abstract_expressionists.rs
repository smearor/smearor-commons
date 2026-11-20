use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Abstract Expressionists" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum AbstractExpressionistsColor {
        /// Pantone 7579 C.
        Pms7579 => "pms-7579",
        /// Pantone 632 C.
        Pms632 => "pms-632",
        /// Pantone 129 C.
        Pms129 => "pms-129",
        /// Pantone 2718 C.
        Pms2718 => "pms-2718",
        /// Pantone Warm Gray 1 C.
        WarmGray1 => "warm-gray-1",
        /// Pantone Black 7 C.
        Black7 => "black-7",
        /// Pantone 7654 C.
        Pms7654 => "pms-7654"
    }
}

/// The sRGB values of the [`AbstractExpressionistsColor`] slots.
pub static ABSTRACT_EXPRESSIONISTS: StaticPalette<AbstractExpressionistsColor, u8> = static_palette! {
    AbstractExpressionistsColor::Pms7579 => PantoneNamedColor::Pms7579C.to_color(),
    AbstractExpressionistsColor::Pms632 => PantoneNamedColor::Pms632C.to_color(),
    AbstractExpressionistsColor::Pms129 => PantoneNamedColor::Pms129C.to_color(),
    AbstractExpressionistsColor::Pms2718 => PantoneNamedColor::Pms2718C.to_color(),
    AbstractExpressionistsColor::WarmGray1 => PantoneNamedColor::WarmGray1C.to_color(),
    AbstractExpressionistsColor::Black7 => PantoneNamedColor::Black7C.to_color(),
    AbstractExpressionistsColor::Pms7654 => PantoneNamedColor::Pms7654C.to_color(),
};
