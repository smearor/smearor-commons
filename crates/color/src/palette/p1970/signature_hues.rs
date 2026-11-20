use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1970s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1970Color {
        /// The lead swatch of the "Colors and Coordinates" palette - Pantone 7752 C.
        ColorsAndCoordinates => "colors-and-coordinates",
        /// The lead swatch of the "Avocado and Harvest Gold" palette - PANTONE 18-0430 TCX `Avocado`.
        AvocadoAndHarvestGold => "avocado-and-harvest-gold",
        /// The lead swatch of the "Feathers and Leathers" palette - PANTONE 19-4329 TCX `Corsair`.
        FeathersAndLeathers => "feathers-and-leathers",
        /// The lead swatch of the "Provence" palette - PANTONE 16-4032 TCX `Provence`.
        Provence => "provence",
        /// The lead swatch of the "Land Art" palette - Pantone 284 C.
        LandArt => "land-art",
        /// The lead swatch of the "The Day the World Turned Day-Glo" palette - Pantone Process Blue C.
        DayTheWorldTurnedDayGlo => "day-the-world-turned-day-glo",
        /// The lead swatch of the "Night Life" palette - Pantone White C.
        NightLife => "night-life",
        /// The lead swatch of the "Hotel California" palette - Pantone 3005 C.
        HotelCalifornia => "hotel-california"
    }
}

/// The sRGB values of the [`SignatureHues1970Color`] slots.
pub static SIGNATURE_HUES_1970: StaticPalette<SignatureHues1970Color, u8> = static_palette! {
    SignatureHues1970Color::ColorsAndCoordinates => PantoneNamedColor::Pms7752C.to_color(),
    SignatureHues1970Color::AvocadoAndHarvestGold => PantoneFhiNamedColor::Avocado.to_color(),
    SignatureHues1970Color::FeathersAndLeathers => PantoneFhiNamedColor::Corsair.to_color(),
    SignatureHues1970Color::Provence => PantoneFhiNamedColor::Provence.to_color(),
    SignatureHues1970Color::LandArt => PantoneNamedColor::Pms284C.to_color(),
    SignatureHues1970Color::DayTheWorldTurnedDayGlo => PantoneNamedColor::ProcessBlueC.to_color(),
    SignatureHues1970Color::NightLife => PantoneNamedColor::WhiteC.to_color(),
    SignatureHues1970Color::HotelCalifornia => PantoneNamedColor::Pms3005C.to_color(),
};
