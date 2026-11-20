use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Leyendecker Man" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum LeyendeckerManColor {
        /// PANTONE 16-1331 TCX `Toast`.
        Toast => "toast",
        /// PANTONE 12-0806 TCX `Rutabaga`.
        Rutabaga => "rutabaga",
        /// PANTONE 19-4057 TCX `True Blue`.
        TrueBlue => "true-blue",
        /// PANTONE 19-0508 TCX `Peat`.
        Peat => "peat",
        /// PANTONE 16-1451 TCX `Nasturtium`.
        Nasturtium => "nasturtium",
        /// PANTONE 19-2014 TCX `Prune`.
        Prune => "prune"
    }
}

/// The sRGB values of the [`LeyendeckerManColor`] slots.
pub static LEYENDECKER_MAN: StaticPalette<LeyendeckerManColor, u8> = static_palette! {
    LeyendeckerManColor::Toast => PantoneFhiNamedColor::Toast.to_color(),
    LeyendeckerManColor::Rutabaga => PantoneFhiNamedColor::Rutabaga.to_color(),
    LeyendeckerManColor::TrueBlue => PantoneFhiNamedColor::TrueBlue.to_color(),
    LeyendeckerManColor::Peat => PantoneFhiNamedColor::Peat.to_color(),
    LeyendeckerManColor::Nasturtium => PantoneFhiNamedColor::Nasturtium.to_color(),
    LeyendeckerManColor::Prune => PantoneFhiNamedColor::Prune.to_color(),
};
