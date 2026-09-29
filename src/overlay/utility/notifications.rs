use std::path::PathBuf;

use amane::{
    Apps, Center, Column, Image, Notification, Notifications, Padding, Parent, Pointer,
    Rectangle, Row, ScrollArea, Service, Size, SpaceBetween, Stack, Start, Text, Weight, Widget,
    children,
};

use super::{fade_target, hover};
use crate::clock::Clock;
use crate::fonts;
use crate::motion;
use crate::overlay::Overlay;
use crate::theme::{self, Theme};

const LIST: &str = "notifications";

const TITLE_HEIGHT: f32 = 30.0;

const GAP: f32 = 10.0;

const CARD_PADDING: Padding = Padding {
    top: 10.0,
    right: 36.0,
    bottom: 10.0,
    left: 12.0,
};

const CARD_RADIUS: f32 = 16.0;

const ICON_SIZE: f32 = 38.0;
const ICON_MARGIN: f32 = 7.0;
const ICON_GAP: f32 = 10.0;

const TEXT_GAP: f32 = 2.0;

const CLOSE_SIZE: f32 = 15.0;
const CLOSE_INSET: f32 = 12.0;

const BUTTON_HEIGHT: f32 = 26.0;
const BUTTON_PADDING: f32 = 9.0;
const BUTTON_GAP: f32 = 6.0;
const ACTIONS_GAP: f32 = 8.0;

const BELL_ICON: &str = "󰂚";
const CLOSE_ICON: &str = "󰅖";

pub fn view(overlay: &Overlay, theme: &Theme, width: f32, height: f32) -> Column {
    let notifications = Notifications::read();

    let list_height = height - TITLE_HEIGHT - GAP;

    Column::new(children![
        title_row(overlay, theme, &notifications, width),
        list(overlay, theme, &notifications, width, list_height),
    ])
    .gap(GAP)
}

fn title_row(overlay: &Overlay, theme: &Theme, notifications: &Notifications, width: f32) -> Row {
    let title = Text::new("Notifications")
        .size(15.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.text);

    let mut parts: Vec<Box<dyn Widget>> = vec![Box::new(title)];

    if !notifications.list().is_empty() {
        let hover_name = String::from("notifications:clear");

        let amount = motion::fade(&hover_name, fade_target(overlay, &hover_name));

        let clear = Text::new("Clear all")
            .size(11.0)
            .font(fonts::BODY)
            .color(theme::mix(theme.muted_text, theme.danger, amount));

        let clear = Rectangle::new()
            .width(size_of(&clear))
            .height(TITLE_HEIGHT)
            .align_child(Start, Center)
            .cursor(Pointer)
            .on_hover(move |inside| hover(hover_name.clone(), inside))
            .on_click(|_| Notifications::clear())
            .child(clear);

        parts.push(Box::new(clear));
    }

    Row::new(parts)
        .width(width)
        .height(TITLE_HEIGHT)
        .justify(SpaceBetween)
        .align(Center)
}

// newest on top
fn list(
    overlay: &Overlay,
    theme: &Theme,
    notifications: &Notifications,
    width: f32,
    height: f32,
) -> Rectangle {
    let area = Rectangle::new().width(width).height(height);

    if notifications.list().is_empty() {
        let empty = Text::new("No notifications")
            .size(13.0)
            .font(fonts::BODY)
            .color(theme.muted_text);

        return area.align_child(Center, Center).child(empty);
    }

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();
    let mut total = 0.0;

    for notification in notifications.list().iter().rev() {
        let (card, card_height) = card(overlay, theme, notification, width);

        total += card_height;

        cards.push(Box::new(card));
    }

    total += GAP * (cards.len() - 1) as f32;

    let column = Column::new(cards).width(width).height(total).gap(GAP);

    area.child(ScrollArea::new(LIST, column))
}

// the icon beside the text, the buttons under both, and a close button in the corner
fn card(overlay: &Overlay, theme: &Theme, notification: &Notification, width: f32) -> (Stack, f32) {
    let inner_width = width - CARD_PADDING.left - CARD_PADDING.right;

    let text_width = inner_width - ICON_SIZE - ICON_GAP;

    let (text, text_height) = text_column(theme, notification, text_width);

    let top_height = f32::max(ICON_SIZE, text_height);

    let top = Row::new(children![icon(theme, notification), text])
        .height(top_height)
        .gap(ICON_GAP);

    let mut rows: Vec<Box<dyn Widget>> = vec![Box::new(top)];

    let mut inner_height = top_height;

    if !notification.actions().is_empty() {
        rows.push(Box::new(actions(overlay, theme, notification)));

        inner_height += ACTIONS_GAP + BUTTON_HEIGHT;
    }

    let height = CARD_PADDING.top + inner_height + CARD_PADDING.bottom;

    let body = Rectangle::new()
        .width(width)
        .height(height)
        .radius(CARD_RADIUS)
        .fill(theme.surface)
        .padding(CARD_PADDING)
        .child(Column::new(rows).gap(ACTIONS_GAP));

    let mut layers: Vec<Box<dyn Widget>> = vec![Box::new(body)];

    // the top of the card runs its default action, the buttons below keep their own clicks
    if notification.has_default_action() {
        let id = notification.id();

        let click = Rectangle::new()
            .width(width)
            .height(CARD_PADDING.top + top_height)
            .cursor(Pointer)
            .on_click(move |_| Notifications::click(id));

        layers.push(Box::new(click));
    }

    layers.push(Box::new(close_button(overlay, theme, notification.id(), width)));

    (Stack::new(layers).width(width).height(height), height)
}

