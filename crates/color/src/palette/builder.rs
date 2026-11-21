//! A fluent builder for [`Palette`]s - expressive assembly plus a
//! completeness gate for [`PaletteDefinition`] keys.

use crate::Color;
use crate::Palette;
use crate::PaletteDefinition;
use std::collections::HashMap;
use std::hash::Hash;

/// A fluent builder assembling a [`Palette`] name by name.
///
/// ```
/// use smearor_commons_color::Color;
/// use smearor_commons_color::Palette;
/// use smearor_commons_color::palette_definition;
///
/// palette_definition! {
///     /// Square colors of a board theme.
///     pub enum BoardSlot {
///         /// Light squares.
///         Light => "light",
///         /// Dark squares.
///         Dark => "dark"
///     }
/// }
///
/// let palette = Palette::<BoardSlot, u8>::builder()
///     .color(BoardSlot::Light, Color::<u8>::new(0xED, 0xD4, 0x9E))
///     .color(BoardSlot::Dark, Color::<u8>::new(0x73, 0x4D, 0x2E))
///     .build();
/// assert!(palette.is_complete());
/// ```
#[derive(Debug, Default)]
pub struct PaletteBuilder<N: Eq + Hash + 'static, T: 'static> {
    /// Name/color pairs collected so far.
    colors: HashMap<N, Color<T>>,
}

impl<N: Eq + Hash + 'static, T: 'static> PaletteBuilder<N, T> {
    /// Starts an empty builder.
    pub fn new() -> Self {
        Self { colors: HashMap::new() }
    }

    /// Registers `color` under `name` - later duplicates win.
    ///
    /// Names go through `Into`, so enum variants can be used
    /// directly and [`crate::NamedColor`] keys can be written as
    /// plain `&str`.
    pub fn color(mut self, name: impl Into<N>, color: Color<T>) -> Self {
        self.colors.insert(name.into(), color);
        self
    }

    /// Registers all `(name, color)` pairs - later duplicates win.
    pub fn colors(mut self, pairs: impl IntoIterator<Item = (N, Color<T>)>) -> Self {
        self.colors.extend(pairs);
        self
    }

    /// Finishes the palette, complete or not.
    pub fn build(self) -> Palette<N, T> {
        Palette::new(self.colors)
    }
}

impl<N: PaletteDefinition, T: 'static> PaletteBuilder<N, T> {
    /// Finishes the palette only when every declared slot carries a
    /// color - the completeness gate [`Palette::is_complete`]
    /// enforces after the fact.
    pub fn build_complete(self) -> Result<Palette<N, T>, PaletteBuildError<N>> {
        let palette = self.build();
        let missing = palette.missing();
        if missing.is_empty() {
            Ok(palette)
        } else {
            Err(PaletteBuildError::Missing(missing))
        }
    }
}

/// The palette could not be built because declared slots have no
/// color - the failing slots are part of the error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PaletteBuildError<N> {
    /// These [`PaletteDefinition`] slots were left without a color.
    #[error("missing colors for palette slots: {0:?}")]
    Missing(Vec<N>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NamedColor;
    use crate::palette_definition;

    palette_definition! {
        /// Colors of a chess board theme.
        pub enum BoardSlot {
            /// Light squares.
            Light => "light",
            /// Dark squares.
            Dark => "dark",
            /// Frame around the board.
            Frame => "frame"
        }
    }

    #[test]
    fn color_registers_by_name() {
        let palette = Palette::<BoardSlot, u8>::builder()
            .color(BoardSlot::Light, Color::<u8>::new(0xED, 0xD4, 0x9E))
            .color(BoardSlot::Dark, Color::<u8>::new(0x73, 0x4D, 0x2E))
            .build();
        assert_eq!(palette.get(&BoardSlot::Light), Some(&Color::<u8>::new(0xED, 0xD4, 0x9E)));
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn names_go_through_into() {
        let palette = Palette::<NamedColor, u8>::builder().color("light", Color::<u8>::new(0xED, 0xD4, 0x9E)).build();
        assert_eq!(palette.get("light"), Some(&Color::<u8>::new(0xED, 0xD4, 0x9E)));
    }

    #[test]
    fn later_duplicates_win() {
        let palette = Palette::<BoardSlot, u8>::builder()
            .color(BoardSlot::Light, Color::<u8>::WHITE)
            .color(BoardSlot::Light, Color::<u8>::BLACK)
            .build();
        assert_eq!(palette.get(&BoardSlot::Light), Some(&Color::<u8>::BLACK));
    }

    #[test]
    fn colors_registers_pairs() {
        let palette = Palette::<BoardSlot, u8>::builder()
            .colors([(BoardSlot::Light, Color::<u8>::WHITE), (BoardSlot::Dark, Color::<u8>::BLACK)])
            .build();
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn build_complete_succeeds_when_every_slot_is_set() {
        let palette = Palette::<BoardSlot, u8>::builder()
            .color(BoardSlot::Light, Color::<u8>::WHITE)
            .color(BoardSlot::Dark, Color::<u8>::BLACK)
            .color(BoardSlot::Frame, Color::<u8>::new(0x80, 0x60, 0x40))
            .build_complete()
            .unwrap_or_else(|error| panic!("unexpected build error: {error}"));
        assert!(palette.is_complete());
    }

    #[test]
    fn build_complete_reports_the_missing_slots() {
        let error = Palette::<BoardSlot, u8>::builder().color(BoardSlot::Light, Color::<u8>::WHITE).build_complete();
        assert_eq!(error, Err(PaletteBuildError::Missing(vec![BoardSlot::Dark, BoardSlot::Frame])));
        assert_eq!(error.err().map(|e| e.to_string()).as_deref(), Some("missing colors for palette slots: [Dark, Frame]"));
    }
}
