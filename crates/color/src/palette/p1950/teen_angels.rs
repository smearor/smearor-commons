use crate::Color;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Teen Angels" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum TeenAngelsColor {
        /// Pantone 7471 C.
        Pms7471 => "pms-7471",
        /// Pantone 7625 C.
        Pms7625 => "pms-7625",
        /// Pantone 7606 C.
        Pms7606 => "pms-7606",
        /// Pantone 559 C.
        Pms559 => "pms-559",
        /// Pantone 10350 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10350 => "pms-10350",
        /// Pantone 10156 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10156 => "pms-10156",
        /// Pantone 10191 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10191 => "pms-10191",
        /// Pantone 10264 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10264 => "pms-10264"
    }
}

/// The sRGB values of the [`TeenAngelsColor`] slots.
pub static TEEN_ANGELS: StaticPalette<TeenAngelsColor, u8> = static_palette! {
    TeenAngelsColor::Pms7471 => PantoneNamedColor::Pms7471C.to_color(),
    TeenAngelsColor::Pms7625 => PantoneNamedColor::Pms7625C.to_color(),
    TeenAngelsColor::Pms7606 => PantoneNamedColor::Pms7606C.to_color(),
    TeenAngelsColor::Pms559 => PantoneNamedColor::Pms559C.to_color(),
    TeenAngelsColor::Pms10350 => Color::from_rgb_u32(0x947944),
    TeenAngelsColor::Pms10156 => Color::from_rgb_u32(0xBB7D70),
    TeenAngelsColor::Pms10191 => Color::from_rgb_u32(0x9B8B95),
    TeenAngelsColor::Pms10264 => Color::from_rgb_u32(0x648395),
};
