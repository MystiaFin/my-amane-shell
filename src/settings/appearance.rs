use amane::{
    Center, Column, Padding, Rectangle, Row, Service, SpaceBetween, Start, Text, Weight, children,
};

use super::choice;
use crate::fonts;
use crate::theme::{Mode, Theme};

const ROW_HEIGHT: f32 = 64.0;

// light, dark, or following the system's preference
pub fn view(theme: &Theme, width: f32) -> Rectangle {
    let picked = Mode::read().light;

    let options = [("Auto", None), ("Light", Some(true)), ("Dark", Some(false))];

    let labels = Column::new(children![
        Text::new("Color mode").size(15.0).font(fonts::BODY).weight(Weight::SemiBold).color(theme.text),
        Text::new("Follow the system, or always use light or dark")
            .size(12.0)
            .font(fonts::BODY)
            .color(theme.muted_text),
    ])
    .gap(4.0);

    let control = choice::view(theme, "color-mode", &options, picked, Mode::set);

    let row = Row::new(children![labels, control]).width(width - 32.0).justify(SpaceBetween).align(Center);

    Rectangle::new()
        .width(width)
        .height(ROW_HEIGHT)
        .radius(16.0)
        .fill(theme.hover_surface)
        .padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 16.0,
        })
        .align_child(Start, Center)
        .child(row)
}
