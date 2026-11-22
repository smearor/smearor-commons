/// The rain-column domain of `matrix.frag`: per-column delay/speed
/// randomness and the fade constants of the drop + window masks.
///
/// BMW's rain RNG is the *sinus* hash family (`fract(sin(x) * k)`),
/// deliberately not the Hoskins family - kept for parity with the
/// reference visuals.
/// Per-column start delay - `hash11(column)` in the sinus family.
pub(super) fn column_hash(column: f64) -> f64 {
    crate::fract(column.sin() * 78.233)
}

/// Second column RNG from the same GLSL family (`cos` variant) -
/// scales the drop speed.
pub(super) fn column_speed_hash(column: f64) -> f64 {
    crate::fract(column.cos() * 12.989)
}

/// Column-end fade width in pixels - `matrix.frag`'s `EDGE_FADE`.
pub(super) const EDGE_FADE: f64 = 30.0;
/// Pixel length of the window-dissolve gradient behind a drop -
/// `FADE_WIDTH`.
pub(super) const FADE_WIDTH: f64 = 150.0;
/// Exponential tail length of a drop's trail - `TRAIL_LENGTH`.
pub(super) const TRAIL_LENGTH: f64 = 0.2;
/// Progress at which the global trail fade-out starts -
/// `FINAL_FADE_START_TIME`.
pub(super) const FINAL_FADE_START_TIME: f64 = 0.8;
