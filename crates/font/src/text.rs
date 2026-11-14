//! Text-level metrics: width truncation.

use ab_glyph::Font;
use ab_glyph::PxScaleFont;
use ab_glyph::ScaleFont;
use smearor_commons_geometry::Pixels;

/// Truncates `text` to fit within `max_width` at the scaled font.
///
/// Incremental advance tracking — O(n) in the number of characters.
pub fn truncate_text_to_width<F: Font>(text: &str, scaled_font: &PxScaleFont<&F>, max_width: Pixels<f32>) -> String {
    let mut result = String::new();
    let mut current_width = Pixels::new(0.0_f32);
    for ch in text.chars() {
        let glyph_id = scaled_font.glyph_id(ch);
        let char_width = Pixels::new(scaled_font.h_advance(glyph_id));
        if current_width + char_width > max_width {
            break;
        }
        current_width = current_width + char_width;
        result.push(ch);
    }
    result
}
