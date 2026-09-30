use std::env;
use std::fs;
use std::path::PathBuf;

use amane::Service;

// light or dark as chosen by hand, none to follow the wallpaper; kept across restarts
#[derive(Default)]
pub struct Mode {
    pub light: Option<bool>,
}

impl Service for Mode {
    fn new() -> Self {
        let saved = fs::read_to_string(path()).unwrap_or_default();

        let light = match saved.trim() {
            "light" => Some(true),
            "dark" => Some(false),
            _ => None,
        };

        Self { light }
    }

    // it only changes through input
    fn listen() {}
}

impl Mode {
    // flips whatever is showing now, so the first click always changes something
    pub fn toggle(showing_light: bool) {
        Self::set(Some(!showing_light));
    }

    // none goes back to following the wallpaper
    pub fn set(light: Option<bool>) {
        Self::write().light = light;

        save(light);
    }
}

fn save(light: Option<bool>) {
    let text = match light {
        Some(true) => "light",
        Some(false) => "dark",
        None => "auto",
    };

    let path = path();

    let folder = path.parent().expect("failed to find the state folder");

    // losing the choice only means the next start follows the wallpaper again
    let _ = fs::create_dir_all(folder);
    let _ = fs::write(&path, text);
}

fn path() -> PathBuf {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    PathBuf::from(format!("{home}/.local/state/amane/mode"))
}
