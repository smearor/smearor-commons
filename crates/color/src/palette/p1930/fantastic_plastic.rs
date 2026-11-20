use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Fantastic Plastic" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FantasticPlasticColor {
        /// PANTONE 19-4305 TCX `Pirate Black`.
        PirateBlack => "pirate-black",
        /// PANTONE 14-0847 TCX `Buff Yellow`.
        BuffYellow => "buff-yellow",
        /// PANTONE 15-1058 TCX `Radiant Yellow`. Caption spells `Radient Yellow`.
        RadiantYellow => "radiant-yellow",
        /// PANTONE 17-1558 TCX `Grenadine`.
        Grenadine => "grenadine",
        /// PANTONE 17-1464 TCX `Red Orange`.
        RedOrange => "red-orange",
        /// PANTONE 18-6320 TCX `Fairway`.
        Fairway => "fairway",
        /// PANTONE 19-1230 TCX `Friar Brown`.
        FriarBrown => "friar-brown",
        /// PANTONE 19-3022 TCX `Gloxinia`.
        Gloxinia => "gloxinia"
    }
}

/// The sRGB values of the [`FantasticPlasticColor`] slots.
pub static FANTASTIC_PLASTIC: StaticPalette<FantasticPlasticColor, u8> = static_palette! {
    FantasticPlasticColor::PirateBlack => PantoneFhiNamedColor::PirateBlack.to_color(),
    FantasticPlasticColor::BuffYellow => PantoneFhiNamedColor::BuffYellow.to_color(),
    FantasticPlasticColor::RadiantYellow => PantoneFhiNamedColor::RadiantYellow.to_color(),
    FantasticPlasticColor::Grenadine => PantoneFhiNamedColor::Grenadine.to_color(),
    FantasticPlasticColor::RedOrange => PantoneFhiNamedColor::RedOrange.to_color(),
    FantasticPlasticColor::Fairway => PantoneFhiNamedColor::Fairway.to_color(),
    FantasticPlasticColor::FriarBrown => PantoneFhiNamedColor::FriarBrown.to_color(),
    FantasticPlasticColor::Gloxinia => PantoneFhiNamedColor::Gloxinia.to_color(),
};
