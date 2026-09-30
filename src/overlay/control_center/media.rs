use std::time::Duration;

use amane::{
    Center, Column, Image, Media, Padding, Pointer, Rectangle, Row, Service, Start, Text, Weight,
    Widget, children,
};

use super::{art, visualizer, wave};
use crate::fonts;
use crate::motion::{self, FAST_SPATIAL};
use crate::overlay::Overlay;
use crate::overlay::utility::{fade_target, hover};
use crate::theme::{self, Theme};

const RADIUS: f32 = 20.0;
const ART_RADIUS: f32 = 16.0;

const PADDING: f32 = 18.0;
const GAP: f32 = 16.0;
const LINE_GAP: f32 = 8.0;

const TITLE_HEIGHT: f32 = 24.0;
const ARTIST_HEIGHT: f32 = 18.0;
const TIME_WIDTH: f32 = 30.0;
const CONTROLS_HEIGHT: f32 = 48.0;

const BUTTON: f32 = 38.0;
const PLAY_BUTTON: f32 = 48.0;

const EMPTY_ICON: &str = "󰝚";
const PREVIOUS_ICON: &str = "󰒮";
const PLAY_ICON: &str = "󰐊";
const PAUSE_ICON: &str = "󰏤";
const NEXT_ICON: &str = "󰒭";

// the cover beside the title, artist, progress and playback buttons
pub fn view(overlay: &Overlay, theme: &Theme, width: f32, height: f32) -> Rectangle {
    let media = Media::read();

    let inner_width = width - PADDING * 2.0;
    let inner_height = height - PADDING * 2.0;

    let art_size = (inner_width * 0.36).clamp(112.0, 220.0).min(inner_height);

    let details_width = inner_width - art_size - GAP;

    let row = Row::new(children![
        cover(&media, theme, art_size),
        details(overlay, &media, theme, details_width, art_size),
    ])
    .gap(GAP)
    .align(Center);

    Rectangle::new()
        .width(width)
        .height(height)
        .radius(RADIUS)
        .fill(theme.surface)
        .padding(Padding {
            top: PADDING,
            right: PADDING,
            bottom: PADDING,
            left: PADDING,
        })
        .align_child(Start, Center)
        .child(row)
}

// a music note stands in until the cover is on disk and decoded
fn cover(media: &Media, theme: &Theme, size: f32) -> Rectangle {
    let slot = Rectangle::new()
        .width(size)
        .height(size)
        .radius(ART_RADIUS)
        .fill(theme.border)
        .clip();

    if let Some(path) = art::path(media.art_url()) {
        if Image::loaded(&path) {
            let pixels = size as u32;

            return slot.fill(Image::cover(path).thumbnail(pixels, pixels));
        }
    }

    slot.align_child(Center, Center)
        .child(Text::new(EMPTY_ICON).size(52.0).font(fonts::NERD).tight().color(theme.muted_text))
}

fn details(overlay: &Overlay, media: &Media, theme: &Theme, width: f32, height: f32) -> Column {
    let (title, artist) = if media.title().is_empty() {
        ("Nothing playing", "Open Spotify or another media player")
    } else {
        (media.title(), media.artist())
    };

    let title = line(title, width, TITLE_HEIGHT, 17.0, Weight::Bold, theme.text);
    let artist = line(artist, width, ARTIST_HEIGHT, 13.0, Weight::Regular, theme.muted_text);

    let fixed = TITLE_HEIGHT
        + ARTIST_HEIGHT
        + visualizer::HEIGHT
        + wave::HEIGHT
        + CONTROLS_HEIGHT
        + LINE_GAP * 5.0;

    let space = Rectangle::new().width(width).height((height - fixed).max(0.0));

    Column::new(children![
        title,
        artist,
        space,
        visualizer::view(theme, width),
        progress(media, theme, width),
        controls(overlay, media, theme, width),
    ])
    .gap(LINE_GAP)
}

// one line of text, cut off with an ellipsis when too long
fn line(text: &str, width: f32, height: f32, size: f32, weight: Weight, color: amane::Color) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(height)
        .align_child(Start, Center)
        .child(Text::new(text).size(size).font(fonts::BODY).weight(weight).color(color).elide())
}

fn progress(media: &Media, theme: &Theme, width: f32) -> Row {
    let position = media.position();
    let length = media.length();

    let played = if length.is_zero() {
        0.0
    } else {
        position.as_secs_f32() / length.as_secs_f32()
    };

    let wave_width = width - TIME_WIDTH * 2.0 - LINE_GAP * 2.0;

    Row::new(children![
        time(position, theme, Start),
        wave::view(wave_width, played, media.playing(), theme.accent, theme.border),
        time(length, theme, amane::End),
    ])
    .gap(LINE_GAP)
    .align(Center)
}

// "3:07"
fn time(duration: Duration, theme: &Theme, side: impl Into<amane::Align>) -> Rectangle {
    let seconds = duration.as_secs();

    let text = format!("{}:{:02}", seconds / 60, seconds % 60);

    Rectangle::new()
        .width(TIME_WIDTH)
        .height(wave::HEIGHT)
        .align_child(side, Center)
        .child(Text::new(text).size(10.0).font(fonts::BODY).color(theme.muted_text))
}

fn controls(overlay: &Overlay, media: &Media, theme: &Theme, width: f32) -> Row {
    let play_icon = if media.playing() { PAUSE_ICON } else { PLAY_ICON };

    let buttons: Vec<Box<dyn Widget>> = vec![
        Box::new(button(overlay, theme, "media:previous", PREVIOUS_ICON, Media::previous)),
        Box::new(play_button(media.playing(), theme, play_icon)),
        Box::new(button(overlay, theme, "media:next", NEXT_ICON, Media::next)),
    ];

    Row::new(buttons)
        .width(width)
        .height(CONTROLS_HEIGHT)
        .gap(10.0)
        .justify(Center)
        .align(Center)
}

// a round button that only shows a background under the pointer
fn button(overlay: &Overlay, theme: &Theme, name: &str, icon: &str, action: fn()) -> Rectangle {
    let hover_name = String::from(name);

    let amount = motion::fade(name, fade_target(overlay, name));

    let fill = theme::mix(amane::Color::TRANSPARENT, theme.border, amount);

    Rectangle::new()
        .width(BUTTON)
        .height(BUTTON)
        .radius(BUTTON / 2.0)
        .fill(fill)
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(move |_| action())
        .align_child(Center, Center)
        .child(Text::new(icon).size(18.0).font(fonts::NERD).tight().color(theme.text))
}

// round while paused, and squares off a little while playing
fn play_button(playing: bool, theme: &Theme, icon: &str) -> Rectangle {
    let target = if playing { 12.0 } else { PLAY_BUTTON / 2.0 };

    let radius = motion::follow("media:play-radius", target, FAST_SPATIAL);

    Rectangle::new()
        .width(PLAY_BUTTON)
        .height(PLAY_BUTTON)
        .radius(radius)
        .fill(theme.accent)
        .cursor(Pointer)
        .on_click(|_| Media::play_pause())
        .align_child(Center, Center)
        .child(Text::new(icon).size(21.0).font(fonts::NERD).tight().color(theme.on_accent))
}
