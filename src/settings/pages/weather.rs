use super::super::button::{self, Style};
use super::super::page::Page;
use super::super::slider::{self, Range};
use super::super::{choice, field, row};
use crate::floating::Weather;

pub fn build(page: &mut Page) {
    field::add(
        page,
        "weather_place",
        "Place name",
        "The name the weather card shows",
    );

    field::add(
        page,
        "weather_latitude",
        "Latitude",
        "North is positive, like 6.18 for Jakarta",
    );

    field::add(
        page,
        "weather_longitude",
        "Longitude",
        "East is positive, like 106.83 for Jakarta",
    );

    page.end_group();

    choice::add(
        page,
        "weather_unit",
        "Temperature unit",
        "The unit the weather card shows temperatures in",
        &[("Celsius", "celsius"), ("Fahrenheit", "fahrenheit")],
    );

    slider::add(
        page,
        "weather_minutes",
        "Refresh interval",
        "How often the weather is asked for again",
        Range {
            min: 5.0,
            max: 60.0,
            step: 5.0,
            label: slider::minutes,
        },
    );

    let label = "Refresh";

    let control = button::view(page.theme, label, Style::Plain, true, Weather::refresh);

    let refresh = row::view(
        page.theme,
        page.width,
        row::HEIGHT,
        "Refresh now",
        "Ask for the current weather without waiting",
        control,
        button::width(label),
    );

    page.row(row::HEIGHT, refresh);

    page.end_group();
}
