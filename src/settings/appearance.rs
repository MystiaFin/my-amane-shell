use amane::{Rectangle, Service};

use super::{choice, row};
use crate::theme::{Mode, Theme};

// light, dark, or following the wallpaper's brightness
pub fn view(theme: &Theme, width: f32) -> Rectangle {
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
