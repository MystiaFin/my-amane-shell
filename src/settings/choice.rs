use amane::{Center, Pointer, Rectangle, Row, Text, Weight, Widget};

use crate::fonts;
use crate::motion;
use crate::theme::{self, Theme};

const HEIGHT: f32 = 34.0;
const OPTION_WIDTH: f32 = 84.0;
const INSET: f32 = 3.0;

// a row of options with one picked, like auto, light and dark; `name` keeps each fade apart
pub fn view<T: Copy + PartialEq + 'static>(
    theme: &Theme,
    name: &str,
    options: &[(&str, T)],
    picked: T,
    pick: fn(T),
) -> Rectangle {
    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();

    for (label, value) in options {
        let value = *value;

        let target = if value == picked { 1.0 } else { 0.0 };

        let amount = motion::fade(&format!("{name}:{label}"), target);

        let fill = theme::mix(theme.surface, theme.accent, amount);
        let text = theme::mix(theme.text, theme.on_accent, amount);

        let button = Rectangle::new()
            .width(OPTION_WIDTH)
            .height(HEIGHT - INSET * 2.0)
            .radius(10.0)
            .fill(fill)
            .cursor(Pointer)
            .on_click(move |_| pick(value))
            .align_child(Center, Center)
            .child(Text::new(*label).size(13.0).font(fonts::BODY).weight(Weight::Medium).color(text));

        buttons.push(Box::new(button));
    }

    let width = OPTION_WIDTH * options.len() as f32 + INSET * 2.0;

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(12.0)
        .fill(theme.surface)
        .align_child(Center, Center)
        .child(Row::new(buttons))
}
