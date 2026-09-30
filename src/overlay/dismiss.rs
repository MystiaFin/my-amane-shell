use amane::{Color, Full, Layer, LayerWindow, Monitor, Parent, Rectangle, Service, Zone};

use super::{Overlay, utility};
use crate::settings::Settings;

/*
 * a clear window over the whole screen, under the panels, while one is open
 * and click outside to dismiss is on; a click on it closes every panel.
 * it draws nothing, so unlike the liquid it costs nothing to cover the screen
 */
pub fn view(_monitor: &Monitor) -> LayerWindow {
    let overlay = Overlay::read();

    let open = overlay.power_menu.shown
        || overlay.control_center.shown
        || overlay.launcher.shown
        || overlay.utility.shown;

    drop(overlay);

    let catching = open && Settings::read().flag("click_outside_dismiss");

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Top)
        .space(Zone::Respect)
        .namespace("dismiss")
        .visible(catching)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(Color::TRANSPARENT)
                .on_click(|_| close_all()),
        )
}

fn close_all() {
    let mut overlay = Overlay::write();

    overlay.power_menu.hide();
    overlay.control_center.hide();
    overlay.launcher.hide();

    utility::close(&mut overlay);
}
