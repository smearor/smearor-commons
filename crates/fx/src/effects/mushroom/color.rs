use smearor_commons_color::palette_definition;

palette_definition! {
    /// The `Mushroom` star-ramp slots, in interpolation order - the
    /// named equivalent of BMW's `mushroom-star-color-0` ..
    /// `mushroom-star-color-5` GSettings keys. `Star0` is the
    /// ring-center (`v = 0`) color, `Star5` the outer edge.
    pub enum StarColor {
        /// Stop 0 - ring center (`mushroom-star-color-0`).
        #[serde(rename = "star-0")]
        Star0 => "star-0",
        /// Stop 1 (`mushroom-star-color-1`).
        #[serde(rename = "star-1")]
        Star1 => "star-1",
        /// Stop 2 (`mushroom-star-color-2`).
        #[serde(rename = "star-2")]
        Star2 => "star-2",
        /// Stop 3 (`mushroom-star-color-3`).
        #[serde(rename = "star-3")]
        Star3 => "star-3",
        /// Stop 4 (`mushroom-star-color-4`).
        #[serde(rename = "star-4")]
        Star4 => "star-4",
        /// Stop 5 - ring edge (`mushroom-star-color-5`).
        #[serde(rename = "star-5")]
        Star5 => "star-5"
    }
}
