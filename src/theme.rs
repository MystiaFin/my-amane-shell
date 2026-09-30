mod hsl;
mod mode;

use amane::{Color, Palette, Service};

use hsl::Hsl;

pub use mode::Mode;

// no tone gets more saturated than this
const MAX_SATURATION: f32 = 0.82;

// green and red to start from, pulled a little toward the accent
const SUCCESS_SEED: Color = Color::rgb(0xa6, 0xe3, 0xa1);
const DANGER_SEED: Color = Color::rgb(0xf3, 0x8b, 0xa8);

// the colors every component draws with, rebuilt from the wallpaper each frame
pub struct Theme {
    pub light: bool,

    pub background: Color,
    pub surface: Color,
    pub hover_surface: Color,
    pub selected_surface: Color,
    pub border: Color,

    pub text: Color,
    pub secondary_text: Color,
    pub muted_text: Color,

    pub accent: Color,
    pub on_accent: Color,

    pub success: Color,
    pub danger: Color,
}

// every color is the wallpaper's darkest or most vivid color, re-lit
pub fn current() -> Theme {
    let palette = Palette::read();

    // a choice made by hand wins, otherwise a bright wallpaper gets a light theme
    let light = Mode::read().light.unwrap_or(palette.light());

    let base = palette.background();
    let seed = palette.accent();

    let seed_saturation = hsl::from_color(seed).saturation;

    let background = pick(light, 0.94, 0.075);
    let surface = pick(light, 0.88, 0.12);

    let background = tone(base, background, pick(light, 0.08, 0.30));
    let surface = tone(base, surface, pick(light, 0.10, 0.28));

    let accent = tone(seed, pick(light, 0.42, 0.68), seed_saturation.max(0.58));

    let text = tone(base, pick(light, 0.10, 0.91), 0.10);
    let secondary_text = tone(base, pick(light, 0.30, 0.72), 0.14);
    let muted_text = tone(base, pick(light, 0.42, 0.52), 0.16);

    let border = mix(surface, accent, pick(light, 0.26, 0.32));
    let hover_surface = mix(surface, accent, pick(light, 0.12, 0.20));
    let selected_surface = mix(surface, accent, pick(light, 0.20, 0.15));

    let success = tone(mix(SUCCESS_SEED, accent, 0.20), pick(light, 0.42, 0.70), 0.48);
    let danger = tone(mix(DANGER_SEED, accent, 0.20), pick(light, 0.42, 0.70), 0.48);

    // on a light theme the bar's accent pill is darkened so its text stays readable
    let accent = if light {
        mix(accent, Color::BLACK, 0.22)
    } else {
        accent
    };

    let on_accent = if luminance(accent) > 0.179 {
        tone(base, 0.08, 0.12)
    } else {
        tone(base, 0.96, 0.08)
    };

    Theme {
        light,
        background,
        surface,
        hover_surface,
        selected_surface,
        border,
        text,
        secondary_text,
        muted_text,
        accent,
        on_accent,
        success,
        danger,
    }
}

fn pick(light: bool, when_light: f32, when_dark: f32) -> f32 {
    if light { when_light } else { when_dark }
}

// keeps the color's hue, sets its lightness, and keeps it at least a little colorful
fn tone(color: Color, lightness: f32, min_saturation: f32) -> Color {
    let original = hsl::from_color(color);

    let saturation = original.saturation.min(MAX_SATURATION).max(min_saturation);

    hsl::to_color(Hsl {
        hue: original.hue,
        saturation,
        lightness: lightness.clamp(0.02, 0.98),
    })
}

// amount 0 gives first, 1 gives second
pub fn mix(first: Color, second: Color, amount: f32) -> Color {
    let blend = |from: u8, to: u8| {
        let from = f32::from(from);
        let to = f32::from(to);

        (from + (to - from) * amount).round() as u8
    };

    Color::rgb(
        blend(first.red(), second.red()),
        blend(first.green(), second.green()),
        blend(first.blue(), second.blue()),
    )
}

// relative luminance, to choose dark or light text on the accent
fn luminance(color: Color) -> f32 {
    let linear = |channel: u8| {
        let value = f32::from(channel) / 255.0;

        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };

    linear(color.red()) * 0.2126 + linear(color.green()) * 0.7152 + linear(color.blue()) * 0.0722
}
