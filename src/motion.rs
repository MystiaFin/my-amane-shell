mod follow;
mod glide;
mod spring;

use std::time::Duration;

pub use follow::{fade, follow};
pub use glide::Glide;
pub use spring::Spring;

use amane::Service;

use crate::settings::Settings;

// leaves quickly and settles very softly, for anything that moves across the screen
const SPATIAL: [f32; 4] = [0.2, 0.0, 0.0, 1.0];

// for colors and opacity, quick to start so a hover answers at once
const EFFECTS: [f32; 4] = [0.34, 0.8, 0.34, 1.0];

pub const FAST_EFFECTS: u64 = 100;
pub const DEFAULT_EFFECTS: u64 = 140;
pub const FAST_SPATIAL: u64 = 240;
pub const DEFAULT_SPATIAL: u64 = 340;

// damped exactly enough to stop without overshooting, and a little firmer when closing
const PANEL_STIFFNESS: f32 = 500.0;
pub const PANEL_OPEN_DAMPING: f32 = 44.72;
pub const PANEL_CLOSE_DAMPING: f32 = 50.0;

const SIZE_STIFFNESS: f32 = 250.0;
const SIZE_DAMPING: f32 = 31.62;

pub fn spatial(value: f32, milliseconds: u64) -> Glide {
    Glide::new(value, Duration::from_millis(milliseconds), SPATIAL)
}

pub fn effects(value: f32, milliseconds: u64) -> Glide {
    Glide::new(value, Duration::from_millis(milliseconds), EFFECTS)
}

// how far a panel is out, from 0 to 1
pub fn panel() -> Spring {
    Spring::new(0.0)
        .stiffness(PANEL_STIFFNESS)
        .damping(PANEL_OPEN_DAMPING)
        .precision(0.0005)
}

// a size in pixels that follows its content
pub fn size(value: f32) -> Spring {
    Spring::new(value)
        .stiffness(SIZE_STIFFNESS)
        .damping(SIZE_DAMPING)
        .precision(0.1)
}

// how much faster than normal everything moves, from the settings
pub fn speed() -> f32 {
    Settings::read().number("animation_speed").max(0.1)
}

// with reduced motion every animation is already where it is going
pub fn reduced() -> bool {
    Settings::read().flag("reduce_motion")
}

// how long a wait on an animation really lasts, for threads that sleep through one
pub fn paced(milliseconds: u64) -> Duration {
    if reduced() {
        return Duration::ZERO;
    }

    Duration::from_millis(milliseconds).div_f32(speed())
}
