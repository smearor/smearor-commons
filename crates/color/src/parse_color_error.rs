/// Error returned when parsing an invalid hex color string.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid color '{input}': {reason}")]
pub struct ParseColorError {
    /// The invalid input string.
    pub input: String,
    /// Human-readable reason for the failure.
    pub reason: &'static str,
}

impl ParseColorError {
    /// Creates a parse error for the given input and reason.
    pub(crate) fn new(input: &str, reason: &'static str) -> Self {
        Self {
            input: input.to_string(),
            reason,
        }
    }
}
