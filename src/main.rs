mod bar;
mod clock;
mod fonts;
mod liquid;
mod motion;
mod overlay;
mod theme;
mod wallpaper;

use amane::{App, Apps, Service};

use wallpaper::Wallpaper;

fn main() {
    // reading it once starts the palette before the first frame
    drop(Wallpaper::read());

    // the app list is read once up front, so the launcher opens without waiting for it
    drop(Apps::read());

    App::new()
        .font(fonts::BODY)
        .window_per_monitor(bar::view)
        .window_per_monitor(overlay::view)
        .ipc("launcher", overlay::launcher::ipc)
        .ipc("utility", overlay::utility::ipc)
        .run();
}
