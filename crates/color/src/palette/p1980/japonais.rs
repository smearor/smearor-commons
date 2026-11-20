use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Japonais" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum JaponaisColor {
        /// PANTONE 19-4005 TCX `Stretch Limo`.
        StretchLimo => "stretch-limo",
        /// PANTONE 15-5704 TCX `Mineral Gray`.
        MineralGray => "mineral-gray",
        /// PANTONE 19-2024 TCX `Rhododendron`.
        Rhododendron => "rhododendron",
        /// PANTONE 15-0643 TCX `Cress Green`.
        CressGreen => "cress-green",
        /// PANTONE 17-3628 TCX `Amethyst Orchid`.
        AmethystOrchid => "amethyst-orchid",
        /// PANTONE 17-3812 TCX `Dusk`.
        Dusk => "dusk"
    }
}

/// The sRGB values of the [`JaponaisColor`] slots.
pub static JAPONAIS: StaticPalette<JaponaisColor, u8> = static_palette! {
    JaponaisColor::StretchLimo => PantoneFhiNamedColor::StretchLimo.to_color(),
    JaponaisColor::MineralGray => PantoneFhiNamedColor::MineralGray.to_color(),
    JaponaisColor::Rhododendron => PantoneFhiNamedColor::Rhododendron.to_color(),
    JaponaisColor::CressGreen => PantoneFhiNamedColor::CressGreen.to_color(),
    JaponaisColor::AmethystOrchid => PantoneFhiNamedColor::AmethystOrchid.to_color(),
    JaponaisColor::Dusk => PantoneFhiNamedColor::Dusk.to_color(),
};
