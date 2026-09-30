mod appearance;
mod choice;

use amane::{
    Center, Column, Padding, Parent, Rectangle, Row, Start, Text, Weight, Widget, Window, children,
};

use crate::fonts;
use crate::theme::{self, Theme};

const WIDTH: f32 = 980.0;
const HEIGHT: f32 = 680.0;

const NAVIGATION_WIDTH: f32 = 220.0;
const ITEM_HEIGHT: f32 = 40.0;

const PAGE_PADDING: f32 = 28.0;

// the pages in the list on the left; only one so far
const PAGES: [(&str, &str); 1] = [("\u{f0379}", "Appearance")];

// the shell's settings, opened with open_window from the launcher or `amane ipc call settings`
pub fn view() -> Window {
    let theme = theme::current();

    let row = Row::new(children![navigation(&theme), page(&theme)]);

    Window::new()
        .title("Settings")
        .size(WIDTH, HEIGHT)
        .child(Rectangle::new().width(Parent).height(Parent).fill(theme.background).child(row))
}

fn navigation(theme: &Theme) -> Rectangle {
    let mut items: Vec<Box<dyn Widget>> = Vec::new();

    for (icon, label) in PAGES {
        let item = Row::new(children![
            Text::new(icon).size(18.0).font(fonts::NERD).tight().color(theme.accent),
            Text::new(label).size(14.0).font(fonts::BODY).weight(Weight::Medium).color(theme.text),
        ])
        .gap(12.0)
        .align(Center);

        // the only page is always the one shown
        let item = Rectangle::new()
            .width(NAVIGATION_WIDTH - 24.0)
            .height(ITEM_HEIGHT)
            .radius(12.0)
            .fill(theme.selected_surface)
            .padding(Padding {
                top: 0.0,
                right: 14.0,
                bottom: 0.0,
                left: 14.0,
            })
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

fn page(theme: &Theme) -> Rectangle {
    let title = Text::new("Appearance")
        .size(22.0)
        .font(fonts::BODY)
        .weight(Weight::Bold)
        .color(theme.text);

    let width = WIDTH - NAVIGATION_WIDTH - PAGE_PADDING * 2.0;

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .padding(Padding {
            top: PAGE_PADDING,
            right: PAGE_PADDING,
            bottom: PAGE_PADDING,
            left: PAGE_PADDING,
        })
        .child(Column::new(children![title, appearance::view(theme, width)]).gap(20.0))
}

// opens the window, or does nothing while it is open; it closes from its title bar
pub fn ipc(_arguments: &[String]) -> String {
    amane::open_window(view);

    String::from("ok")
}
