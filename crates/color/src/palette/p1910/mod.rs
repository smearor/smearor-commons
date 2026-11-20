//! The 1910s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod coming_home;
mod cubism;
mod parrish_blues;
mod signature_hues;
mod theatrics;
mod wiener_werkstaette;
mod world_war_1;
mod youthful_pastimes;

pub use coming_home::COMING_HOME;
pub use coming_home::ComingHomeColor;
pub use cubism::CUBISM;
pub use cubism::CubismColor;
pub use parrish_blues::PARRISH_BLUES;
pub use parrish_blues::ParrishBluesColor;
pub use signature_hues::SIGNATURE_HUES_1910;
pub use signature_hues::SignatureHues1910Color;
pub use theatrics::THEATRICS;
pub use theatrics::TheatricsColor;
pub use wiener_werkstaette::WIENER_WERKSTAETTE;
pub use wiener_werkstaette::WienerWerkstaetteColor;
pub use world_war_1::WORLD_WAR_1;
pub use world_war_1::WorldWar1Color;
pub use youthful_pastimes::YOUTHFUL_PASTIMES;
pub use youthful_pastimes::YouthfulPastimesColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(THEATRICS.is_complete());
        assert!(PARRISH_BLUES.is_complete());
        assert!(WIENER_WERKSTAETTE.is_complete());
        assert!(YOUTHFUL_PASTIMES.is_complete());
        assert!(CUBISM.is_complete());
        assert!(WORLD_WAR_1.is_complete());
        assert!(COMING_HOME.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(THEATRICS.len(), 8);
        assert_eq!(PARRISH_BLUES.len(), 7);
        assert_eq!(WIENER_WERKSTAETTE.len(), 10);
        assert_eq!(YOUTHFUL_PASTIMES.len(), 5);
        assert_eq!(CUBISM.len(), 8);
        assert_eq!(WORLD_WAR_1.len(), 6);
        assert_eq!(COMING_HOME.len(), 7);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1910.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1910.len(), 7);
        assert_eq!(SignatureHues1910Color::slots()[0], SignatureHues1910Color::Theatrics);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1910.get(&SignatureHues1910Color::Theatrics), Some(&Color::<u8>::from_rgb_u32(0xE47127)));
    }
}
