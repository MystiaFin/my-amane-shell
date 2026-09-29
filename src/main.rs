mod bar;
mod clock;
mod fonts;
mod theme;
mod wallpaper;

use amane::{App, Service};

use wallpaper::Wallpaper;

fn main() {
    // reading it once starts the palette before the first frame
    drop(Wallpaper::read());

    App::new().font(fonts::BODY).window_per_monitor(bar::view).run();
}
