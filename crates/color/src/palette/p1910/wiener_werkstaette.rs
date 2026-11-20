use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Wiener Werkstätte" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WienerWerkstaetteColor {
        /// PANTONE 19-4203 TCX `Moonless Night`.
        MoonlessNight => "moonless-night",
        /// PANTONE 19-1521 TCX `Red Mahogany`.
        RedMahogany => "red-mahogany",
        /// PANTONE 18-4225 TCX `Saxony Blue`.
        SaxonyBlue => "saxony-blue",
        /// PANTONE 18-3949 TCX `Dazzling Blue`.
        DazzlingBlue => "dazzling-blue",
        /// PANTONE 16-5815 TCX `Feldspar`.
        Feldspar => "feldspar",
        /// PANTONE 18-1540 TCX `Cinnabar`.
        Cinnabar => "cinnabar",
        /// PANTONE 17-3910 TCX `Lavender Gray`.
        LavenderGray => "lavender-gray",
        /// PANTONE 13-0739 TCX `Cream Gold`.
        CreamGold => "cream-gold",
        /// PANTONE 13-1405 TCX `Shell`.
        Shell => "shell",
        /// PANTONE 14-5002 TCX `Silver`.
        Silver => "silver"
    }
}

/// The sRGB values of the [`WienerWerkstaetteColor`] slots.
pub static WIENER_WERKSTAETTE: StaticPalette<WienerWerkstaetteColor, u8> = static_palette! {
    WienerWerkstaetteColor::MoonlessNight => PantoneFhiNamedColor::MoonlessNight.to_color(),
    WienerWerkstaetteColor::RedMahogany => PantoneFhiNamedColor::RedMahogany.to_color(),
    WienerWerkstaetteColor::SaxonyBlue => PantoneFhiNamedColor::SaxonyBlue.to_color(),
    WienerWerkstaetteColor::DazzlingBlue => PantoneFhiNamedColor::DazzlingBlue.to_color(),
    WienerWerkstaetteColor::Feldspar => PantoneFhiNamedColor::Feldspar.to_color(),
    WienerWerkstaetteColor::Cinnabar => PantoneFhiNamedColor::Cinnabar.to_color(),
    WienerWerkstaetteColor::LavenderGray => PantoneFhiNamedColor::LavenderGray.to_color(),
    WienerWerkstaetteColor::CreamGold => PantoneFhiNamedColor::CreamGold.to_color(),
    WienerWerkstaetteColor::Shell => PantoneFhiNamedColor::Shell.to_color(),
    WienerWerkstaetteColor::Silver => PantoneFhiNamedColor::Silver.to_color(),
};
