use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Happy at Home" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum HappyAtHomeColor {
        /// PANTONE 12-0736 TCX `Lemon Drop`.
        LemonDrop => "lemon-drop",
        /// PANTONE 13-5309 TCX `Pastel Turquoise`.
        PastelTurquoise => "pastel-turquoise",
        /// PANTONE 14-4201 TCX `Lunar Rock`.
        LunarRock => "lunar-rock",
        /// PANTONE 15-0533 TCX `Linden Green`.
        LindenGreen => "linden-green",
        /// PANTONE 15-0332 TCX `Leaf Green`.
        LeafGreen => "leaf-green",
        /// PANTONE 18-5410 TCX `Silver Pine`.
        SilverPine => "silver-pine",
        /// PANTONE 18-1629 TCX `Faded Rose`.
        FadedRose => "faded-rose"
    }
}

/// The sRGB values of the [`HappyAtHomeColor`] slots.
pub static HAPPY_AT_HOME: StaticPalette<HappyAtHomeColor, u8> = static_palette! {
    HappyAtHomeColor::LemonDrop => PantoneFhiNamedColor::LemonDrop.to_color(),
    HappyAtHomeColor::PastelTurquoise => PantoneFhiNamedColor::PastelTurquoise.to_color(),
    HappyAtHomeColor::LunarRock => PantoneFhiNamedColor::LunarRock.to_color(),
    HappyAtHomeColor::LindenGreen => PantoneFhiNamedColor::LindenGreen.to_color(),
    HappyAtHomeColor::LeafGreen => PantoneFhiNamedColor::LeafGreen.to_color(),
    HappyAtHomeColor::SilverPine => PantoneFhiNamedColor::SilverPine.to_color(),
    HappyAtHomeColor::FadedRose => PantoneFhiNamedColor::FadedRose.to_color(),
};
