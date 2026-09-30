mod appearance;
mod choice;
mod row;
mod user;

use amane::{
    Center, Column, Padding, Parent, Pointer, Rectangle, Row, Service, Start, Text, TextInput,
    Weight, Widget, Window, children,
};

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

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    User,
    Appearance,
}

// the pages in the list on the left, in order
const PAGES: [(Page, &str, &str); 2] = [
    (Page::User, "\u{f0004}", "User"),
    (Page::Appearance, "\u{f0379}", "Appearance"),
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
    let (title, content): (&str, Box<dyn Widget>) = match shown {
        Page::User => ("User", Box::new(user::view(theme, PAGE_WIDTH))),
        Page::Appearance => ("Appearance", Box::new(appearance::view(theme, PAGE_WIDTH))),
    };

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
        .child(Column::new(vec![Box::new(title), content]).gap(20.0))
}

// opens the window, or does nothing while it is open; it closes from its title bar
pub fn open() {
    // the name field starts with what was saved, not with what was typed last time
    let typed_name = String::from(Profile::read().typed_name());

    TextInput::set_text(user::NAME_INPUT, &typed_name);

    amane::open_window(view);
}

pub fn ipc(_arguments: &[String]) -> String {
    open();

    String::from("ok")
}
