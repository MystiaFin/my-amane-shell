use std::env;
use std::fs;
use std::hash::{BuildHasher, RandomState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use amane::{Image, Palette, Service};

use crate::motion::{self, Glide};

// the wallpaper picker writes the chosen file here
const SELECTION: &str = ".config/quickshell/wallpaper-selection";

const PALETTE_SIZE: usize = 16;

// every wallpaper animation takes this long, like the quickshell config's wallpaper group
pub const DURATION: u64 = 1200;

// a file that never decodes is shown anyway after this long, as nothing
const LOAD_TIMEOUT: Duration = Duration::from_secs(3);

const LOAD_POLL: Duration = Duration::from_millis(16);

/*
 * set by the view once it draws the decoded wallpaper; the window takes a
 * moment to reach the screen at startup, and the intro would play unseen
 */
pub static DRAWN: AtomicBool = AtomicBool::new(false);

/*
 * the chosen wallpaper and the one on screen: a new choice waits until
 * its file is decoded, then grows over the old one as a circle
 */
pub struct Wallpaper {
    // the chosen file, which the palette is made from
    pub path: String,

    // the file on screen, and the one growing over it
    pub shown: String,
    pub incoming: Option<String>,

    // where the circle grows from, as a share of the screen's width and height
    pub center: (f32, f32),

    // how far the circle has grown, 0 to 1
    pub reveal: Glide,

    // at startup the backdrop fades in, then the wallpaper rises from below, both 0 to 1
    pub backdrop: Glide,
    pub rise: Glide,
}

impl Service for Wallpaper {
    fn new() -> Self {
        let path = read_selection();

        Palette::write().open(&path, PALETTE_SIZE);

        Self {
            shown: path.clone(),
            path,
            incoming: None,
            center: (0.5, 0.5),
            reveal: motion::spatial(0.0, DURATION),
            backdrop: motion::effects(0.0, DURATION),
            rise: motion::spatial(0.0, DURATION),
        }
    }

    fn listen() {
        // made first, so a choice made during the intro is still seen
        let changes = amane::watch_file(&selection_file());

        play_intro();

        for _ in changes {
            let path = read_selection();

            if path == Self::read().path {
                continue;
            }

            Palette::write().open(&path, PALETTE_SIZE);

            Self::write().path = path.clone();

            reveal(path);
        }
    }
}

fn play_intro() {
    let started = Instant::now();

    while !DRAWN.load(Ordering::Relaxed) && started.elapsed() < LOAD_TIMEOUT {
        thread::sleep(LOAD_POLL);
    }

    Wallpaper::write().backdrop.to(1.0);

    thread::sleep(Duration::from_millis(DURATION));

    Wallpaper::write().rise.to(1.0);
}

// the circle grows over the old wallpaper, which is replaced once it covers everything
fn reveal(path: String) {
    wait_until_loaded(&path);

    {
        let mut wallpaper = Wallpaper::write();

        wallpaper.incoming = Some(path.clone());
        wallpaper.center = (random_share(), random_share());
        wallpaper.reveal = motion::spatial(0.0, DURATION);

        wallpaper.reveal.to(1.0);
    }

    thread::sleep(Duration::from_millis(DURATION));

    let mut wallpaper = Wallpaper::write();

    wallpaper.shown = path;
    wallpaper.incoming = None;
}

fn wait_until_loaded(path: &str) {
    let started = Instant::now();

    while !Image::loaded(path) && started.elapsed() < LOAD_TIMEOUT {
        thread::sleep(LOAD_POLL);
    }
}

// std seeds every RandomState differently, which is all the randomness this needs
fn random_share() -> f32 {
    let random = RandomState::new().hash_one(0);

    (random % 1000) as f32 / 1000.0
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

// the service sees the file change and reveals the new wallpaper
pub fn choose(path: &str) {
    let url = format!("file://{path}");

    fs::write(selection_file(), url).expect("failed to write wallpaper selection");
}
