use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

use amane::{
    AccessPoint, Center, Color, Column, End, Network, Padding, Pointer, Rectangle, Row, ScrollArea,
    Service, Stack, Start, Text, TextInput, Weight, Widget, children,
};

use super::header::{self, Header};
use super::{close_password, hover, hovered};
use crate::fonts;
use crate::motion::{self, FAST_SPATIAL};
use crate::overlay::Overlay;
use crate::theme::{self, Theme};

// the password field's name, which keeps what was typed between redraws
const PASSWORD: &str = "wifi-password";

const LIST: &str = "wifi-networks";

const GAP: f32 = 10.0;
const CARD_GAP: f32 = 7.0;

const CARD_HEIGHT: f32 = 50.0;
const OPEN_CARD_HEIGHT: f32 = 104.0;
const CARD_PADDING: f32 = 9.0;

const ROW_HEIGHT: f32 = 32.0;
const FIELD_HEIGHT: f32 = 34.0;

// the icons for signal strength, strongest first, then the rest
const SIGNAL_ICONS: [&str; 4] = ["󰤨", "󰤥", "󰤢", "󰤟"];
const WIFI_ICON: &str = "󰖩";
const DISCONNECT_ICON: &str = "󰖪";
const LOCK_ICON: &str = "󰌾";
const CONFIRM_ICON: &str = "󰄬";
const CLOSE_ICON: &str = "󰅖";
const SHOWN_ICON: &str = "󰈈";
const HIDDEN_ICON: &str = "󰈉";

// how long the switch waits for networkmanager to catch up before showing what it says
const SWITCH_PATIENCE: Duration = Duration::from_secs(4);

thread_local! {
    /*
     * the switch moves as soon as it is clicked, while networkmanager
     * takes a moment to turn the radio on or off
     */
    static WANTED: Cell<Option<(bool, Instant)>> = const { Cell::new(None) };

    // the password as typed so far, for the join button
    static TYPED: RefCell<String> = const { RefCell::new(String::new()) };
}

pub fn view(overlay: &Overlay, theme: &Theme, width: f32, height: f32) -> Column {
    let network = Network::read();

    let enabled = wifi_enabled(&network);

    let (status, status_color) = if !network.ssid().is_empty() {
        (network.ssid(), theme.success)
    } else {
        ("Not connected", theme.muted_text)
    };

    let title = if enabled { "Wi-Fi" } else { "Wi-Fi off" };

    let header = Header {
        name: "wifi",
        icon: WIFI_ICON,
        title,
        status,
        status_color,
        active: enabled,
    };

    let header = header::view(overlay, theme, header, width, move || switch_wifi(!enabled));

    let list_height = height - header::HEIGHT - GAP;

    let mut parts: Vec<Box<dyn Widget>> = vec![Box::new(header)];

    if enabled {
        parts.push(Box::new(list(overlay, theme, &network, width, list_height)));
    }

    Column::new(parts).gap(GAP)
}

fn wifi_enabled(network: &Network) -> bool {
    let actual = network.wifi_enabled();

    let Some((wanted, since)) = WANTED.get() else {
        return actual;
    };

    if wanted == actual || since.elapsed() > SWITCH_PATIENCE {
        WANTED.set(None);

        return actual;
    }

    wanted
}

fn switch_wifi(enabled: bool) {
    WANTED.set(Some((enabled, Instant::now())));

    Network::set_wifi(enabled);

    // the switch has already moved, so the window has to draw it
    drop(Overlay::write());
}

fn list(overlay: &Overlay, theme: &Theme, network: &Network, width: f32, height: f32) -> Rectangle {
    let area = Rectangle::new().width(width).height(height);

    if network.access_points().is_empty() {
        let empty = Text::new("No networks found")
            .size(12.0)
            .font(fonts::BODY)
            .color(theme.muted_text);

        return area.align_child(Center, Center).child(empty);
    }

    // the joined network first, the rest strongest first as they come
    let mut ordered: Vec<&AccessPoint> = Vec::new();

    for access_point in network.access_points() {
        if access_point.active() {
            ordered.insert(0, access_point);
        } else {
            ordered.push(access_point);
        }
    }

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();
    let mut total = 0.0;

    for access_point in ordered {
        let (card, card_height) = card(overlay, theme, network, access_point, width);

        total += card_height;

        cards.push(Box::new(card));
    }

    total += CARD_GAP * (cards.len() - 1) as f32;

    let column = Column::new(cards).width(width).height(total).gap(CARD_GAP);

    area.child(ScrollArea::new(LIST, column))
}

