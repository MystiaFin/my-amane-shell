use amane::{
    Center, Column, Image, Padding, Pointer, Rectangle, Row, Service, Start, Text, TextInput,
    Weight, children,
};

use super::row;
use crate::fonts;
use crate::profile::Profile;
use crate::theme::Theme;

const AVATAR: f32 = 44.0;
const AVATAR_ICON: &str = "󰀄";

const NAME_WIDTH: f32 = 260.0;

// the text input's name, which keeps what was typed between redraws
pub const NAME_INPUT: &str = "profile-name";

// the picture and name the lock screen shows
pub fn view(theme: &Theme, width: f32) -> Column {
    let profile = Profile::read();

    let picture = row::view(
        theme,
        width,
        "Profile picture",
        "A png or jpeg, shown on the lock screen",
        Row::new(children![avatar(&profile, theme), choose_button(theme)])
            .gap(14.0)
            .align(Center),
    );

    let name = row::view(
        theme,
        width,
        "Display name",
        "Empty uses your login name",
        name_field(theme, &profile.name()),
    );

    Column::new(children![picture, name]).gap(12.0)
}

fn avatar(profile: &Profile, theme: &Theme) -> Rectangle {
    let circle = Rectangle::new()
        .width(AVATAR)
        .height(AVATAR)
        .radius(AVATAR / 2.0)
        .fill(theme.selected_surface)
        .clip();

    if let Some(picture) = profile.picture() {
        if Image::loaded(picture) {
            let pixels = AVATAR as u32 * 2;

            return circle.fill(Image::cover(picture).thumbnail(pixels, pixels));
        }
    }

    circle
        .align_child(Center, Center)
        .child(Text::new(AVATAR_ICON).size(22.0).font(fonts::NERD).tight().color(theme.text))
}

fn choose_button(theme: &Theme) -> Rectangle {
    Rectangle::new()
        .width(132.0)
        .height(34.0)
        .radius(12.0)
        .fill(theme.accent)
        .cursor(Pointer)
        .on_click(|_| Profile::choose_picture())
        .align_child(Center, Center)
        .child(
            Text::new("Choose picture")
                .size(13.0)
                .font(fonts::BODY)
                .weight(Weight::Medium)
                .color(theme.on_accent),
        )
}

fn name_field(theme: &Theme, login_name: &str) -> Rectangle {
    let input = TextInput::new(NAME_INPUT)
        .size(13.0)
        .color(theme.text)
        .placeholder(login_name)
        .on_change(Profile::set_name);

    Rectangle::new()
        .width(NAME_WIDTH)
        .height(34.0)
        .radius(12.0)
        .fill(theme.surface)
        .border(1.0, theme.border)
        .padding(Padding {
            top: 0.0,
            right: 12.0,
            bottom: 0.0,
            left: 12.0,
        })
        .align_child(Start, Center)
        .child(input)
}
