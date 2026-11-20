use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Parks and Recreation" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ParksAndRecreationColor {
        /// Pantone Black 6 C.
        Black6 => "black-6",
        /// Pantone 7553 C.
        Pms7553 => "pms-7553",
        /// Pantone 7760 C.
        Pms7760 => "pms-7760",
        /// Pantone 465 C.
        Pms465 => "pms-465",
        /// Pantone 7698 C.
        Pms7698 => "pms-7698",
        /// Pantone 7660 C.
        Pms7660 => "pms-7660",
        /// Pantone 7639 C.
        Pms7639 => "pms-7639"
    }
}

/// The sRGB values of the [`ParksAndRecreationColor`] slots.
pub static PARKS_AND_RECREATION: StaticPalette<ParksAndRecreationColor, u8> = static_palette! {
    ParksAndRecreationColor::Black6 => PantoneNamedColor::Black6C.to_color(),
    ParksAndRecreationColor::Pms7553 => PantoneNamedColor::Pms7553C.to_color(),
    ParksAndRecreationColor::Pms7760 => PantoneNamedColor::Pms7760C.to_color(),
    ParksAndRecreationColor::Pms465 => PantoneNamedColor::Pms465C.to_color(),
    ParksAndRecreationColor::Pms7698 => PantoneNamedColor::Pms7698C.to_color(),
    ParksAndRecreationColor::Pms7660 => PantoneNamedColor::Pms7660C.to_color(),
    ParksAndRecreationColor::Pms7639 => PantoneNamedColor::Pms7639C.to_color(),
};
