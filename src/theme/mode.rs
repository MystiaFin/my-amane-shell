use amane::Service;

use crate::settings::Settings;

// light or dark as chosen by hand, kept with the other settings
pub struct Mode;

impl Mode {
    // none follows the wallpaper
    pub fn light() -> Option<bool> {
        match Settings::read().text("color_mode") {
            "light" => Some(true),
            "dark" => Some(false),
            _ => None,
        }
    }

    // flips whatever is showing now, so the first click always changes something
    pub fn toggle(showing_light: bool) {
        let mode = if showing_light { "dark" } else { "light" };

        Settings::set("color_mode", mode);
    }
}
