mod about;
mod appearance;
mod bar;
mod behavior;
mod colors;
mod floating;
mod integrations;
mod launcher;
mod user;
mod wallpaper;
mod weather;

pub use user::NAME_INPUT;

use super::Page as Shown;
use super::page::Page;

// fills the page with the shown page's groups
pub fn build(shown: Shown, page: &mut Page) {
    match shown {
        Shown::User => user::build(page),
        Shown::Appearance => appearance::build(page),
        Shown::Colors => colors::build(page),
        Shown::Launcher => launcher::build(page),
        Shown::Wallpaper => wallpaper::build(page),
        Shown::Bar => bar::build(page),
        Shown::Behavior => behavior::build(page),
        Shown::Floating => floating::build(page),
        Shown::Weather => weather::build(page),
        Shown::Integrations => integrations::build(page),
        Shown::About => about::build(page),
    }
}
