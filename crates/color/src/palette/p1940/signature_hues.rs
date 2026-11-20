use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1940s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1940Color {
        /// The lead swatch of the "Fantasia" palette - PANTONE 17-1663 TCX `Bittersweet`.
        Fantasia => "fantasia",
        /// The lead swatch of the "Edward Hopper" palette - Pantone 7477 C.
        EdwardHopper => "edward-hopper",
        /// The lead swatch of the "World War II" palette - PANTONE 16-1334 TCX `Tan`.
        WorldWar2 => "world-war-2",
        /// The lead swatch of the "Carefree and Casual" palette - PANTONE 18-5621 TCX `Fir`.
        CarefreeAndCasual => "carefree-and-casual",
        /// The lead swatch of the "The American Dream" palette - PANTONE 14-1230 TCX `Apricot Wash`.
        AmericanDream => "american-dream",
        /// The lead swatch of the "Hit Parade" palette - Pantone 3005 C.
        HitParade => "hit-parade",
        /// The lead swatch of the "Film Noir" palette - PANTONE 13-0002 TCX `White Sand`.
        FilmNoir => "film-noir",
        /// The lead swatch of the "La Mode" palette - PANTONE 19-4006 TCX `Caviar`.
        LaMode => "la-mode"
    }
}

/// The sRGB values of the [`SignatureHues1940Color`] slots.
pub static SIGNATURE_HUES_1940: StaticPalette<SignatureHues1940Color, u8> = static_palette! {
    SignatureHues1940Color::Fantasia => PantoneFhiNamedColor::Bittersweet.to_color(),
    SignatureHues1940Color::EdwardHopper => PantoneNamedColor::Pms7477C.to_color(),
    SignatureHues1940Color::WorldWar2 => PantoneFhiNamedColor::Tan.to_color(),
    SignatureHues1940Color::CarefreeAndCasual => PantoneFhiNamedColor::Fir.to_color(),
    SignatureHues1940Color::AmericanDream => PantoneFhiNamedColor::ApricotWash.to_color(),
    SignatureHues1940Color::HitParade => PantoneNamedColor::Pms3005C.to_color(),
    SignatureHues1940Color::FilmNoir => PantoneFhiNamedColor::WhiteSand.to_color(),
    SignatureHues1940Color::LaMode => PantoneFhiNamedColor::Caviar.to_color(),
};
