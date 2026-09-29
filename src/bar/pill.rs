use amane::{Color, Full, Rectangle, Text, Weight};

use super::ITEM_HEIGHT;
use crate::fonts;

/*
 * amane has no size-to-content yet, so widths are guessed from the text;
 * Poppins averages about 0.62 of its size per character
 */
pub fn text_width(text: &str, size: f32) -> f32 {
    let characters = text.chars().count() as f32;

    characters * size * 0.62
}

// a rounded capsule, jaqc sizes each one as its content plus some padding
pub fn view(width: f32, fill: Color) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(ITEM_HEIGHT)
        .radius(Full)
        .fill(fill)
}

// jaqc's BarText: Poppins, light weight
pub fn label(text: &str, size: f32, color: Color) -> Text {
    Text::new(text)
        .size(size)
        .font(fonts::BODY)
        .weight(Weight::Light)
        .color(color)
}
