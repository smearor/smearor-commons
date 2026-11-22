use smearor_commons_color::Color;

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
