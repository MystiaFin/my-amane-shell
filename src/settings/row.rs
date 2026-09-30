use amane::{
    Center, Column, Padding, Rectangle, Row, SpaceBetween, Start, Text, Weight, Widget, children,
};

use crate::fonts;
use crate::theme::Theme;

const HEIGHT: f32 = 64.0;
const SIDE_PADDING: f32 = 16.0;

// one setting: its name and a line about it on the left, its control on the right
pub fn view(
    theme: &Theme,
    width: f32,
    title: &str,
    detail: &str,
    control: impl Widget + 'static,
) -> Rectangle {
    let labels = Column::new(children![
        Text::new(title)
            .size(15.0)
            .font(fonts::BODY)
            .weight(Weight::SemiBold)
            .color(theme.text),
        Text::new(detail).size(12.0).font(fonts::BODY).color(theme.muted_text),
    ])
    .gap(4.0);

    let row = Row::new(children![labels, control])
        .width(width - SIDE_PADDING * 2.0)
        .justify(SpaceBetween)
        .align(Center);

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(16.0)
        .fill(theme.hover_surface)
        .padding(Padding {
            top: 0.0,
            right: SIDE_PADDING,
            bottom: 0.0,
            left: SIDE_PADDING,
        })
        .align_child(Start, Center)
        .child(row)
}
