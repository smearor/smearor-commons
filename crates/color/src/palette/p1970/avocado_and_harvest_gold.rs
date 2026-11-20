use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Avocado and Harvest Gold" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum AvocadoAndHarvestGoldColor {
        /// PANTONE 18-0430 TCX `Avocado`.
        Avocado => "avocado",
        /// PANTONE 16-0639 TCX `Golden Olive`.
        GoldenOlive => "golden-olive",
        /// PANTONE 13-0739 TCX `Cream Gold`.
        CreamGold => "cream-gold",
        /// PANTONE 16-0948 TCX `Harvest Gold`.
        HarvestGold => "harvest-gold",
        /// PANTONE 16-1448 TCX `Burnt Orange`.
        BurntOrange => "burnt-orange",
        /// PANTONE 19-1241 TCX `Tortoise Shell`.
        TortoiseShell => "tortoise-shell"
    }
}

/// The sRGB values of the [`AvocadoAndHarvestGoldColor`] slots.
pub static AVOCADO_AND_HARVEST_GOLD: StaticPalette<AvocadoAndHarvestGoldColor, u8> = static_palette! {
    AvocadoAndHarvestGoldColor::Avocado => PantoneFhiNamedColor::Avocado.to_color(),
    AvocadoAndHarvestGoldColor::GoldenOlive => PantoneFhiNamedColor::GoldenOlive.to_color(),
    AvocadoAndHarvestGoldColor::CreamGold => PantoneFhiNamedColor::CreamGold.to_color(),
    AvocadoAndHarvestGoldColor::HarvestGold => PantoneFhiNamedColor::HarvestGold.to_color(),
    AvocadoAndHarvestGoldColor::BurntOrange => PantoneFhiNamedColor::BurntOrange.to_color(),
    AvocadoAndHarvestGoldColor::TortoiseShell => PantoneFhiNamedColor::TortoiseShell.to_color(),
};
