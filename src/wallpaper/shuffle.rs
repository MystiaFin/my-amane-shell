use std::time::Duration;

use amane::Service;

use super::picker::list_folder;
use super::state::{self, Wallpaper};
use crate::settings::Settings;

// counts minutes and picks a random wallpaper from the folder when shuffle's interval is up
#[derive(Default)]
pub struct Shuffle {
    minutes: u32,
}

impl Service for Shuffle {
    fn new() -> Self {
        Self::default()
    }

    fn interval() -> Duration {
        Duration::from_secs(60)
    }

    // nothing a window shows changes here, the wallpaper service takes the new file from there
    fn update(&mut self) -> bool {
        let settings = Settings::read();

        let on = settings.flag("wallpaper_shuffle");
        let every = settings.number("wallpaper_shuffle_minutes") as u32;

        drop(settings);

        if !on {
            self.minutes = 0;

            return false;
        }

        self.minutes += 1;

        if self.minutes < every {
            return false;
        }

        self.minutes = 0;

        pick();

        false
    }
}

// any file but the one on screen
fn pick() {
    let current = Wallpaper::read().path.clone();

    let mut files = list_folder();

    files.retain(|file| *file != current);

    if files.is_empty() {
        return;
    }

    let index = (state::random_share() * files.len() as f32) as usize;

    state::choose(&files[index.min(files.len() - 1)]);
}
