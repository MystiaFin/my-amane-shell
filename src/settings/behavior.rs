use amane::{Column, children};

use super::slider::{self, Range};
use super::switch;
use crate::theme::Theme;

// how panels close and how fast things move
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        switch::row(
            theme,
            width,
            "launcher_close_on_launch",
            "Close launcher after launch",
            "Hide the launcher once something starts",
        ),
        switch::row(
            theme,
            width,
            "launcher_escape_clears",
            "Escape clears first",
            "Escape empties the search before it closes the launcher",
        ),
        switch::row(
            theme,
            width,
            "click_outside_dismiss",
            "Click outside to dismiss",
            "A click outside an open panel closes it",
        ),
        slider::row(
            theme,
            width,
            "animation_speed",
            "Animation speed",
            "Faster or slower motion everywhere",
            Range {
                min: 0.5,
                max: 2.0,
                step: 0.1,
                label: slider::times,
            },
        ),
        switch::row(
            theme,
            width,
            "reduce_motion",
            "Reduce motion",
            "Things change at once instead of moving",
        ),
    ])
    .gap(12.0)
}
