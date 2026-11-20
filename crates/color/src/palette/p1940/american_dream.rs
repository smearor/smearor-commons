use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The American Dream" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum AmericanDreamColor {
        /// PANTONE 14-1230 TCX `Apricot Wash`.
        ApricotWash => "apricot-wash",
        /// PANTONE 13-1031 TCX `Apricot Sherbet`.
        ApricotSherbet => "apricot-sherbet",
        /// PANTONE 14-6319 TCX `Meadow`.
        Meadow => "meadow",
        /// PANTONE 19-1338 TCX `Russet Brown`.
        RussetBrown => "russet-brown",
        /// PANTONE 12-0712 TCX `Vanilla`.
        Vanilla => "vanilla",
        /// PANTONE 14-1506 TCX `Rose Smoke`.
        RoseSmoke => "rose-smoke"
    }
}

/// The sRGB values of the [`AmericanDreamColor`] slots.
pub static AMERICAN_DREAM: StaticPalette<AmericanDreamColor, u8> = static_palette! {
    AmericanDreamColor::ApricotWash => PantoneFhiNamedColor::ApricotWash.to_color(),
    AmericanDreamColor::ApricotSherbet => PantoneFhiNamedColor::ApricotSherbet.to_color(),
    AmericanDreamColor::Meadow => PantoneFhiNamedColor::Meadow.to_color(),
    AmericanDreamColor::RussetBrown => PantoneFhiNamedColor::RussetBrown.to_color(),
    AmericanDreamColor::Vanilla => PantoneFhiNamedColor::Vanilla.to_color(),
    AmericanDreamColor::RoseSmoke => PantoneFhiNamedColor::RoseSmoke.to_color(),
};
