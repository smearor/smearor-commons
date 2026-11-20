use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Conspicuous Consumption" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ConspicuousConsumptionColor {
        /// PANTONE 14-5002 TCX `Silver`.
        Silver => "silver",
        /// PANTONE 15-0927 TCX `Pale Gold`.
        PaleGold => "pale-gold",
        /// PANTONE 14-1012 TCX `Gilded Beige`.
        GildedBeige => "gilded-beige",
        /// PANTONE 19-1012 TCX `French Roast`.
        FrenchRoast => "french-roast",
        /// PANTONE 19-4005 TCX `Stretch Limo`.
        StretchLimo => "stretch-limo",
        /// PANTONE 11-4800 TCX `Blanc De Blanc`.
        BlancDeBlanc => "blanc-de-blanc"
    }
}

/// The sRGB values of the [`ConspicuousConsumptionColor`] slots.
pub static CONSPICUOUS_CONSUMPTION: StaticPalette<ConspicuousConsumptionColor, u8> = static_palette! {
    ConspicuousConsumptionColor::Silver => PantoneFhiNamedColor::Silver.to_color(),
    ConspicuousConsumptionColor::PaleGold => PantoneFhiNamedColor::PaleGold.to_color(),
    ConspicuousConsumptionColor::GildedBeige => PantoneFhiNamedColor::GildedBeige.to_color(),
    ConspicuousConsumptionColor::FrenchRoast => PantoneFhiNamedColor::FrenchRoast.to_color(),
    ConspicuousConsumptionColor::StretchLimo => PantoneFhiNamedColor::StretchLimo.to_color(),
    ConspicuousConsumptionColor::BlancDeBlanc => PantoneFhiNamedColor::BlancDeBlanc.to_color(),
};