// one network, which opens a password field under it when it needs one
fn card(
    overlay: &Overlay,
    theme: &Theme,
    network: &Network,
    access_point: &AccessPoint,
    width: f32,
) -> (Stack, f32) {
    let ssid = access_point.ssid();

    let active = access_point.active();

    let open = !active && overlay.password_for.as_deref() == Some(ssid);

    let target = if open { OPEN_CARD_HEIGHT } else { CARD_HEIGHT };

    let height = motion::follow(&format!("wifi:{ssid}:height"), target, FAST_SPATIAL);

    let inner_width = width - CARD_PADDING * 2.0;

    let connecting = network.connecting() && overlay.joining.as_deref() == Some(ssid);

    let mut rows: Vec<Box<dyn Widget>> = vec![Box::new(top_row(
        overlay,
        theme,
        access_point,
        connecting,
        inner_width,
    ))];

    // only built while any of it shows
    if height > CARD_HEIGHT + 0.5 {
        rows.push(Box::new(password_row(overlay, theme, ssid, inner_width)));
    }

    let fill = if active {
        theme.selected_surface
    } else {
        theme.surface
    };

    let mut body = Rectangle::new()
        .width(width)
        .height(height)
        .radius(12.0)
        .fill(fill)
        .padding(Padding {
            top: CARD_PADDING,
            right: CARD_PADDING,
            bottom: CARD_PADDING,
            left: CARD_PADDING,
        })
        .child(Column::new(rows).gap(7.0));

    // a clip costs the gpu a layer per card, so only while the password row slides in or out
    let sliding = height > CARD_HEIGHT + 0.5 && height < OPEN_CARD_HEIGHT - 0.5;

    if sliding {
        body = body.clip();
    }

    // the whole top of the card joins, apart from the disconnect button on the joined one
    let click_width = if active { width - CARD_HEIGHT } else { width };

    let name = String::from(ssid);
    let secured = access_point.secured();
    let saved = access_point.saved();

    let mut click = Rectangle::new().width(click_width).height(CARD_HEIGHT);

    if !active {
        click = click
            .cursor(Pointer)
            .on_click(move |_| card_clicked(&name, secured, saved));
    }

    (Stack::new(children![body, click]), height)
}

fn top_row(
    overlay: &Overlay,
    theme: &Theme,
    access_point: &AccessPoint,
    connecting: bool,
    width: f32,
) -> Row {
    let active = access_point.active();

    let signal = Text::new(signal_icon(access_point.strength()))
        .size(17.0)
        .font(fonts::NERD)
        .tight()
        .color(theme.accent);

    let signal = Rectangle::new()
        .width(20.0)
        .height(ROW_HEIGHT)
        .align_child(Center, Center)
        .child(signal);

    let status = if active {
        "Connected"
    } else if connecting {
        "Connecting…"
    } else if access_point.saved() {
        "Saved"
    } else {
        "Available"
    };

    let (weight, status_color) = if active {
        (Weight::SemiBold, theme.success)
    } else {
        (Weight::Regular, theme.muted_text)
    };

    let name = Text::new(access_point.ssid())
        .size(12.0)
        .font(fonts::BODY)
        .weight(weight)
        .color(theme.text)
        .elide();

    let status = Text::new(status)
        .size(9.0)
        .font(fonts::BODY)
        .color(status_color);

    // what is left between the signal icon and the button on the right
    let text_width = width - 20.0 - ROW_HEIGHT - 9.0 * 2.0;

    let name = Rectangle::new().width(text_width).height(18.0).child(name);

    let text = Rectangle::new()
        .width(text_width)
        .height(ROW_HEIGHT)
        .align_child(Start, Center)
        .child(Column::new(children![name, status]));

    let action = if active {
        disconnect_button(overlay, theme, connecting)
    } else {
        let icon = if access_point.saved() {
            CONFIRM_ICON
        } else {
            LOCK_ICON
        };

        Rectangle::new()
            .width(ROW_HEIGHT)
            .height(ROW_HEIGHT)
            .align_child(End, Center)
            .child(
                Text::new(icon)
                    .size(14.0)
                    .font(fonts::NERD)
                    .tight()
                    .color(theme.muted_text),
            )
    };

    Row::new(children![signal, text, action])
        .gap(9.0)
        .align(Center)
}

fn disconnect_button(overlay: &Overlay, theme: &Theme, connecting: bool) -> Rectangle {
    let hover_name = String::from("wifi:disconnect");

    let amount = motion::fade(&hover_name, fade_target(overlay, &hover_name));

    let alpha = (amount * 255.0).round() as u8;

    let fill = Color::rgba(
        theme.danger.red(),
        theme.danger.green(),
        theme.danger.blue(),
        alpha,
    );

    let icon_color = theme::mix(theme.danger, theme.on_accent, amount);

    let button = Rectangle::new()
        .width(ROW_HEIGHT)
        .height(ROW_HEIGHT)
        .radius(ROW_HEIGHT / 2.0)
        .fill(fill)
        .align_child(Center, Center)
        .child(
            Text::new(DISCONNECT_ICON)
                .size(16.0)
                .font(fonts::NERD)
                .tight()
                .color(icon_color),
        );

    // waits for the last change to finish first
    if connecting {
        return button.opacity(0.4);
    }

    button
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(|_| Network::disconnect())
}

