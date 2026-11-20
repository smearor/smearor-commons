//! The 1940s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod american_dream;
mod carefree_and_casual;
mod edward_hopper;
mod fantasia;
mod film_noir;
mod hit_parade;
mod la_mode;
mod signature_hues;
mod world_war_2;

pub use american_dream::AMERICAN_DREAM;
pub use american_dream::AmericanDreamColor;
pub use carefree_and_casual::CAREFREE_AND_CASUAL;
pub use carefree_and_casual::CarefreeAndCasualColor;
pub use edward_hopper::EDWARD_HOPPER;
pub use edward_hopper::EdwardHopperColor;
pub use fantasia::FANTASIA;
pub use fantasia::FantasiaColor;
pub use film_noir::FILM_NOIR;
pub use film_noir::FilmNoirColor;
pub use hit_parade::HIT_PARADE;
pub use hit_parade::HitParadeColor;
pub use la_mode::LA_MODE;
pub use la_mode::LaModeColor;
pub use signature_hues::SIGNATURE_HUES_1940;
pub use signature_hues::SignatureHues1940Color;
pub use world_war_2::WORLD_WAR_2;
pub use world_war_2::WorldWar2Color;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(FANTASIA.is_complete());
        assert!(EDWARD_HOPPER.is_complete());
        assert!(WORLD_WAR_2.is_complete());
        assert!(CAREFREE_AND_CASUAL.is_complete());
        assert!(AMERICAN_DREAM.is_complete());
        assert!(HIT_PARADE.is_complete());
        assert!(FILM_NOIR.is_complete());
        assert!(LA_MODE.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(FANTASIA.len(), 9);
        assert_eq!(EDWARD_HOPPER.len(), 5);
        assert_eq!(WORLD_WAR_2.len(), 7);
        assert_eq!(CAREFREE_AND_CASUAL.len(), 7);
        assert_eq!(AMERICAN_DREAM.len(), 6);
        assert_eq!(HIT_PARADE.len(), 8);
        assert_eq!(FILM_NOIR.len(), 5);
        assert_eq!(LA_MODE.len(), 7);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1940.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1940.len(), 8);
        assert_eq!(SignatureHues1940Color::slots()[0], SignatureHues1940Color::Fantasia);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1940.get(&SignatureHues1940Color::Fantasia), Some(&Color::<u8>::from_rgb_u32(0xD93744)));
    }
}
