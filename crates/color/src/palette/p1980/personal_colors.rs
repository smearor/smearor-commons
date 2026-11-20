use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Personal Colors" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum PersonalColorsColor {
        /// PANTONE 19-1220 TCX `Cappuccino`.
        Cappuccino => "cappuccino",
        /// PANTONE 19-3138 TCX `Byzantium`.
        Byzantium => "byzantium",
        /// PANTONE 14-0826 TCX `Pampas`.
        Pampas => "pampas",
        /// PANTONE 18-1631 TCX `Earth Red`.
        EarthRed => "earth-red",
        /// PANTONE 16-5815 TCX `Feldspar`.
        Feldspar => "feldspar",
        /// PANTONE 18-4530 TCX `Celestial`.
        Celestial => "celestial",
        /// PANTONE 16-1330 TCX `Muted Clay`.
        MutedClay => "muted-clay",
        /// PANTONE 13-1114 TCX `Bellini`.
        Bellini => "bellini"
    }
}

/// The sRGB values of the [`PersonalColorsColor`] slots.
pub static PERSONAL_COLORS: StaticPalette<PersonalColorsColor, u8> = static_palette! {
    PersonalColorsColor::Cappuccino => PantoneFhiNamedColor::Cappuccino.to_color(),
    PersonalColorsColor::Byzantium => PantoneFhiNamedColor::Byzantium.to_color(),
    PersonalColorsColor::Pampas => PantoneFhiNamedColor::Pampas.to_color(),
    PersonalColorsColor::EarthRed => PantoneFhiNamedColor::EarthRed.to_color(),
    PersonalColorsColor::Feldspar => PantoneFhiNamedColor::Feldspar.to_color(),
    PersonalColorsColor::Celestial => PantoneFhiNamedColor::Celestial.to_color(),
    PersonalColorsColor::MutedClay => PantoneFhiNamedColor::MutedClay.to_color(),
    PersonalColorsColor::Bellini => PantoneFhiNamedColor::Bellini.to_color(),
};
