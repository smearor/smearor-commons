//! The 1970s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod avocado_and_harvest_gold;
mod colors_and_coordinates;
mod day_the_world_turned_day_glo;
mod feathers_and_leathers;
mod hotel_california;
mod land_art;
mod night_life;
mod provence;
mod signature_hues;

pub use avocado_and_harvest_gold::AVOCADO_AND_HARVEST_GOLD;
pub use avocado_and_harvest_gold::AvocadoAndHarvestGoldColor;
pub use colors_and_coordinates::COLORS_AND_COORDINATES;
pub use colors_and_coordinates::ColorsAndCoordinatesColor;
pub use day_the_world_turned_day_glo::DAY_THE_WORLD_TURNED_DAY_GLO;
pub use day_the_world_turned_day_glo::DayTheWorldTurnedDayGloColor;
pub use feathers_and_leathers::FEATHERS_AND_LEATHERS;
pub use feathers_and_leathers::FeathersAndLeathersColor;
pub use hotel_california::HOTEL_CALIFORNIA;
pub use hotel_california::HotelCaliforniaColor;
pub use land_art::LAND_ART;
pub use land_art::LandArtColor;
pub use night_life::NIGHT_LIFE;
pub use night_life::NightLifeColor;
pub use provence::PROVENCE;
pub use provence::ProvenceColor;
pub use signature_hues::SIGNATURE_HUES_1970;
pub use signature_hues::SignatureHues1970Color;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(COLORS_AND_COORDINATES.is_complete());
        assert!(AVOCADO_AND_HARVEST_GOLD.is_complete());
        assert!(FEATHERS_AND_LEATHERS.is_complete());
        assert!(PROVENCE.is_complete());
        assert!(LAND_ART.is_complete());
        assert!(DAY_THE_WORLD_TURNED_DAY_GLO.is_complete());
        assert!(NIGHT_LIFE.is_complete());
        assert!(HOTEL_CALIFORNIA.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(COLORS_AND_COORDINATES.len(), 8);
        assert_eq!(AVOCADO_AND_HARVEST_GOLD.len(), 6);
        assert_eq!(FEATHERS_AND_LEATHERS.len(), 6);
        assert_eq!(PROVENCE.len(), 7);
        assert_eq!(LAND_ART.len(), 7);
        assert_eq!(DAY_THE_WORLD_TURNED_DAY_GLO.len(), 8);
        assert_eq!(NIGHT_LIFE.len(), 7);
        assert_eq!(HOTEL_CALIFORNIA.len(), 7);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1970.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1970.len(), 8);
        assert_eq!(SignatureHues1970Color::slots()[0], SignatureHues1970Color::ColorsAndCoordinates);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(
            SIGNATURE_HUES_1970.get(&SignatureHues1970Color::ColorsAndCoordinates),
            Some(&Color::<u8>::from_rgb_u32(0xCFB023))
        );
    }
}
