/// Error returned when parsing an invalid color string.
///
/// The variants distinguish *why* parsing failed: hex syntax errors
/// ([`Self::MissingHashPrefix`], [`Self::InvalidDigitCount`],
/// [`Self::InvalidHexDigit`]) are produced by
/// [`crate::HexColor`], [`Self::UnknownNamedColor`],
/// [`Self::UnknownPantoneColor`] and [`Self::UnknownPantoneSuffix`]
/// by [`crate::CssNamedColor`], [`crate::PantoneNamedColor`] and
/// [`crate::PantoneSuffix`], [`Self::UnknownPantoneFhiColor`] and
/// [`Self::UnknownPantoneFhiSuffix`] by [`crate::PantoneFhiNamedColor`]
/// and [`crate::PantoneFhiSuffix`], and [`Self::InvalidColor`] by
/// [`crate::Color`] when the input matches neither grammar.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseColorError {
    /// The input is missing the leading `#` of a hex color.
    #[error("invalid color '{input}': expected a leading '#'")]
    MissingHashPrefix {
        /// The invalid input string.
        input: String,
    },
    /// The hex part has an invalid number of digits (3, 4, 6, or 8
    /// expected).
    #[error("invalid color '{input}': expected 3, 4, 6, or 8 hex digits, found {count}")]
    InvalidDigitCount {
        /// The invalid input string.
        input: String,
        /// The number of digits found.
        count: usize,
    },
    /// The hex part contains a non-hex character.
    #[error("invalid color '{input}': '{character}' is not a hex digit")]
    InvalidHexDigit {
        /// The invalid input string.
        input: String,
        /// The offending character.
        character: char,
    },
    /// The input is not a known CSS `<named-color>` keyword.
    #[error("invalid color '{input}': unknown CSS named color")]
    UnknownNamedColor {
        /// The invalid input string.
        input: String,
    },
    /// The input is not a known Pantone color name.
    #[error("invalid color '{input}': unknown Pantone color")]
    UnknownPantoneColor {
        /// The invalid input string.
        input: String,
    },
    /// The input is not a known Pantone fan-book suffix (`C`, `U` or `M`).
    #[error("invalid color '{input}': unknown Pantone suffix, expected 'C', 'U' or 'M'")]
    UnknownPantoneSuffix {
        /// The invalid input string.
        input: String,
    },
    /// The input is not a known Pantone FHI color code or name.
    #[error("invalid color '{input}': unknown Pantone FHI color")]
    UnknownPantoneFhiColor {
        /// The invalid input string.
        input: String,
    },
    /// The input is not a known Pantone FHI material suffix (`TCX`,
    /// `TPX` or `TPG`).
    #[error("invalid color '{input}': unknown Pantone FHI suffix, expected 'TCX', 'TPX' or 'TPG'")]
    UnknownPantoneFhiSuffix {
        /// The invalid input string.
        input: String,
    },
    /// The input matches neither the hex grammar nor a CSS named color.
    #[error("invalid color '{input}': expected a '#'-prefixed hex color or a CSS named color")]
    InvalidColor {
        /// The invalid input string.
        input: String,
    },
}

impl ParseColorError {
    /// The invalid input string.
    pub fn input(&self) -> &str {
        match self {
            Self::MissingHashPrefix { input }
            | Self::InvalidDigitCount { input, .. }
            | Self::InvalidHexDigit { input, .. }
            | Self::UnknownNamedColor { input }
            | Self::UnknownPantoneColor { input }
            | Self::UnknownPantoneSuffix { input }
            | Self::UnknownPantoneFhiColor { input }
            | Self::UnknownPantoneFhiSuffix { input }
            | Self::InvalidColor { input } => input,
        }
    }
}
