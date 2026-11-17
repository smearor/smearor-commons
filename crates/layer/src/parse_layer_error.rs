/// Error returned when parsing an invalid layer name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid layer '{input}': expected background, bottom, top or overlay")]
pub struct ParseLayerError {
    /// The invalid input string.
    pub input: String,
}

impl ParseLayerError {
    /// Creates a parse error for the given input.
    pub(crate) fn new(input: &str) -> Self {
        Self { input: input.to_string() }
    }
}
