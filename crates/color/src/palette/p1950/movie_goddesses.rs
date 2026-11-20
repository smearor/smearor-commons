use crate::Color;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Movie Goddesses" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum MovieGoddessesColor {
        /// Pantone 9286 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9286 => "pms-9286",
        /// Pantone 9483 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9483 => "pms-9483",
        /// Pantone 9140 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9140 => "pms-9140",
        /// Pantone 9402 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9402 => "pms-9402",
        /// Pantone 9200 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9200 => "pms-9200",
        /// Pantone 9340 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms9340 => "pms-9340"
    }
}

/// The sRGB values of the [`MovieGoddessesColor`] slots.
pub static MOVIE_GODDESSES: StaticPalette<MovieGoddessesColor, u8> = static_palette! {
    MovieGoddessesColor::Pms9286 => Color::from_rgb_u32(0xFCDCDD),
    MovieGoddessesColor::Pms9483 => Color::from_rgb_u32(0xB6DFDB),
    MovieGoddessesColor::Pms9140 => Color::from_rgb_u32(0xFFFAC3),
    MovieGoddessesColor::Pms9402 => Color::from_rgb_u32(0xC2DAF2),
    MovieGoddessesColor::Pms9200 => Color::from_rgb_u32(0xFEDCC0),
    MovieGoddessesColor::Pms9340 => Color::from_rgb_u32(0xFDEEF5),
};
