use amane::{Image, Service};

use super::Wallpaper;
use crate::settings::Settings;

/*
 * the wallpaper is kept this many times smaller, blurred once while it
 * decodes, and stretched back up; a real blur of the area behind each
 * frame took about 45 ms on the laptop's intel gpu
 */
const SHRINK: f32 = 12.0;

// in the small copy's pixels, so about SHRINK times wider on screen
const RADIUS: u32 = 2;

// the shown wallpaper blurred, to fill a screen sized rectangle behind glass
pub fn blurred(screen_width: f32, screen_height: f32) -> Image {
    let strength = Settings::read().number("blur_strength");

    // a weaker blur keeps a bigger copy, none keeps it sharp
    let shrink = 1.0 + (SHRINK - 1.0) * strength;

    let radius = if strength > 0.0 { RADIUS } else { 0 };

    let width = (screen_width / shrink) as u32;
    let height = (screen_height / shrink) as u32;

    Image::cover(&Wallpaper::read().shown)
        .thumbnail(width, height)
        .blurred(radius)
}
