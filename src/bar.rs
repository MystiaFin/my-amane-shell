mod center;
mod motion;
mod pill;
mod star;
mod ring;
mod system;
mod workspaces;

use amane::{Full, Layer, LayerWindow, Monitor, Parent, Rectangle, Row, Vertical, Zone, children};

use crate::theme;

// jaqc's default statusBarHeight
pub const HEIGHT: f32 = 40.0;

// the height of every pill and ring row inside the bar
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
                .fill(theme.background)
                .child(sections),
        )
}
