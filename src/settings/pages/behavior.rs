use super::super::page::Page;
use super::super::slider::{self, Range};
use super::super::switch;

pub fn build(page: &mut Page) {
    switch::add(
        page,
        "launcher_close_on_launch",
        "Close launcher after launch",
        "Dismiss the launcher after starting an app or tmux session",
    );

    switch::add(
        page,
        "launcher_escape_clears",
        "Escape clears launcher first",
        "Clear a non-empty query before Escape closes the launcher",
    );

    switch::add(
        page,
        "click_outside_dismiss",
        "Click outside to dismiss",
        "Let clicks outside open panels close the current overlay",
    );

    page.end_group();

    slider::add(
        page,
        "animation_speed",
        "Animation speed",
        "Apply a global speed multiplier to every animation",
        Range {
            min: 0.5,
            max: 2.0,
            step: 0.1,
            label: slider::times,
        },
    );

    switch::add(
        page,
        "reduce_motion",
        "Reduce motion",
        "Disable shell motion while preserving state changes",
    );

    page.end_group();
}
