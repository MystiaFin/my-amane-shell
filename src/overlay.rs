mod panel;
pub mod power_menu;
mod state;

use amane::{Full, InputArea, Layer, LayerWindow, Monitor, Service, Zone};

use crate::liquid::{self, Blob};
use crate::theme;

pub use state::Overlay;

/*
 * one window over the whole screen that every panel grows out of;
 * it only takes input where a panel is, so the desktop below still works.
 * it stays mapped even with every panel away: mapping a window takes a
 * round trip with the compositor, long enough to miss a panel's opening
 */
pub fn view(_: &Monitor) -> LayerWindow {
    let theme = theme::current();

    let overlay = Overlay::read();

    let mut blobs: Vec<Blob> = Vec::new();
    let mut areas: Vec<InputArea> = Vec::new();

    if let Some((blob, area)) = power_menu::blob(&overlay, &theme) {
        blobs.push(blob);
        areas.push(area);
    }

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Top)
        .space(Zone::Respect)
        .input_region(areas)
        .child(liquid::view(theme.background, blobs))
}
