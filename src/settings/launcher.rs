use amane::{Column, children};

use super::slider::{self, Range};
use super::switch;
use crate::theme::Theme;

// the launcher's size, what its results show, and its commands
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        slider::row(
            theme,
            width,
            "launcher_width",
            "Width",
            "The launcher's width",
            Range {
                min: 420.0,
                max: 900.0,
                step: 20.0,
                label: slider::pixels,
            },
        ),
        slider::row(
            theme,
            width,
            "launcher_rows",
            "Visible results",
            "How many results show before the list scrolls",
            Range {
                min: 3.0,
                max: 14.0,
                step: 1.0,
                label: slider::count,
            },
        ),
        switch::row(
            theme,
            width,
            "launcher_descriptions",
            "Descriptions",
            "An app's description under its name",
        ),
        switch::row(theme, width, "launcher_icons", "Icons", "App and command icons"),
        switch::row(
            theme,
            width,
            "launcher_remember_query",
            "Remember query",
            "Keep the last search when the launcher opens again",
        ),
        switch::row(theme, width, "launcher_commands", "Command mode", "Typing > lists commands"),
        switch::row(theme, width, "command_settings", "Settings command", "Settings in command mode"),
        switch::row(theme, width, "command_colors", "Color command", "Color scheme in command mode"),
        switch::row(theme, width, "command_tmux", "Tmux command", "Tmux sessions in command mode"),
        switch::row(theme, width, "command_wallpapers", "Wallpaper command", "Wallpapers in command mode"),
    ])
    .gap(12.0)
}
