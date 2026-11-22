use serde::Deserialize;
use serde::Serialize;

/// Which texture a layered pass renders - Burn-My-Windows' `uLayer`.
///
/// Single-layer effects always render `Layer::Window`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    /// The backdrop behind the content (`uLayer = 0` in BMW).
    Surface,
    /// The animated content itself (`uLayer = 1` in BMW) - the default
    /// for every single-layer effect.
    #[default]
    Window,
}
