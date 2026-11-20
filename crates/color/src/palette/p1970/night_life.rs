use crate::Color;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Night Life" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum NightLifeColor {
        /// Pantone White C.
        White => "white",
        /// Pantone 10103 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10103 => "pms-10103",
        /// Pantone 212 C.
        Pms212 => "pms-212",
        /// Pantone Warm Red C.
        WarmRed => "warm-red",
        /// Pantone 129 C.
        Pms129 => "pms-129",
        /// Pantone 301 C.
        Pms301 => "pms-301",
        /// Pantone Black 3 C.
        Black3 => "black-3"
    }
}

/// The sRGB values of the [`NightLifeColor`] slots.
pub static NIGHT_LIFE: StaticPalette<NightLifeColor, u8> = static_palette! {
    NightLifeColor::White => PantoneNamedColor::WhiteC.to_color(),
    NightLifeColor::Pms10103 => Color::from_rgb_u32(0xC0C1C3),
    NightLifeColor::Pms212 => PantoneNamedColor::Pms212C.to_color(),
    NightLifeColor::WarmRed => PantoneNamedColor::WarmRedC.to_color(),
    NightLifeColor::Pms129 => PantoneNamedColor::Pms129C.to_color(),
    NightLifeColor::Pms301 => PantoneNamedColor::Pms301C.to_color(),
    NightLifeColor::Black3 => PantoneNamedColor::Black3C.to_color(),
};
