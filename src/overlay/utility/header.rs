use amane::{Center, Color, Column, Padding, Rectangle, Row, Start, Text, Weight, children};

use super::switch;
use crate::fonts;
use crate::overlay::Overlay;
use crate::theme::Theme;

pub const HEIGHT: f32 = 58.0;

const PADDING: f32 = 10.0;
const ICON_SIZE: f32 = 36.0;
const GAP: f32 = 10.0;

// what a page's header shows: its icon, a title, a status line and an on/off switch
pub struct Header<'a> {
    pub name: &'a str,
    pub icon: &'a str,
    pub title: &'a str,
    pub status: &'a str,
    pub status_color: Color,
    pub active: bool,
}

pub fn view(
    overlay: &Overlay,
    theme: &Theme,
    header: Header,
    width: f32,
    on_toggle: impl Fn() + 'static,
) -> Rectangle {
    let icon_fill = if header.active {
        theme.accent
    } else {
        theme.border
    };

    let icon = Rectangle::new()
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .radius(12.0)
        .fill(icon_fill)
        .align_child(Center, Center)
        .child(Text::new(header.icon).size(18.0).font(fonts::NERD).color(theme.on_accent));

    // what is left between the icon and the switch
    let text_width = width - PADDING * 2.0 - ICON_SIZE - switch::WIDTH - GAP * 2.0;

    let title = Text::new(header.title)
        .size(13.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.text);

    let status = Text::new(header.status)
        .size(10.0)
        .font(fonts::BODY)
        .color(header.status_color)
        .elide();

    let status = Rectangle::new().width(text_width).height(16.0).child(status);

    let text = Rectangle::new()
        .width(text_width)
        .height(ICON_SIZE)
        .align_child(Start, Center)
        .child(Column::new(children![title, status]));

    let switch = switch::view(overlay, theme, header.name, header.active, on_toggle);

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(16.0)
        .fill(theme.surface)
        .padding(Padding {
            top: PADDING,
            right: PADDING,
            bottom: PADDING,
            left: PADDING,
        })
        .child(Row::new(children![icon, text, switch]).gap(GAP).align(Center))
}
