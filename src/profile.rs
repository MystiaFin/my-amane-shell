mod portal;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;

use amane::Service;

// the name and picture the lock screen shows, kept across restarts
#[derive(Default)]
pub struct Profile {
    // empty shows the login name
    name: String,

    picture: Option<PathBuf>,
}

impl Service for Profile {
    // the file holds a line like "name=Mystia" and one like "picture=/home/me/me.png"
    fn new() -> Self {
        let saved = fs::read_to_string(path()).unwrap_or_default();

        let mut profile = Self::default();

        for line in saved.lines() {
            if let Some(name) = line.strip_prefix("name=") {
                profile.name = String::from(name);
            }

            if let Some(picture) = line.strip_prefix("picture=") {
                profile.picture = Some(PathBuf::from(picture));
            }
        }

        profile
    }

    // it only changes through input
    fn listen() {}
}

impl Profile {
    pub fn name(&self) -> String {
        if !self.name.trim().is_empty() {
            return String::from(self.name.trim());
        }

        env::var("USER").unwrap_or_default()
    }

    // the name as typed, empty when the login name is used
    pub fn typed_name(&self) -> &str {
        &self.name
    }

    // none until one is chosen, or when the file is gone
    pub fn picture(&self) -> Option<&PathBuf> {
        self.picture.as_ref().filter(|picture| picture.exists())
    }

    pub fn set_name(name: String) {
        let mut profile = Self::write();

        profile.name = name;

        profile.save();
    }

    // the portal's file picker waits on the user, so it runs on its own thread
    pub fn choose_picture() {
        thread::spawn(|| {
            let Some(picture) = portal::choose_image("Choose a profile picture") else {
                return;
            };

            let mut profile = Self::write();

            profile.picture = Some(picture);

            profile.save();
        });
    }

    // back to no picture, the lock screen shows an icon instead
    pub fn clear_picture() {
        let mut profile = Self::write();

        profile.picture = None;

        profile.save();
    }

    // losing the file only means the next start shows the login name and no picture
    fn save(&self) {
        let mut text = format!("name={}\n", self.name);

        if let Some(picture) = &self.picture {
            text.push_str(&format!("picture={}\n", picture.display()));
        }

        let path = path();

        let folder = path.parent().expect("failed to find the state folder");

        let _ = fs::create_dir_all(folder);
        let _ = fs::write(&path, text);
    }
}

fn path() -> PathBuf {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    PathBuf::from(format!("{home}/.local/state/amane/profile"))
}
