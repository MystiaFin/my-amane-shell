use amane::{
    Center, Color, Column, Image, Lock, Padding, Rectangle, Service, Start, Text, TextInput,
    Weight, children,
};

use crate::fonts;
use crate::profile::Profile;
use crate::theme;

pub const HEIGHT: f32 = 190.0;

const FIELD_WIDTH: f32 = 360.0;
const FIELD_HEIGHT: f32 = 50.0;

const AVATAR: f32 = 72.0;
const AVATAR_ICON: &str = "󰀄";

// the text input's name, which keeps what was typed between redraws
const INPUT: &str = "lock-password";

const WHITE: Color = Color::rgb(0xf7, 0xf7, 0xf7);
const FIELD: Color = Color::rgba(0x21, 0x25, 0x2b, 0xb0);
const ERROR: Color = Color::rgb(0xff, 0xb4, 0xab);

// whether anything is typed, which wakes the lock screen's password block
#[derive(Default)]
pub struct Typing {
    pub typing: bool,
}

impl Service for Typing {
    fn new() -> Self {
        Self::default()
    }

    // it only changes through input
    fn listen() {}
}

// the avatar, the user's name, the password field and what pam said
pub fn view(lock: &Lock) -> Column {
    let profile = Profile::read();

    let name = Text::new(profile.name())
        .size(17.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(WHITE);

    let message = if lock.failed() {
        "Wrong password, try again"
    } else {
        ""
    };

    let message = Rectangle::new()
        .width(FIELD_WIDTH)
        .height(18.0)
        .align_child(Center, Center)
        .child(Text::new(message).size(11.0).font(fonts::BODY).color(ERROR));

    Column::new(children![avatar(&profile), name, field(lock), message])
        .gap(10.0)
        .align(Center)
}

// the chosen picture in a circle, or a person icon until one is chosen and decoded
fn avatar(profile: &Profile) -> Rectangle {
    let circle = Rectangle::new()
        .width(AVATAR)
        .height(AVATAR)
        .radius(AVATAR / 2.0)
        .fill(Color::rgba(0, 0, 0, 0x4d))
        .clip();

    if let Some(picture) = profile.picture() {
        if Image::loaded(picture) {
            let pixels = AVATAR as u32 * 2;

            return circle.fill(Image::cover(picture).thumbnail(pixels, pixels));
        }
    }

    circle.align_child(Center, Center).child(
        Text::new(AVATAR_ICON)
            .size(34.0)
            .font(fonts::NERD)
            .tight()
            .color(WHITE),
    )
}

fn field(lock: &Lock) -> Rectangle {
    let placeholder = if lock.checking() {
        "Authenticating…"
    } else {
        "Password"
    };

    let input = TextInput::new(INPUT)
        .size(14.0)
        .color(WHITE)
        .placeholder(placeholder)
        .password()
        .focused()
        .on_change(|text| Typing::write().typing = !text.is_empty())
        .on_submit(submit);

    Rectangle::new()
        .width(FIELD_WIDTH)
        .height(FIELD_HEIGHT)
        .radius(16.0)
        .fill(FIELD)
        .border(1.0, theme::current().accent)
        .padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 16.0,
        })
        .align_child(Start, Center)
        .child(input)
}

// the field empties either way: a wrong password is typed again from scratch
fn submit(password: String) {
    if password.is_empty() {
        return;
    }

    Lock::unlock(&password);

    TextInput::set_text(INPUT, "");
}
