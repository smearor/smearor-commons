use crate::Color;
use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1930s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1930Color {
        /// The lead swatch of the "Deco Architecture" palette - Pantone `Silver` metallic chip (closest catalog color is Pantone 877 C); not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        DecoArchitecture => "deco-architecture",
        /// The lead swatch of the "Illusions" palette - PANTONE 18-0000 TCX `Smoked Pearl`.
        Illusions => "illusions",
        /// The lead swatch of the "Fantastic Plastic" palette - PANTONE 19-4305 TCX `Pirate Black`.
        FantasticPlastic => "fantastic-plastic",
        /// The lead swatch of the "Diversions" palette - Pantone 5473 C.
        Diversions => "diversions",
        /// The lead swatch of the "Parks and Recreation" palette - Pantone Black 6 C.
        ParksAndRecreation => "parks-and-recreation",
        /// The lead swatch of the "Roseville" palette - PANTONE 17-0215 TCX `Aspen Green`.
        Roseville => "roseville",
        /// The lead swatch of the "The Wizard of Oz" palette - PANTONE 14-4121 TCX `Blue Bell`.
        WizardOfOz => "wizard-of-oz",
        /// The lead swatch of the "The World of Tomorrow" palette - PANTONE 19-3847 TCX `Deep Blue`.
        WorldOfTomorrow => "world-of-tomorrow"
    }
}

/// The sRGB values of the [`SignatureHues1930Color`] slots.
pub static SIGNATURE_HUES_1930: StaticPalette<SignatureHues1930Color, u8> = static_palette! {
    SignatureHues1930Color::DecoArchitecture => Color::from_rgb_u32(0xABAFB0),
    SignatureHues1930Color::Illusions => PantoneFhiNamedColor::SmokedPearl.to_color(),
    SignatureHues1930Color::FantasticPlastic => PantoneFhiNamedColor::PirateBlack.to_color(),
    SignatureHues1930Color::Diversions => PantoneNamedColor::Pms5473C.to_color(),
    SignatureHues1930Color::ParksAndRecreation => PantoneNamedColor::Black6C.to_color(),
    SignatureHues1930Color::Roseville => PantoneFhiNamedColor::AspenGreen.to_color(),
    SignatureHues1930Color::WizardOfOz => PantoneFhiNamedColor::BlueBell.to_color(),
    SignatureHues1930Color::WorldOfTomorrow => PantoneFhiNamedColor::DeepBlue.to_color(),
};
