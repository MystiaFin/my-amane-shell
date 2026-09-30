use amane::{Center, Pointer, Rectangle, Row, Service, Text, Weight, Widget};

use super::{Settings, row};
use crate::fonts;
use crate::motion;
use crate::theme::{self, Theme};

const HEIGHT: f32 = 34.0;
const OPTION_WIDTH: f32 = 84.0;

// many options share this much room, so the row keeps space for its title
const MOST_WIDTH: f32 = 390.0;
const INSET: f32 = 3.0;

// a setting that is one of a few words
pub fn row(
    theme: &Theme,
    width: f32,
    key: &'static str,
    title: &str,
    detail: &str,
    options: &[(&str, &'static str)],
) -> Rectangle {
    let settings = Settings::read();

    // the saved word as one of the options, so it compares with them
    let mut picked = options[0].1;

    for (_, value) in options {
        if *value == settings.text(key) {
            picked = value;
        }
    }

    drop(settings);

    let control = view(theme, key, options, picked, move |value| Settings::set(key, value));

    row::view(theme, width, title, detail, control)
}

// a row of options with one picked, like auto, light and dark; `name` keeps each fade apart
pub fn view<T: Copy + PartialEq + 'static>(
    theme: &Theme,
    name: &str,
    options: &[(&str, T)],
    picked: T,
    pick: impl Fn(T) + Clone + 'static,
) -> Rectangle {
    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();

    let option_width = OPTION_WIDTH.min(MOST_WIDTH / options.len() as f32);

    for (label, value) in options {
        let value = *value;

        let pick = pick.clone();

        let target = if value == picked { 1.0 } else { 0.0 };

        let amount = motion::fade(&format!("{name}:{label}"), target);

        let fill = theme::mix(theme.surface, theme.accent, amount);
        let text = theme::mix(theme.text, theme.on_accent, amount);

        let button = Rectangle::new()
            .width(option_width)
            .height(HEIGHT - INSET * 2.0)
            .radius(10.0)
            .fill(fill)
            .cursor(Pointer)
            .on_click(move |_| pick(value))
            .align_child(Center, Center)
            .child(Text::new(*label).size(13.0).font(fonts::BODY).weight(Weight::Medium).color(text));

        buttons.push(Box::new(button));
    }

    let width = option_width * options.len() as f32 + INSET * 2.0;

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(12.0)
        .fill(theme.surface)
        .align_child(Center, Center)
        .child(Row::new(buttons))
}
