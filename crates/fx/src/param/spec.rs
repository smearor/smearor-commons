use super::ParamKind;

/// How one parameter is declared - keys match serde field names.
///
/// `params()` metadata drives everything downstream: GTK control rows
/// (phase 3 experimenter), serde defaults, `emit` codegen.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamSpec {
    /// The serde/JSON key, e.g. `"scale"`.
    pub key: &'static str,
    /// The control label, e.g. `"Scale"`.
    pub label: &'static str,
    /// Widget + default information.
    pub kind: ParamKind,
}
