use amane::{
    Battery, Center, Color, End, Memory, Parent, Rectangle, Row, Service, SpaceBetween, Text,
    Widget, children,
};

use super::{pill, ring};
use crate::fonts;
use crate::theme::Theme;

// jaqc's Icons.memory, and its notification, wifi and bluetooth tray icons
const MEMORY_ICON: &str = "󰍛";
const TRAY_ICONS: [&str; 3] = ["󰂚", "󰖩", "󰂯"];

// jaqc's resource rings are 24px with a 3px line
const RING_SIZE: f32 = 24.0;
const RING_THICKNESS: f32 = 3.0;

// measured from jaqc's bar: the tray pill is 70px, 50px of icons and 10px padding each side
const TRAY_WIDTH: f32 = 70.0;
const TRAY_PADDING: f32 = 10.0;

// with the row's 10px gap this makes jaqc's 15px from the screen edge
const EDGE: f32 = 5.0;

pub fn view(theme: &Theme, width: f32) -> Row {
    let mut items: Vec<Box<dyn Widget>> = Vec::new();

    if let Some(battery) = battery(theme) {
        items.push(Box::new(battery));
    }

    items.push(Box::new(memory(theme)));

    items.push(Box::new(tray(theme)));

    items.push(Box::new(Rectangle::new().width(EDGE).height(1.0)));

    Row::new(items)
        .width(width)
        .height(Parent)
        .gap(10.0)
        .justify(End)
        .align(Center)
}

// jaqc's StatusResourceIndicator: a ring with a small icon inside, then the value
fn indicator(value: f32, color: Color, icon: &str, text: &str, theme: &Theme) -> Row {
    let ring = ring::view(RING_SIZE, RING_THICKNESS, value, color, theme.border);

    let icon = Rectangle::new()
        .width(RING_SIZE)
        .height(RING_SIZE)
        .align_child(Center, Center)
        .child(Text::new(icon).size(10.0).font(fonts::MATERIAL).color(theme.text));

    /*
     * amane has no stack widget yet, so a gap of minus the ring's width
     * puts the icon back over the ring
     */
    let ring_with_icon = Row::new(children![ring, icon]).gap(-RING_SIZE);

    Row::new(children![ring_with_icon, pill::label(text, 14.0, theme.text)])
        .gap(6.0)
        .align(Center)
}

// none on a desktop without a battery
fn battery(theme: &Theme) -> Option<Row> {
    let battery = Battery::read();

    if !battery.present() {
        return None;
    }

    let percent = battery.percent();

    let color = if battery.charging() {
        theme.success
    } else if percent > 20 {
        theme.accent
    } else {
        theme.danger
    };

    let icon = battery_icon(percent, battery.charging());

    let text = format!("{percent}%");

    let value = f32::from(percent) / 100.0;

    Some(indicator(value, color, icon, &text, theme))
}

// jaqc's BatteryService.icon, one glyph per 10%
fn battery_icon(percent: u8, charging: bool) -> &'static str {
    if charging {
        return "󰚥";
    }

    match percent {
        91.. => "󰁹",
        81..=90 => "󰂂",
        71..=80 => "󰂁",
        61..=70 => "󰂀",
        51..=60 => "󰁿",
        41..=50 => "󰁾",
        31..=40 => "󰁽",
        21..=30 => "󰁼",
        11..=20 => "󰁻",
        _ => "󰁺",
    }
}

fn memory(theme: &Theme) -> Row {
    let memory = Memory::read();

    let usage = f32::from(memory.percent()) / 100.0;

    let color = if usage < 0.5 {
        theme.success
    } else if usage < 0.8 {
        theme.accent
    } else {
        theme.danger
    };

    // jaqc shows the used amount, like "5.2G"
    let gibibytes = memory.used_kib() as f32 / 1024.0 / 1024.0;

    let text = format!("{gibibytes:.1}G");

    indicator(usage, color, MEMORY_ICON, &text, theme)
}

// opens the utility center in jaqc; here it only shows the icons for now
fn tray(theme: &Theme) -> Rectangle {
    let mut icons: Vec<Box<dyn Widget>> = Vec::new();

    for icon in TRAY_ICONS {
        let text = Text::new(icon).size(15.0).font(fonts::NERD).color(theme.on_accent);

        icons.push(Box::new(text));
    }

    // spread across the inside, so the spacing matches however wide amane measures each glyph
    let row = Row::new(icons)
        .width(TRAY_WIDTH - TRAY_PADDING * 2.0)
        .justify(SpaceBetween)
        .align(Center);

    pill::view(TRAY_WIDTH, theme.accent)
        .align_child(Center, Center)
        .child(row)
}
