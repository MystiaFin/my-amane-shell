use amane::{Center, Padding, Rectangle, Service, Start, TextInput};

use super::{Settings, row};
use crate::theme::Theme;

const FIELD_WIDTH: f32 = 260.0;

// a setting that is typed; the text input is named after the key
pub fn row(theme: &Theme, width: f32, key: &'static str, title: &str, detail: &str) -> Rectangle {
    let input = TextInput::new(key)
        .size(13.0)
        .color(theme.text)
        .on_change(move |text| Settings::set(key, text));

    let field = Rectangle::new()
        .width(FIELD_WIDTH)
        .height(34.0)
        .radius(12.0)
        .fill(theme.surface)
        .border(1.0, theme.border)
        .padding(Padding {
            top: 0.0,
            right: 12.0,
            bottom: 0.0,
            left: 12.0,
        })
        .align_child(Start, Center)
        .child(input);

    row::view(theme, width, title, detail, field)
}

// the field starts with what was saved, not with what was typed last time
pub fn fill(key: &'static str) {
    let saved = String::from(Settings::read().text(key));

    TextInput::set_text(key, &saved);
}
