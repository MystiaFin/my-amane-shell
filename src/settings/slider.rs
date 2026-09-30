use amane::{
    Center, End, Full, Point, Pointer, Rectangle, Row, Service, Stack, Text, Weight, children,
};

use super::{Settings, row};
use crate::fonts;
use crate::theme::Theme;

const TRACK_WIDTH: f32 = 180.0;
const TRACK_HEIGHT: f32 = 6.0;
const THUMB: f32 = 16.0;
const HEIGHT: f32 = 28.0;

const VALUE_WIDTH: f32 = 56.0;

// the range a slider covers, how far one step moves it, and how its value reads
pub struct Range {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub label: fn(f32) -> String,
}

// a setting that is a number
pub fn row(
    theme: &Theme,
    width: f32,
    key: &'static str,
    title: &str,
    detail: &str,
    range: Range,
) -> Rectangle {
    let value = Settings::read().number(key);

    let label = Text::new((range.label)(value))
        .size(12.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.text);

    let label = Rectangle::new()
        .width(VALUE_WIDTH)
        .height(HEIGHT)
        .align_child(End, Center)
        .child(label);

    let control = Row::new(children![view(theme, key, value, range), label])
        .gap(8.0)
        .align(Center);

    row::view(theme, width, title, detail, control)
}

// pressing anywhere on it sets the value under the pointer, and dragging follows
fn view(theme: &Theme, key: &'static str, value: f32, range: Range) -> Stack {
    let share = ((value - range.min) / (range.max - range.min)).clamp(0.0, 1.0);

    let filled = Rectangle::new()
        .width(TRACK_WIDTH * share)
        .height(TRACK_HEIGHT)
        .radius(Full)
        .fill(theme.accent);

    let track = Rectangle::new()
        .width(TRACK_WIDTH)
        .height(TRACK_HEIGHT)
        .radius(Full)
        .fill(theme.border)
        .translate(0.0, (HEIGHT - TRACK_HEIGHT) / 2.0)
        .child(filled);

    // kept inside the track at both ends
    let thumb_x = (share * TRACK_WIDTH - THUMB / 2.0).clamp(0.0, TRACK_WIDTH - THUMB);

    let thumb = Rectangle::new()
        .width(THUMB)
        .height(THUMB)
        .radius(Full)
        .fill(theme.accent)
        .translate(thumb_x, (HEIGHT - THUMB) / 2.0);

    let area = Rectangle::new()
        .width(TRACK_WIDTH)
        .height(HEIGHT)
        .cursor(Pointer)
        .on_drag(move |point| drag(point, key, &range));

    Stack::new(children![track, thumb, area])
        .width(TRACK_WIDTH)
        .height(HEIGHT)
}

fn drag(point: Point, key: &'static str, range: &Range) {
    let share = (point.x / TRACK_WIDTH).clamp(0.0, 1.0);

    let raw = range.min + share * (range.max - range.min);

    let value = (raw / range.step).round() * range.step;

    // every move reports, but the file only needs writing when the value changes
    if (value - Settings::read().number(key)).abs() < range.step / 2.0 {
        return;
    }

    Settings::set(key, format!("{value:.3}"));
}

// labels for the ranges above, like "1.25×", "40 px" or "30 min"
pub fn times(value: f32) -> String {
    format!("{value:.2}×")
}

pub fn plain(value: f32) -> String {
    format!("{value:.2}")
}

pub fn count(value: f32) -> String {
    format!("{value:.0}")
}

pub fn pixels(value: f32) -> String {
    format!("{value:.0} px")
}

pub fn milliseconds(value: f32) -> String {
    format!("{value:.0} ms")
}

pub fn minutes(value: f32) -> String {
    format!("{value:.0} min")
}
