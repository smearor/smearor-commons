use std::hash::Hash;

/// Defines the color vocabulary of a [`crate::Palette`]: which
/// semantic color slots exist and how they are named.
///
/// Implemented by the palette's key type - typically a fieldless
/// enum in the consuming crate. With an enum key every lookup is
/// type-checked: `palette.get(&BoardSlot::Light)` cannot address a
/// name that does not exist, [`crate::Palette::missing`] reports
/// slots without a color and [`crate::Palette::is_complete`]
/// verifies that all declared slots are filled.
///
/// The [`crate::palette_definition!`] macro generates a fieldless
/// enum plus this implementation from a compact name list.
///
/// Key types with a dynamic vocabulary (like [`crate::NamedColor`])
/// cannot implement `PaletteDefinition` - they stay usable as
/// palette keys, just without the completeness checks.
pub trait PaletteDefinition: Copy + Eq + Hash + 'static {
    /// All color slots of the palette in declaration order - the
    /// canonical list for completeness checks and ordered iteration.
    fn slots() -> &'static [Self];

    /// The semantic name of this slot (`"light"`, `"ink"`, ...) -
    /// used for display and diagnostics.
    fn name(&self) -> &'static str;
}

/// Defines a fieldless enum and its [`PaletteDefinition`]
/// implementation from a `Variant => "name"` list.
///
/// The generated enum derives `Debug`, `Clone`, `Copy`, `PartialEq`,
/// `Eq`, `Hash`, `serde::Serialize` and `serde::Deserialize` -
/// `serde` must be a dependency of the calling crate. Attributes in
/// front of `enum` are forwarded, so `#[serde(rename_all =
/// "snake_case")]` can adapt the wire names.
///
/// ```
/// use smearor_commons_color::PaletteDefinition;
/// use smearor_commons_color::palette_definition;
///
/// palette_definition! {
///     /// Colors of a chess board theme.
///     pub enum BoardSlot {
///         /// Light squares.
///         Light => "light",
///         /// Dark squares.
///         Dark => "dark",
///         /// Frame around the board.
///         Frame => "frame"
///     }
/// }
///
/// assert_eq!(BoardSlot::slots(), &[BoardSlot::Light, BoardSlot::Dark, BoardSlot::Frame]);
/// assert_eq!(BoardSlot::Frame.name(), "frame");
/// ```
#[macro_export]
macro_rules! palette_definition {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident => $slot_name:literal
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        $vis enum $name {
            $(
                $(#[$vmeta])*
                $variant
            ),*
        }

        impl $crate::PaletteDefinition for $name {
            fn slots() -> &'static [Self] {
                &[$($name::$variant),*]
            }

            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => $slot_name),*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn slots_lists_variants_in_declaration_order() {
        assert_eq!(BoardSlot::slots(), &[BoardSlot::Light, BoardSlot::Dark, BoardSlot::Frame]);
    }

    #[test]
    fn name_returns_the_slot_name() {
        assert_eq!(BoardSlot::Light.name(), "light");
        assert_eq!(BoardSlot::Frame.name(), "frame");
    }
}
