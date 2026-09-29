use std::env;
use std::fs;

use amane::{Palette, Service};

// jaqc's wallpaper picker writes the chosen file here, so both shells follow the same wallpaper
const SELECTION: &str = ".config/quickshell/wallpaper-selection";

// jaqc's quantizer depth 4 gives 16 colors
const PALETTE_SIZE: usize = 16;

// the path of the current wallpaper, re-read whenever the selection file changes
pub struct Wallpaper {
    pub path: String,
}

impl Service for Wallpaper {
    fn new() -> Self {
        let path = read_selection();

        Palette::write().open(&path, PALETTE_SIZE);

        Self { path }
    }

    fn listen() {
        for _ in amane::watch_file(&selection_file()) {
            let path = read_selection();

            if path == Self::read().path {
                continue;
            }

            Palette::write().open(&path, PALETTE_SIZE);

            Self::write().path = path;
        }
    }
}

fn selection_file() -> String {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    format!("{home}/{SELECTION}")
}

// the file holds a url like file:///home/me/Pictures/wall.png
fn read_selection() -> String {
    let text = fs::read_to_string(selection_file()).unwrap_or_default();

    let url = text.trim();

    let path = url.strip_prefix("file://").unwrap_or(url);

    String::from(path)
}
