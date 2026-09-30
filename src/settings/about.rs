use std::sync::OnceLock;

use amane::{Center, Column, Pointer, Rectangle, Text, Weight, children};

use super::{Settings, row, text};
use crate::fonts;
use crate::theme::Theme;

// asked once, a commit made while running shows after a restart
static COMMIT: OnceLock<String> = OnceLock::new();

// which config is running, and a way back to the defaults
pub fn view(theme: &Theme, width: f32) -> Column {
    let commit = COMMIT.get_or_init(|| {
        let commit = amane::output("git -C ~/.config/amane rev-parse --short HEAD");

        String::from(commit.trim())
    });

    let commit = Text::new(commit.as_str())
        .size(13.0)
        .font(fonts::BODY)
        .weight(Weight::Medium)
        .color(theme.secondary_text);

    Column::new(children![
        row::view(theme, width, "Config checkout", "The commit this shell was built from", commit),
        row::view(
            theme,
            width,
            "Reset settings",
            "Every setting back to its default",
            reset_button(theme),
        ),
    ])
    .gap(12.0)
}

fn reset_button(theme: &Theme) -> Rectangle {
    Rectangle::new()
        .width(96.0)
        .height(34.0)
        .radius(12.0)
        .fill(theme.danger)
        .cursor(Pointer)
        .on_click(|_| {
            Settings::reset();

            text::fill("wallpaper_folder");
        })
        .align_child(Center, Center)
        .child(
            Text::new("Reset")
                .size(13.0)
                .font(fonts::BODY)
                .weight(Weight::Medium)
                .color(theme.on_accent),
        )
}
