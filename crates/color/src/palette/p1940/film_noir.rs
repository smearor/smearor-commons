use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Film Noir" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FilmNoirColor {
        /// PANTONE 13-0002 TCX `White Sand`.
        WhiteSand => "white-sand",
        /// PANTONE 16-3802 TCX `Ash`.
        Ash => "ash",
        /// PANTONE 16-3803 TCX `Gull Gray`.
        GullGray => "gull-gray",
        /// PANTONE 19-0000 TCX `Raven`.
        Raven => "raven",
        /// PANTONE 18-1763 TCX `High Risk Red`.
        HighRiskRed => "high-risk-red"
    }
}

/// The sRGB values of the [`FilmNoirColor`] slots.
pub static FILM_NOIR: StaticPalette<FilmNoirColor, u8> = static_palette! {
    FilmNoirColor::WhiteSand => PantoneFhiNamedColor::WhiteSand.to_color(),
    FilmNoirColor::Ash => PantoneFhiNamedColor::Ash.to_color(),
    FilmNoirColor::GullGray => PantoneFhiNamedColor::GullGray.to_color(),
    FilmNoirColor::Raven => PantoneFhiNamedColor::Raven.to_color(),
    FilmNoirColor::HighRiskRed => PantoneFhiNamedColor::HighRiskRed.to_color(),
};
