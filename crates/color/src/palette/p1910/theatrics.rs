use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Theatrics" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum TheatricsColor {
        /// PANTONE 16-1255 TCX `Russet Orange`.
        RussetOrange => "russet-orange",
        /// PANTONE 17-1558 TCX `Grenadine`.
        Grenadine => "grenadine",
        /// PANTONE 18-3949 TCX `Dazzling Blue`.
        DazzlingBlue => "dazzling-blue",
        /// PANTONE 17-1522 TCX `Mauvewood`.
        Mauvewood => "mauvewood",
        /// PANTONE 18-4432 TCX `Turkish Tile`.
        TurkishTile => "turkish-tile",
        /// PANTONE 18-1629 TCX `Faded Rose`.
        FadedRose => "faded-rose",
        /// PANTONE 17-1147 TCX `Amber Brown`.
        AmberBrown => "amber-brown",
        /// PANTONE 12-0826 TCX `Golden Haze`.
        GoldenHaze => "golden-haze"
    }
}

/// The sRGB values of the [`TheatricsColor`] slots.
pub static THEATRICS: StaticPalette<TheatricsColor, u8> = static_palette! {
    TheatricsColor::RussetOrange => PantoneFhiNamedColor::RussetOrange.to_color(),
    TheatricsColor::Grenadine => PantoneFhiNamedColor::Grenadine.to_color(),
    TheatricsColor::DazzlingBlue => PantoneFhiNamedColor::DazzlingBlue.to_color(),
    TheatricsColor::Mauvewood => PantoneFhiNamedColor::Mauvewood.to_color(),
    TheatricsColor::TurkishTile => PantoneFhiNamedColor::TurkishTile.to_color(),
    TheatricsColor::FadedRose => PantoneFhiNamedColor::FadedRose.to_color(),
    TheatricsColor::AmberBrown => PantoneFhiNamedColor::AmberBrown.to_color(),
    TheatricsColor::GoldenHaze => PantoneFhiNamedColor::GoldenHaze.to_color(),
};
