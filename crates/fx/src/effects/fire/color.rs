use smearor_commons_color::palette_definition;

palette_definition! {
    /// The `Fire` flame-ramp slots, in rising-temperature order - the
    /// named equivalent of BMW's `fire-color-1` .. `fire-color-5`
    /// GSettings keys.
    ///
    /// The slot order doubles as the ramp order: [`crate::Fire`]
    /// evaluates [`smearor_commons_color::Palette`] values at the
    /// fixed stop positions `0.0`, `0.2`, `0.35`, `0.5`, `0.8`.
    pub enum FireColor {
        /// Transparent ember at the burn edge (`fire-color-1`).
        #[serde(rename = "ember")]
        Ember => "ember",
        /// Deep red outer flame (`fire-color-2`).
        #[serde(rename = "red")]
        Red => "red",
        /// Orange flame body (`fire-color-3`).
        #[serde(rename = "orange")]
        Orange => "orange",
        /// Yellow-hot inner flame (`fire-color-4`).
        #[serde(rename = "yellow")]
        Yellow => "yellow",
        /// White-hot core (`fire-color-5`).
        #[serde(rename = "white")]
        White => "white"
    }
}
