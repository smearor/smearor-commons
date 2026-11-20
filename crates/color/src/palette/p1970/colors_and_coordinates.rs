use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Colors and Coordinates" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ColorsAndCoordinatesColor {
        /// Pantone 7752 C.
        Pms7752 => "pms-7752",
        /// Pantone 157 C.
        Pms157 => "pms-157",
        /// Pantone 7465 C.
        Pms7465 => "pms-7465",
        /// Pantone 2718 C.
        Pms2718 => "pms-2718",
        /// Pantone 121 C.
        Pms121 => "pms-121",
        /// Pantone 211 C.
        Pms211 => "pms-211",
        /// Pantone 7416 C.
        Pms7416 => "pms-7416",
        /// Pantone 644 C.
        Pms644 => "pms-644"
    }
}

/// The sRGB values of the [`ColorsAndCoordinatesColor`] slots.
pub static COLORS_AND_COORDINATES: StaticPalette<ColorsAndCoordinatesColor, u8> = static_palette! {
    ColorsAndCoordinatesColor::Pms7752 => PantoneNamedColor::Pms7752C.to_color(),
    ColorsAndCoordinatesColor::Pms157 => PantoneNamedColor::Pms157C.to_color(),
    ColorsAndCoordinatesColor::Pms7465 => PantoneNamedColor::Pms7465C.to_color(),
    ColorsAndCoordinatesColor::Pms2718 => PantoneNamedColor::Pms2718C.to_color(),
    ColorsAndCoordinatesColor::Pms121 => PantoneNamedColor::Pms121C.to_color(),
    ColorsAndCoordinatesColor::Pms211 => PantoneNamedColor::Pms211C.to_color(),
    ColorsAndCoordinatesColor::Pms7416 => PantoneNamedColor::Pms7416C.to_color(),
    ColorsAndCoordinatesColor::Pms644 => PantoneNamedColor::Pms644C.to_color(),
};
