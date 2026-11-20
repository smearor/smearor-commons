use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Cocktails and Laughter" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CocktailsAndLaughterColor {
        /// PANTONE 19-2118 TCX `Winetasting`.
        Winetasting => "winetasting",
        /// PANTONE 17-1540 TCX `Apricot Brandy`.
        ApricotBrandy => "apricot-brandy",
        /// PANTONE 14-1316 TCX `Dusty Pink`.
        DustyPink => "dusty-pink",
        /// PANTONE 17-1927 TCX `Desert Rose`.
        DesertRose => "desert-rose",
        /// PANTONE 17-0625 TCX `Boa`.
        Boa => "boa",
        /// PANTONE 15-0927 TCX `Pale Gold`.
        PaleGold => "pale-gold",
        /// PANTONE 17-4015 TCX `Infinity`.
        Infinity => "infinity"
    }
}

/// The sRGB values of the [`CocktailsAndLaughterColor`] slots.
pub static COCKTAILS_AND_LAUGHTER: StaticPalette<CocktailsAndLaughterColor, u8> = static_palette! {
    CocktailsAndLaughterColor::Winetasting => PantoneFhiNamedColor::Winetasting.to_color(),
    CocktailsAndLaughterColor::ApricotBrandy => PantoneFhiNamedColor::ApricotBrandy.to_color(),
    CocktailsAndLaughterColor::DustyPink => PantoneFhiNamedColor::DustyPink.to_color(),
    CocktailsAndLaughterColor::DesertRose => PantoneFhiNamedColor::DesertRose.to_color(),
    CocktailsAndLaughterColor::Boa => PantoneFhiNamedColor::Boa.to_color(),
    CocktailsAndLaughterColor::PaleGold => PantoneFhiNamedColor::PaleGold.to_color(),
    CocktailsAndLaughterColor::Infinity => PantoneFhiNamedColor::Infinity.to_color(),
};
