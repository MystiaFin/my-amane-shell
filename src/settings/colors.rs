use amane::{Column, Service, children};

use super::slider::{self, Range};
use super::{choice, row, switch};
use crate::theme::{Mode, Theme};

// the palette the shell draws with and how it is made
pub fn view(theme: &Theme, width: f32) -> Column {
    let scheme = choice::row(
        theme,
        width,
        "scheme",
        "Color scheme",
        "The palette family the shell uses",
        &[("Dynamic", "dynamic"), ("Gruvbox", "gruvbox"), ("Catppuccin", "catppuccin")],
    );

    let accent = choice::row(
        theme,
        width,
        "accent",
        "Accent preset",
        "Used instead of the wallpaper's accent while manual accent is on",
        &[
            ("Blue", "#89b4fa"),
            ("Rose", "#f38ba8"),
            ("Peach", "#fab387"),
            ("Green", "#a6e3a1"),
            ("Teal", "#94e2d5"),
            ("Purple", "#cba6f7"),
        ],
    );

    Column::new(children![
        scheme,
        mode(theme, width),
        switch::row(
            theme,
            width,
            "manual_accent",
            "Manual accent",
            "Override the accent taken from the wallpaper",
        ),
        accent,
        slider::row(
            theme,
            width,
            "saturation",
            "Saturation",
            "How colorful the dynamic palette is",
            Range {
                min: 0.55,
                max: 1.45,
                step: 0.05,
                label: slider::times,
            },
        ),
        slider::row(
            theme,
            width,
            "contrast",
            "Contrast",
            "How far apart the dynamic palette's tones sit",
            Range {
                min: 0.75,
                max: 1.35,
                step: 0.05,
                label: slider::times,
            },
        ),
        switch::row(
            theme,
            width,
            "follow_wallpaper",
            "Follow wallpaper colors",
            "Rebuild the dynamic palette when the wallpaper changes",
        ),
    ])
    .gap(12.0)
}

// light, dark, or following the wallpaper's brightness
fn mode(theme: &Theme, width: f32) -> amane::Rectangle {
    let picked = Mode::read().light;

    let options = [("Auto", None), ("Light", Some(true)), ("Dark", Some(false))];

    let control = choice::view(theme, "color-mode", &options, picked, Mode::set);

    row::view(
        theme,
        width,
        "Color mode",
        "Auto follows how bright the wallpaper is",
        control,
    )
}