// the summary, up to two lines of body, then which app sent it and when
fn text_column(theme: &Theme, notification: &Notification, width: f32) -> (Column, f32) {
    let summary = Text::new(notification.summary())
        .size(13.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.text)
        .elide();

    let received = Clock::read().hours_minutes(notification.received());

    let source = Text::new(format!("{}  •  {received}", notification.app_name()))
        .size(9.0)
        .font(fonts::BODY)
        .color(theme.secondary_text)
        .elide();

    let mut parts: Vec<(Text, f32)> = Vec::new();

    parts.push(fitted(summary, width));

    let body = plain_text(notification.body());

    if !body.trim().is_empty() {
        let body = Text::new(body)
            .size(11.0)
            .font(fonts::BODY)
            .color(theme.muted_text)
            .wrap()
            .max_lines(2)
            .elide();

        parts.push(fitted(body, width));
    }

    parts.push(fitted(source, width));

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();
    let mut total = 0.0;

    for (text, height) in parts {
        total += height;

        rows.push(Box::new(Rectangle::new().width(width).height(height).child(text)));
    }

    total += TEXT_GAP * (rows.len() - 1) as f32;

    (Column::new(rows).width(width).height(total).gap(TEXT_GAP), total)
}

fn fitted(text: Text, width: f32) -> (Text, f32) {
    let height = text.height_in(width);

    (text, height)
}

// the sender's icon when one can be found, a bell otherwise
fn icon(theme: &Theme, notification: &Notification) -> Rectangle {
    let slot = Rectangle::new()
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .radius(8.0)
        .fill(theme.selected_surface)
        .align_child(Center, Center);

    let Some(path) = icon_path(notification) else {
        let bell = Text::new(BELL_ICON)
            .size(17.0)
            .font(fonts::NERD)
            .tight()
            .color(theme.accent);

        return slot.child(bell);
    };

    let inner = ICON_SIZE - ICON_MARGIN * 2.0;

    // twice the size, so it stays sharp on a scaled screen
    let pixels = (inner * 2.0) as u32;

    slot.child(
        Rectangle::new()
            .width(inner)
            .height(inner)
            .fill(Image::contain(path).thumbnail(pixels, pixels)),
    )
}

fn icon_path(notification: &Notification) -> Option<PathBuf> {
    let path = find_icon(notification)?;

    // themes also hold svg icons, which amane can't draw, so the bell stands in
    let extension = path.extension()?.to_str()?.to_lowercase();

    if ["png", "jpg", "jpeg"].contains(&extension.as_str()) {
        Some(path)
    } else {
        None
    }
}

/*
 * the icon is a file path, a file url, or a name from the icon theme; a
 * name is looked up through the installed apps, and with no icon at all
 * the app sending it may still have one
 */
fn find_icon(notification: &Notification) -> Option<PathBuf> {
    let icon = notification.icon();

    let icon = icon.strip_prefix("file://").unwrap_or(icon);

    if icon.starts_with('/') {
        return Some(PathBuf::from(icon));
    }

    let apps = Apps::read();

    for app in apps.list() {
        let same_icon = !icon.is_empty() && app.icon() == Some(icon);

        let same_app = icon.is_empty() && app.name().eq_ignore_ascii_case(notification.app_name());

        if same_icon || same_app {
            return app.icon_path().map(PathBuf::from);
        }
    }

    None
}

// one button for each action the sender offered
fn actions(overlay: &Overlay, theme: &Theme, notification: &Notification) -> Rectangle {
    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();

    for action in notification.actions() {
        let id = notification.id();
        let key = String::from(action.key());

        let hover_name = format!("notification:{id}:{key}");

        let amount = motion::fade(&hover_name, fade_target(overlay, &hover_name));

        let label = Text::new(action.label())
            .size(10.0)
            .font(fonts::BODY)
            .weight(Weight::SemiBold)
            .color(theme.text);

        let button = Rectangle::new()
            .width(size_of(&label) + BUTTON_PADDING * 2.0)
            .height(BUTTON_HEIGHT)
            .radius(8.0)
            .fill(theme::mix(theme.border, theme.selected_surface, amount))
            .align_child(Center, Center)
            .cursor(Pointer)
            .on_hover(move |inside| hover(hover_name.clone(), inside))
            .on_click(move |_| Notifications::invoke(id, &key))
            .child(label);

        buttons.push(Box::new(button));
    }

    let buttons = Row::new(buttons).gap(BUTTON_GAP);

    // lined up with the text, past the icon
    Rectangle::new()
        .width(Parent)
        .height(BUTTON_HEIGHT)
        .padding(Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: ICON_SIZE + ICON_GAP,
        })
        .child(buttons)
}

fn close_button(overlay: &Overlay, theme: &Theme, id: u32, card_width: f32) -> Rectangle {
    let hover_name = format!("notification:{id}:close");

    let amount = motion::fade(&hover_name, fade_target(overlay, &hover_name));

    let glyph = Text::new(CLOSE_ICON)
        .size(15.0)
        .font(fonts::NERD)
        .tight()
        .color(theme::mix(theme.muted_text, theme.danger, amount));

    Rectangle::new()
        .width(CLOSE_SIZE)
        .height(CLOSE_SIZE)
        .translate(card_width - CLOSE_INSET - CLOSE_SIZE, CLOSE_INSET)
        .align_child(Center, Center)
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(move |_| Notifications::dismiss(id))
        .child(glyph)
}

// bodies may hold simple markup like <b>, which is left out
fn plain_text(body: &str) -> String {
    let mut text = String::new();
    let mut inside_tag = false;

    for character in body.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => text.push(character),
            _ => {}
        }
    }

    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn size_of(text: &Text) -> f32 {
    match text.width() {
        Size::Fixed(pixels) => pixels,
        Size::Parent => 0.0,
    }
}
