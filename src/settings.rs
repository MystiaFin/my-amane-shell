mod about;
mod appearance;
mod bar;
mod behavior;
mod choice;
mod colors;
mod floating;
mod integrations;
mod launcher;
mod row;
mod slider;
mod store;
mod switch;
mod text;
mod user;
mod wallpaper;

use amane::{
    Center, Column, Padding, Parent, Pointer, Rectangle, Row, ScrollArea, Service, Start, Text,
    TextInput, Weight, Widget, Window, children,
};

pub use store::Settings;

use crate::fonts;
use crate::motion;
use crate::profile::Profile;
use crate::theme::{self, Theme};

const WIDTH: f32 = 980.0;
const HEIGHT: f32 = 680.0;

const NAVIGATION_WIDTH: f32 = 220.0;
const ITEM_HEIGHT: f32 = 40.0;

const PAGE_PADDING: f32 = 28.0;

const PAGE_WIDTH: f32 = WIDTH - NAVIGATION_WIDTH - PAGE_PADDING * 2.0;

const TITLE_HEIGHT: f32 = 30.0;
const TITLE_GAP: f32 = 20.0;

// what is left under the title, longer pages scroll inside it
const SCROLL_HEIGHT: f32 = HEIGHT - PAGE_PADDING * 2.0 - TITLE_HEIGHT - TITLE_GAP;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Appearance,
    Colors,
    Bar,
    Launcher,
    Wallpaper,
    Floating,
    Behavior,
    Integrations,
    About,
}

// the pages in the list on the left, in order
const PAGES: [(Page, &str, &str); 10] = [
    (Page::User, "\u{f0004}", "User"),
    (Page::Appearance, "\u{f0379}", "Appearance"),
    (Page::Colors, "\u{f03d8}", "Colors"),
    (Page::Bar, "\u{f0a6d}", "Bar"),
    (Page::Launcher, "\u{f0349}", "Launcher"),
    (Page::Wallpaper, "\u{f02e9}", "Wallpaper"),
    (Page::Floating, "\u{f0570}", "Desktop widgets"),
    (Page::Behavior, "\u{f0493}", "Behavior"),
    (Page::Integrations, "\u{f0431}", "Integrations"),
    (Page::About, "\u{f02fd}", "About"),
];

// which page the window shows
pub struct Shown {
    page: Page,
}

impl Service for Shown {
    fn new() -> Self {
        Self { page: Page::User }
    }

    // it only changes through input
    fn listen() {}
}

// the shell's settings, opened with open_window from the launcher or `amane ipc call settings`
pub fn view() -> Window {
    let theme = theme::current();

    let shown = Shown::read().page;

    let row = Row::new(children![navigation(&theme, shown), page(&theme, shown)]);

    Window::new()
        .title("Settings")
        .size(WIDTH, HEIGHT)
        .child(Rectangle::new().width(Parent).height(Parent).fill(theme.background).child(row))
}

fn navigation(theme: &Theme, shown: Page) -> Rectangle {
    let mut items: Vec<Box<dyn Widget>> = Vec::new();

    for (page, icon, label) in PAGES {
        let target = if page == shown { 1.0 } else { 0.0 };

        let amount = motion::fade(&format!("settings-page:{label}"), target);

        let fill = theme::mix(theme.surface, theme.selected_surface, amount);

        let item = Row::new(children![
            Text::new(icon).size(18.0).font(fonts::NERD).tight().color(theme.accent),
            Text::new(label).size(14.0).font(fonts::BODY).weight(Weight::Medium).color(theme.text),
        ])
        .gap(12.0)
        .align(Center);

        let item = Rectangle::new()
            .width(NAVIGATION_WIDTH - 24.0)
            .height(ITEM_HEIGHT)
            .radius(12.0)
            .fill(fill)
            .padding(Padding {
                top: 0.0,
                right: 14.0,
                bottom: 0.0,
                left: 14.0,
            })
            .cursor(Pointer)
            .on_click(move |_| Shown::write().page = page)
            .align_child(Start, Center)
            .child(item);

        items.push(Box::new(item));
    }

    Rectangle::new()
        .width(NAVIGATION_WIDTH)
        .height(Parent)
        .fill(theme.surface)
        .padding(Padding {
            top: 16.0,
            right: 12.0,
            bottom: 16.0,
            left: 12.0,
        })
        .child(Column::new(items).gap(4.0))
}

fn page(theme: &Theme, shown: Page) -> Rectangle {
    let content: Box<dyn Widget> = match shown {
        Page::User => Box::new(user::view(theme, PAGE_WIDTH)),
        Page::Appearance => Box::new(appearance::view(theme, PAGE_WIDTH)),
        Page::Colors => Box::new(colors::view(theme, PAGE_WIDTH)),
        Page::Bar => Box::new(bar::view(theme, PAGE_WIDTH)),
        Page::Launcher => Box::new(launcher::view(theme, PAGE_WIDTH)),
        Page::Wallpaper => Box::new(wallpaper::view(theme, PAGE_WIDTH)),
        Page::Floating => Box::new(floating::view(theme, PAGE_WIDTH)),
        Page::Behavior => Box::new(behavior::view(theme, PAGE_WIDTH)),
        Page::Integrations => Box::new(integrations::view(theme, PAGE_WIDTH)),
        Page::About => Box::new(about::view(theme, PAGE_WIDTH)),
    };

    // the page's label from the list doubles as its title and its scroll position's name
    let mut title = "";

    for (page, _, label) in PAGES {
        if page == shown {
            title = label;
        }
    }

    let content = ScrollArea::new(title, Column::new(vec![content]))
        .width(PAGE_WIDTH)
        .height(SCROLL_HEIGHT);

    let title = Text::new(title)
        .size(22.0)
        .font(fonts::BODY)
        .weight(Weight::Bold)
        .color(theme.text);

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .padding(Padding {
            top: PAGE_PADDING,
            right: PAGE_PADDING,
            bottom: PAGE_PADDING,
            left: PAGE_PADDING,
        })
        .child(Column::new(children![title, content]).gap(TITLE_GAP))
}

// opens the window, or does nothing while it is open; it closes from its title bar
pub fn open() {
    // the name field starts with what was saved, not with what was typed last time
    let typed_name = String::from(Profile::read().typed_name());

    TextInput::set_text(user::NAME_INPUT, &typed_name);

    text::fill("wallpaper_folder");

    amane::open_window(view);
}

pub fn ipc(_arguments: &[String]) -> String {
    open();

    String::from("ok")
}

// opens the window on one page, like colors from the launcher's color command
pub fn open_on(page: Page) {
    Shown::write().page = page;

    open();
}