// the password, then cancel and join
fn password_row(overlay: &Overlay, theme: &Theme, ssid: &str, width: f32) -> Row {
    let field_width = width - FIELD_HEIGHT * 2.0 - 7.0 * 2.0;

    let name = String::from(ssid);

    let mut input = TextInput::new(PASSWORD)
        .size(11.0)
        .color(theme.text)
        .focused()
        .on_change(|password| TYPED.set(password))
        .on_submit(move |password| join(&name, &password));

    if !overlay.show_password {
        input = input.password();
    }

    let input = Rectangle::new()
        .width(field_width - 50.0)
        .height(FIELD_HEIGHT)
        .translate(10.0, 0.0)
        .align_child(Start, Center)
        .child(input);

    let reveal_icon = if overlay.show_password {
        SHOWN_ICON
    } else {
        HIDDEN_ICON
    };

    let reveal = Rectangle::new()
        .width(36.0)
        .height(FIELD_HEIGHT)
        .translate(field_width - 36.0, 0.0)
        .cursor(Pointer)
        .on_click(|_| {
            let mut overlay = Overlay::write();

            overlay.show_password = !overlay.show_password;
        })
        .align_child(Center, Center)
        .child(
            Text::new(reveal_icon)
                .size(14.0)
                .font(fonts::NERD)
                .tight()
                .color(theme.muted_text),
        );

    let field = Rectangle::new()
        .width(field_width)
        .height(FIELD_HEIGHT)
        .radius(8.0)
        .fill(theme.selected_surface)
        .border(1.0, theme.accent)
        .child(Stack::new(children![input, reveal]));

    let cancel = small_button(
        overlay,
        theme,
        "wifi:cancel",
        CLOSE_ICON,
        theme.danger,
        theme.border,
    )
    .on_click(|_| close_password(&mut Overlay::write()));

    let name = String::from(ssid);

    let confirm = small_button(
        overlay,
        theme,
        "wifi:join",
        CONFIRM_ICON,
        theme.success,
        theme.success,
    )
    .on_click(move |_| join(&name, &TYPED.take()));

    Row::new(children![field, cancel, confirm])
        .gap(7.0)
        .align(Center)
}

// a square icon button that takes on a color while the pointer is on it
fn small_button(
    overlay: &Overlay,
    theme: &Theme,
    name: &str,
    icon: &str,
    icon_color: Color,
    hover_fill: Color,
) -> Rectangle {
    let hover_name = String::from(name);

    let amount = motion::fade(&hover_name, fade_target(overlay, &hover_name));

    let fill = theme::mix(theme.selected_surface, hover_fill, amount);

    // a filled success button gets the accent's text color, like the reference hover
    let icon_color = if hover_fill == icon_color {
        theme::mix(icon_color, theme.on_accent, amount)
    } else {
        icon_color
    };

    Rectangle::new()
        .width(FIELD_HEIGHT)
        .height(FIELD_HEIGHT)
        .radius(8.0)
        .fill(fill)
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .align_child(Center, Center)
        .child(
            Text::new(icon)
                .size(14.0)
                .font(fonts::NERD)
                .tight()
                .color(icon_color),
        )
}

fn fade_target(overlay: &Overlay, name: &str) -> f32 {
    if hovered(overlay, name) { 1.0 } else { 0.0 }
}

fn signal_icon(strength: u8) -> &'static str {
    match strength {
        76.. => SIGNAL_ICONS[0],
        51..=75 => SIGNAL_ICONS[1],
        26..=50 => SIGNAL_ICONS[2],
        _ => SIGNAL_ICONS[3],
    }
}

// a saved or open network joins at once, any other asks for its password first
fn card_clicked(ssid: &str, secured: bool, saved: bool) {
    if saved || !secured {
        join(ssid, "");

        return;
    }

    let mut overlay = Overlay::write();

    overlay.password_for = Some(String::from(ssid));
    overlay.show_password = false;

    clear_password();
}

fn join(ssid: &str, password: &str) {
    let password = if password.is_empty() {
        None
    } else {
        Some(password)
    };

    Network::connect(ssid, password);

    let mut overlay = Overlay::write();

    overlay.joining = Some(String::from(ssid));

    close_password(&mut overlay);
}

pub fn clear_password() {
    TextInput::set_text(PASSWORD, "");

    TYPED.take();
}
