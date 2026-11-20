use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Hit Parade" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum HitParadeColor {
        /// Pantone 3005 C.
        Pms3005 => "pms-3005",
        /// Pantone 715 C.
        Pms715 => "pms-715",
        /// Pantone 7566 C.
        Pms7566 => "pms-7566",
        /// Pantone Black C.
        Black => "black",
        /// Pantone White C.
        White => "white",
        /// Pantone 7621 C.
        Pms7621 => "pms-7621",
        /// Pantone 4725 C.
        Pms4725 => "pms-4725",
        /// Pantone 476 C.
        Pms476 => "pms-476"
    }
}

/// The sRGB values of the [`HitParadeColor`] slots.
pub static HIT_PARADE: StaticPalette<HitParadeColor, u8> = static_palette! {
    HitParadeColor::Pms3005 => PantoneNamedColor::Pms3005C.to_color(),
    HitParadeColor::Pms715 => PantoneNamedColor::Pms715C.to_color(),
    HitParadeColor::Pms7566 => PantoneNamedColor::Pms7566C.to_color(),
    HitParadeColor::Black => PantoneNamedColor::BlackC.to_color(),
    HitParadeColor::White => PantoneNamedColor::WhiteC.to_color(),
    HitParadeColor::Pms7621 => PantoneNamedColor::Pms7621C.to_color(),
    HitParadeColor::Pms4725 => PantoneNamedColor::Pms4725C.to_color(),
    HitParadeColor::Pms476 => PantoneNamedColor::Pms476C.to_color(),
};
