//! The 1920s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod art_deco;
mod bauhaus;
mod cocktails_and_laughter;
mod destinations;
mod leyendecker_man;
mod modern_pleasures;
mod rose_is_a_rose;
mod signature_hues;
mod tutmania;

pub use art_deco::ART_DECO;
pub use art_deco::ArtDecoColor;
pub use bauhaus::BAUHAUS;
pub use bauhaus::BauhausColor;
pub use cocktails_and_laughter::COCKTAILS_AND_LAUGHTER;
pub use cocktails_and_laughter::CocktailsAndLaughterColor;
pub use destinations::DESTINATIONS;
pub use destinations::DestinationsColor;
pub use leyendecker_man::LEYENDECKER_MAN;
pub use leyendecker_man::LeyendeckerManColor;
pub use modern_pleasures::MODERN_PLEASURES;
pub use modern_pleasures::ModernPleasuresColor;
pub use rose_is_a_rose::ROSE_IS_A_ROSE;
pub use rose_is_a_rose::RoseIsARoseColor;
pub use signature_hues::SIGNATURE_HUES_1920;
pub use signature_hues::SignatureHues1920Color;
pub use tutmania::TUTMANIA;
pub use tutmania::TutmaniaColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(ART_DECO.is_complete());
        assert!(TUTMANIA.is_complete());
        assert!(COCKTAILS_AND_LAUGHTER.is_complete());
        assert!(DESTINATIONS.is_complete());
        assert!(LEYENDECKER_MAN.is_complete());
        assert!(BAUHAUS.is_complete());
        assert!(MODERN_PLEASURES.is_complete());
        assert!(ROSE_IS_A_ROSE.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(ART_DECO.len(), 6);
        assert_eq!(TUTMANIA.len(), 7);
        assert_eq!(COCKTAILS_AND_LAUGHTER.len(), 7);
        assert_eq!(DESTINATIONS.len(), 8);
        assert_eq!(LEYENDECKER_MAN.len(), 6);
        assert_eq!(BAUHAUS.len(), 8);
        assert_eq!(MODERN_PLEASURES.len(), 7);
        assert_eq!(ROSE_IS_A_ROSE.len(), 6);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1920.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1920.len(), 8);
        assert_eq!(SignatureHues1920Color::slots()[0], SignatureHues1920Color::ArtDeco);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1920.get(&SignatureHues1920Color::ArtDeco), Some(&Color::<u8>::from_rgb_u32(0xA2A2A1)));
    }
}
