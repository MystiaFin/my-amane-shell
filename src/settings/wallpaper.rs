use amane::{Column, children};

use super::slider::{self, Range};
use super::{choice, switch, text};
use crate::theme::Theme;

// where wallpapers come from and how a new one appears
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        text::row(
            theme,
            width,
            "wallpaper_folder",
            "Wallpaper folder",
            "Where the picker and shuffle look for images",
        ),
        choice::row(
            theme,
            width,
            "wallpaper_transition",
            "Transition",
            "How a new wallpaper replaces the old one",
            &[("Circle", "circle"), ("Fade", "fade"), ("Instant", "instant")],
        ),
        slider::row(
            theme,
            width,
            "wallpaper_duration",
            "Transition duration",
            "How long the circle or fade takes",
            Range {
                min: 100.0,
                max: 3000.0,
                step: 100.0,
                label: slider::milliseconds,
            },
        ),
        switch::row(
            theme,
            width,
            "wallpaper_shuffle",
            "Shuffle",
            "Pick another image from the folder now and then",
        ),
        slider::row(
            theme,
            width,
            "wallpaper_shuffle_minutes",
            "Shuffle interval",
            "How often shuffle picks a new image",
            Range {
                min: 1.0,
                max: 180.0,
                step: 1.0,
                label: slider::minutes,
            },
        ),
    ])
    .gap(12.0)
}
