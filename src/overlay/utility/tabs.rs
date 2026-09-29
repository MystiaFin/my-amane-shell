use amane::{Center, Pointer, Rectangle, Row, Text, Widget};

use super::{Page, hover, hovered, select};
use crate::fonts;
use crate::motion::{self, FAST_SPATIAL};
use crate::overlay::Overlay;
use crate::theme::Theme;

pub const HEIGHT: f32 = 38.0;

// the chosen tab is wider, and round at the ends
const WIDTH: f32 = 44.0;
const ACTIVE_WIDTH: f32 = 56.0;

const RADIUS: f32 = 12.0;

struct Tab {
    page: Page,
    name: &'static str,
    icon: &'static str,
}

const TABS: [Tab; 3] = [
    Tab {
        page: Page::Notifications,
        name: "notifications",
        icon: "󰂚",
    },
    Tab {
        page: Page::Wifi,
        name: "wifi",
        icon: "󰖩",
    },
    Tab {
        page: Page::Bluetooth,
        name: "bluetooth",
        icon: "󰂯",
    },
];

pub fn view(overlay: &Overlay, theme: &Theme, width: f32) -> Row {
    let mut tabs: Vec<Box<dyn Widget>> = Vec::new();

    for tab in &TABS {
        tabs.push(Box::new(button(overlay, theme, tab)));
    }

    Row::new(tabs)
        .width(width)
        .height(HEIGHT)
        .gap(8.0)
        .justify(Center)
        .align(Center)
}

fn button(overlay: &Overlay, theme: &Theme, tab: &Tab) -> Rectangle {
    let active = overlay.page == tab.page;

    let hover_name = format!("tab:{}", tab.name);

    let (target_width, target_radius) = if active {
        (ACTIVE_WIDTH, HEIGHT / 2.0)
    } else {
        (WIDTH, RADIUS)
    };

    let width = motion::follow(&format!("{hover_name}:width"), target_width, FAST_SPATIAL);
    let radius = motion::follow(&format!("{hover_name}:radius"), target_radius, FAST_SPATIAL);

    let fill = if active {
        theme.accent
    } else if hovered(overlay, &hover_name) {
        theme.border
    } else {
        theme.surface
    };

    let icon_color = if active { theme.on_accent } else { theme.text };

    let page = tab.page;

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(radius)
        .fill(fill)
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(move |_| select(page))
        .align_child(Center, Center)
        .child(Text::new(tab.icon).size(18.0).font(fonts::NERD).color(icon_color))
}
