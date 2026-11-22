use serde::Deserialize;
use serde::Serialize;
use smearor_commons_color::Color;

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

/// The widget/kind of a parameter, carrying the schema default.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
    /// Float slider - GSettings schema `d`.
    Float {
        /// Lower slider bound.
        min: f64,
        /// Upper slider bound.
        max: f64,
        /// Step granularity (drives slider digits).
        step: f64,
        /// The schema default.
        default: f64,
    },
    /// Integer spin - GSettings schema `i`.
    Int {
        /// Lower bound.
        min: i64,
        /// Upper bound.
        max: i64,
        /// The schema default.
        default: i64,
    },
    /// Toggle - GSettings schema `b`.
    Toggle {
        /// The schema default.
        default: bool,
    },
    /// Color picker - GSettings schema `s` `rgba(...)`.
    /// `Color<f64>` serializes as a string (`"rgba(76, 51, 25, 0.0)"`,
    /// `#fc0`) - byte-identical to BMW's color keys.
    Color {
        /// The schema default.
        default: Color<f64>,
    },
    /// Dropdown - enum-ish params (Morph's `uStyle` etc.).
    Choice {
        /// The dropdown entries in index order.
        options: &'static [&'static str],
        /// The schema default index.
        default: usize,
    },
}

/// A live parameter value - the UI's read/write currency.
///
/// Untagged: JSON has no kind tag; the pairing with [`ParamKind`]
/// (same index in `params()`) is what makes a value meaningful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// A float parameter.
    Float(f64),
    /// A dropdown index.
    Choice(usize),
    /// An integer parameter.
    Int(i64),
    /// A toggle.
    Bool(bool),
    /// A color parameter (serializes as the `Color<f64>` string form).
    Color(Color<f64>),
}
