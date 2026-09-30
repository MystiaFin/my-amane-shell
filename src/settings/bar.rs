use amane::{Column, children};

use super::slider::{self, Range};
use super::{choice, switch};
use crate::theme::Theme;

// what the bar shows and how
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        choice::row(
            theme,
            width,
            "bar_position",
            "Position",
            "The screen edge the bar sits on",
            &[("Top", "top"), ("Bottom", "bottom")],
        ),
        slider::row(
            theme,
            width,
            "bar_height",
            "Height",
            "The bar's height, its content keeps its size",
            Range {
                min: 32.0,
                max: 64.0,
                step: 2.0,
                label: slider::pixels,
            },
        ),
        switch::row(
            theme,
            width,
            "bar_auto_hide",
            "Auto-hide",
            "Slide away until the pointer reaches the screen edge",
        ),
        choice::row(
            theme,
            width,
            "workspace_style",
            "Workspace indicator",
            "How the workspace strip marks each workspace",
            &[("Pill", "pill"), ("Dots", "dots"), ("Numbers", "numbers")],
        ),
        switch::row(theme, width, "bar_logo", "NixOS logo", "Opens the power menu"),
        switch::row(theme, width, "bar_workspaces", "Workspaces", "The workspace strip"),
        switch::row(theme, width, "bar_workspace_name", "Workspace name", "The active workspace's name"),
        switch::row(theme, width, "bar_audio", "Audio", "Output and microphone rings"),
        switch::row(theme, width, "bar_media", "Media", "What is playing"),
        switch::row(theme, width, "bar_clock", "Clock and date", "Time and date in the middle"),
        switch::row(theme, width, "bar_battery", "Battery", "Battery level, on machines with one"),
        switch::row(theme, width, "bar_memory", "Memory", "Memory in use"),
        switch::row(theme, width, "bar_tray", "Tray", "Notifications, network and Bluetooth"),
        switch::row(theme, width, "clock_24_hour", "24-hour clock", "21:05 instead of 09:05 PM"),
        switch::row(theme, width, "clock_seconds", "Show seconds", "Seconds in the bar's clock"),
    ])
    .gap(12.0)
}
