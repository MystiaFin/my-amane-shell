use amane::{Full, Pointer, Rectangle, Service};

use super::{Settings, row};
use crate::motion::{self, FAST_SPATIAL};
use crate::theme::Theme;

const WIDTH: f32 = 48.0;
const HEIGHT: f32 = 28.0;

// the knob grows when switched on
const KNOB: f32 = 16.0;
const CHECKED_KNOB: f32 = 24.0;

// how far the knob sits from the left end when off, and from the right end when on
const OFF_INSET: f32 = 6.0;
const ON_INSET: f32 = 2.0;

// a setting that is on or off
pub fn row(theme: &Theme, width: f32, key: &'static str, title: &str, detail: &str) -> Rectangle {
    let checked = Settings::read().flag(key);

    row::view(theme, width, title, detail, view(theme, key, checked))
}

// a capsule with a knob that slides right when on
fn view(theme: &Theme, key: &'static str, checked: bool) -> Rectangle {
    let (target_size, target_x) = if checked {
        (CHECKED_KNOB, WIDTH - CHECKED_KNOB - ON_INSET)
    } else {
        (KNOB, OFF_INSET)
    };

    let size = motion::follow(&format!("settings-switch:{key}:size"), target_size, FAST_SPATIAL);
    let x = motion::follow(&format!("settings-switch:{key}:x"), target_x, FAST_SPATIAL);

    let (fill, knob_color) = if checked {
        (theme.accent, theme.on_accent)
    } else {
        (theme.selected_surface, theme.secondary_text)
    };

    let knob = Rectangle::new()
        .width(size)
        .height(size)
        .radius(Full)
        .fill(knob_color)
        .translate(x, (HEIGHT - size) / 2.0);

    let mut track = Rectangle::new()
        .width(WIDTH)
        .height(HEIGHT)
        .radius(Full)
        .fill(fill)
        .cursor(Pointer)
        .on_click(move |_| Settings::toggle(key))
        .child(knob);

    if !checked {
        track = track.border(2.0, theme.border);
    }

    track
}
