use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1910s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1910Color {
        /// The lead swatch of the "Theatrics" palette - PANTONE 16-1255 TCX `Russet Orange`.
        Theatrics => "theatrics",
        /// The lead swatch of the "Parrish Blues" palette - PANTONE 17-4041 TCX `Marina`.
        ParrishBlues => "parrish-blues",
        /// The lead swatch of the "Wiener Werkstätte" palette - PANTONE 19-4203 TCX `Moonless Night`.
        WienerWerkstaette => "wiener-werkstaette",
        /// The lead swatch of the "Youthful Pastimes" palette - Pantone 7556 C.
        YouthfulPastimes => "youthful-pastimes",
        /// The lead swatch of the "Cubism" palette - PANTONE 19-3903 TCX `Shale`.
        Cubism => "cubism",
        /// The lead swatch of the "World War I" palette - PANTONE 16-1108 TCX `Twill`.
        WorldWar1 => "world-war-1",
        /// The lead swatch of the "Coming Home" palette - PANTONE 14-1025 TCX `Cocoon`.
        ComingHome => "coming-home"
    }
}

/// The sRGB values of the [`SignatureHues1910Color`] slots.
pub static SIGNATURE_HUES_1910: StaticPalette<SignatureHues1910Color, u8> = static_palette! {
    SignatureHues1910Color::Theatrics => PantoneFhiNamedColor::RussetOrange.to_color(),
    SignatureHues1910Color::ParrishBlues => PantoneFhiNamedColor::Marina.to_color(),
    SignatureHues1910Color::WienerWerkstaette => PantoneFhiNamedColor::MoonlessNight.to_color(),
    SignatureHues1910Color::YouthfulPastimes => PantoneNamedColor::Pms7556C.to_color(),
    SignatureHues1910Color::Cubism => PantoneFhiNamedColor::Shale.to_color(),
    SignatureHues1910Color::WorldWar1 => PantoneFhiNamedColor::Twill.to_color(),
    SignatureHues1910Color::ComingHome => PantoneFhiNamedColor::Cocoon.to_color(),
};
