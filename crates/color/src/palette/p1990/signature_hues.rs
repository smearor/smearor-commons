use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1990s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1990Color {
        /// The lead swatch of the "Grunge and Graffiti" palette - PANTONE 18-3718 TCX `Purple Haze`.
        GrungeAndGraffiti => "grunge-and-graffiti",
        /// The lead swatch of the "It's a Good Thing" palette - PANTONE 19-2924 TCX `Hollyhock`. Caption spells `Holyhock`.
        ItsAGoodThing => "its-a-good-thing",
        /// The lead swatch of the "The Nature of Zen" palette - PANTONE 14-0627 TCX `Shadow Green`.
        NatureOfZen => "nature-of-zen",
        /// The lead swatch of the "Out of Africa" palette - PANTONE 18-1454 TCX `Red Clay`.
        OutOfAfrica => "out-of-africa",
        /// The lead swatch of the "Latin Flavors" palette - Pantone 136 C.
        LatinFlavors => "latin-flavors",
        /// The lead swatch of the "Chic over Geek" palette - Pantone 1505 C.
        ChicOverGeek => "chic-over-geek",
        /// The lead swatch of the "Anime" palette - Pantone 230 C.
        Anime => "anime",
        /// The lead swatch of the "Conspicuous Consumption" palette - PANTONE 14-5002 TCX `Silver`.
        ConspicuousConsumption => "conspicuous-consumption",
        /// The lead swatch of the "Future Forecasts" palette - PANTONE 19-1655 TCX `Garnet`.
        FutureForecasts => "future-forecasts"
    }
}

/// The sRGB values of the [`SignatureHues1990Color`] slots.
pub static SIGNATURE_HUES_1990: StaticPalette<SignatureHues1990Color, u8> = static_palette! {
    SignatureHues1990Color::GrungeAndGraffiti => PantoneFhiNamedColor::PurpleHaze.to_color(),
    SignatureHues1990Color::ItsAGoodThing => PantoneFhiNamedColor::Hollyhock.to_color(),
    SignatureHues1990Color::NatureOfZen => PantoneFhiNamedColor::ShadowGreen.to_color(),
    SignatureHues1990Color::OutOfAfrica => PantoneFhiNamedColor::RedClay.to_color(),
    SignatureHues1990Color::LatinFlavors => PantoneNamedColor::Pms136C.to_color(),
    SignatureHues1990Color::ChicOverGeek => PantoneNamedColor::Pms1505C.to_color(),
    SignatureHues1990Color::Anime => PantoneNamedColor::Pms230C.to_color(),
    SignatureHues1990Color::ConspicuousConsumption => PantoneFhiNamedColor::Silver.to_color(),
    SignatureHues1990Color::FutureForecasts => PantoneFhiNamedColor::Garnet.to_color(),
};
