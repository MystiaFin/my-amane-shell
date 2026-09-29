use amane::{Full, Pointer, Rectangle};

use super::{hover, hovered};
use crate::motion::{self, FAST_SPATIAL};
use crate::overlay::Overlay;
use crate::theme::{self, Theme};

pub const WIDTH: f32 = 48.0;
const HEIGHT: f32 = 28.0;

// the knob grows when switched on
const KNOB: f32 = 16.0;
const CHECKED_KNOB: f32 = 24.0;

// how far the knob sits from the left end when off, and from the right end when on
const OFF_INSET: f32 = 6.0;
const ON_INSET: f32 = 2.0;

// a capsule with a knob that slides right when on
pub fn view(
    overlay: &Overlay,
    theme: &Theme,
    name: &str,
    checked: bool,
    on_toggle: impl Fn() + 'static,
) -> Rectangle {
    let hover_name = format!("switch:{name}");

    let (target_size, target_x) = if checked {
        (CHECKED_KNOB, WIDTH - CHECKED_KNOB - ON_INSET)
    } else {
        (KNOB, OFF_INSET)
    };

    let size = motion::follow(&format!("{hover_name}:size"), target_size, FAST_SPATIAL);
    let x = motion::follow(&format!("{hover_name}:x"), target_x, FAST_SPATIAL);

    let (fill, knob_color) = if checked {
        (theme.accent, theme.on_accent)
    } else {
        (theme.selected_surface, theme.secondary_text)
    };

    // a faint layer of the knob's color while the pointer is on it
    let fill = if hovered(overlay, &hover_name) {
        theme::mix(fill, knob_color, 0.08)
    } else {
        fill
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
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(move |_| on_toggle())
        .child(knob);

    if !checked {
        track = track.border(2.0, theme.border);
    }

    track
}
