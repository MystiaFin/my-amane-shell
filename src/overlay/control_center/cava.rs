use std::env;
use std::thread;
use std::time::Duration;

use amane::{Media, Service};

use crate::overlay::Overlay;

// matches `bars` in cava.conf
pub const BARS: usize = 24;

// cava's quiet levels look flat, so they are lifted before being cut off at full
const GAIN: f32 = 1.75;

// how often cava checks whether it should start
const IDLE_CHECK: Duration = Duration::from_millis(200);

// how long to wait before starting cava again after it quit on its own
const RESTART_DELAY: Duration = Duration::from_secs(2);

// the loudness of each frequency band, 0 to 1, lowest first
#[derive(Default)]
pub struct Cava {
    levels: [f32; BARS],
}

/*
 * cava stays warm while something plays, so opening the panel shows bars
 * at once; levels are only stored while the panel is open, since every
 * store is a redraw
 */
impl Service for Cava {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {
        loop {
            while !wanted() {
                thread::sleep(IDLE_CHECK);
            }

            let output = amane::lines(&format!("cava -p '{}'", config_path()));

            // dropping the output stops cava
            for line in output {
                if !wanted() {
                    break;
                }

                if open() {
                    Self::write().levels = parse(&line);
                }
            }

            Self::write().levels = [0.0; BARS];

            if wanted() {
                thread::sleep(RESTART_DELAY);
            }
        }
    }
}

impl Cava {
    pub fn levels(&self) -> &[f32; BARS] {
        &self.levels
    }
}

fn open() -> bool {
    Overlay::read().control_center.shown
}

fn wanted() -> bool {
    open() || Media::read().playing()
}

// a line reads like "12;40;100;...;" with levels from 0 to 100
fn parse(line: &str) -> [f32; BARS] {
    let mut levels = [0.0; BARS];

    for (index, value) in line.split(';').take(BARS).enumerate() {
        let level: f32 = value.trim().parse().unwrap_or(0.0);

        levels[index] = (level / 100.0 * GAIN).clamp(0.0, 1.0);
    }

    levels
}

// the config sits next to the config's src folder
fn config_path() -> String {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    format!("{home}/.config/amane/cava.conf")
}
