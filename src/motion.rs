mod glide;
mod spring;

use std::time::Duration;

pub use glide::Glide;
pub use spring::Spring;

// leaves quickly and settles very softly, for anything that moves across the screen
const SPATIAL: [f32; 4] = [0.2, 0.0, 0.0, 1.0];

// damped exactly enough to stop without overshooting, and a little firmer when closing
const PANEL_STIFFNESS: f32 = 500.0;
pub const PANEL_OPEN_DAMPING: f32 = 44.72;
pub const PANEL_CLOSE_DAMPING: f32 = 50.0;

const SIZE_STIFFNESS: f32 = 250.0;
const SIZE_DAMPING: f32 = 31.62;

pub fn spatial(value: f32, milliseconds: u64) -> Glide {
    Glide::new(value, Duration::from_millis(milliseconds), SPATIAL)
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
