mod center;
mod motion;
mod pill;
mod star;
mod ring;
mod system;
mod workspaces;

use amane::{
    Color, Full, Layer, LayerWindow, Monitor, Parent, Rectangle, Row, Stack, Vertical, Zone, children,
};

use crate::overlay::Overlay;
use crate::theme;

pub const HEIGHT: f32 = 40.0;

const CORNER_RADIUS: f32 = 16.0;

pub const ITEM_HEIGHT: f32 = 26.0;

pub fn view(monitor: &Monitor) -> LayerWindow {
    let theme = theme::current();

    // three equal thirds, so the middle section stays centered whatever the sides hold
    let third = monitor.width as f32 / 3.0;

    let sections = Row::new(children![
        workspaces::view(monitor, &theme, third),
        center::view(&theme, third),
        system::view(&theme, third),
    ]);

    // taller than the bar and clipped, so only its top corners come out rounded
    let background = Rectangle::new()
        .width(Parent)
        .height(HEIGHT + CORNER_RADIUS)
        .radius(CORNER_RADIUS)
        .fill(theme.background);

    let content = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .on_hover(Overlay::hover_bar)
        .child(sections);

    // black behind the rounded corners, so the screen's top corners look rounded too
    LayerWindow::new()
        .width(Full)
        .height(HEIGHT)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(Color::BLACK)
                .clip()
                .child(Stack::new(children![background, content]).width(Parent).height(Parent)),
        )
}
