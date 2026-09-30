mod card;
mod sensors;
mod weather;

use std::time::SystemTime;

use amane::{
    Column, Cpu, End, Full, Layer, LayerWindow, Monitor, Parent, Rectangle, Row, Service,
    SpaceBetween, Stack, Text, Weight, Widget, Workspaces, Zone, children,
};

use crate::bar;
use crate::clock::Clock;
use crate::fonts;
use crate::motion::{self, DEFAULT_SPATIAL};
use crate::theme::{self, Theme};

use card::Reading;
use sensors::Sensors;
use weather::Weather;

const MARGIN: f32 = 36.0;
const GAP: f32 = 16.0;

const WEATHER_WIDTH: f32 = 220.0;
const WEATHER_HEIGHT: f32 = 160.0;

const CLOCK_WIDTH: f32 = 300.0;
const CLOCK_HEIGHT: f32 = 132.0;

const THERMOMETER: &str = "\u{f050f}";
const PROCESSOR: &str = "\u{f035b}";
const SUN: &str = "\u{f0599}";
const DROP: &str = "\u{f058c}";
const LEAF: &str = "\u{f032a}";

/*
 * cards on the desktop, under every window: readings top left, weather top
 * right, the air outside bottom left and a big clock bottom right; they only
 * show on a workspace with no windows, where the desktop is actually seen
 */
pub fn view(monitor: &Monitor) -> LayerWindow {
    let theme = theme::current();

    let width = monitor.width as f32;
    let height = monitor.height as f32 - bar::HEIGHT;

    let empty = Workspaces::read()
        .list()
        .iter()
        .find(|workspace| workspace.active() && workspace.output() == Some(&monitor.name))
        .is_some_and(|workspace| workspace.windows() == 0);

    // the cards fade in and out, so the window stays until they are gone
    let target = if empty { 1.0 } else { 0.0 };

    let shown = motion::follow(&format!("floating:{}", monitor.name), target, DEFAULT_SPATIAL);

    let layers: Vec<Box<dyn Widget>> = vec![
        Box::new(readings(&theme).translate(MARGIN, MARGIN)),
        Box::new(weather(&theme).translate(width - MARGIN - WEATHER_WIDTH, MARGIN)),
        Box::new(air(&theme).translate(MARGIN, height - MARGIN - card::HEIGHT)),
        Box::new(clock(&theme).translate(width - MARGIN - CLOCK_WIDTH, height - MARGIN - CLOCK_HEIGHT)),
    ];

    LayerWindow::new()
        .width(Full)
        .height(Full)
        .layer(Layer::Bottom)
        .space(Zone::Respect)
        .namespace("floating-widgets")
        .visible(empty || shown > 0.001)
        .click_through()
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .opacity(shown)
                .child(Stack::new(layers).width(Parent).height(Parent)),
        )
}

// the cpu's temperature and load, and the gpu's temperature on machines with a sensor for it
fn readings(theme: &Theme) -> Rectangle {
    let sensors = Sensors::read();

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();

    if let Some(cpu) = sensors.cpu {
        cards.push(Box::new(temperature(theme, "CPU TEMPERATURE", cpu)));
    }

    let load = Cpu::read().percent();

    cards.push(Box::new(card::view(
        theme,
        Reading {
            label: "CPU USAGE",
            icon: PROCESSOR,
            value: format!("{load}%"),
            amount: f32::from(load) / 100.0,
            color: theme.accent,
        },
    )));

    if let Some(gpu) = sensors.gpu {
        cards.push(Box::new(temperature(theme, "GPU TEMPERATURE", gpu)));
    }

    row_of(cards)
}

fn temperature(theme: &Theme, label: &str, degrees: u32) -> Rectangle {
    card::view(
        theme,
        Reading {
            label,
            icon: THERMOMETER,
            value: format!("{degrees}°C"),
            amount: degrees as f32 / 100.0,
            color: theme.accent,
        },
    )
}

// uv index, humidity and the us air quality index, from the weather service
fn air(theme: &Theme) -> Rectangle {
    let weather = Weather::read();

    let shown = |value: Option<f32>, digits: usize| match value {
        Some(value) => format!("{value:.digits$}"),
        None => String::from("–"),
    };

    let uv = card::view(
        theme,
        Reading {
            label: "UV INDEX",
            icon: SUN,
            value: shown(weather.uv_index, 1),
            amount: weather.uv_index.unwrap_or(0.0) / 11.0,
            color: theme.danger,
        },
    );

    let humidity = card::view(
        theme,
        Reading {
            label: "HUMIDITY",
            icon: DROP,
            value: match weather.humidity {
                Some(humidity) => format!("{humidity:.0}%"),
                None => String::from("–"),
            },
            amount: weather.humidity.unwrap_or(0.0) / 100.0,
            color: theme.accent,
        },
    );

    let air_quality = card::view(
        theme,
        Reading {
            label: "AQI",
            icon: LEAF,
            value: shown(weather.air_quality, 0),
            amount: weather.air_quality.unwrap_or(0.0) / 300.0,
            color: theme.accent,
        },
    );

    row_of(children![uv, humidity, air_quality])
}

fn weather(theme: &Theme) -> Rectangle {
    let weather = Weather::read();

    let degrees = |value: Option<f32>| match value {
        Some(value) => format!("{value:.0}°"),
        None => String::from("–"),
    };

    let temperature = Text::new(degrees(weather.temperature))
        .size(44.0)
        .font(fonts::BODY)
        .weight(Weight::Bold)
        .color(theme.text);

    let condition = Text::new(weather.condition())
        .size(13.0)
        .font(fonts::BODY)
        .weight(Weight::Medium)
        .color(theme.text);

    let range = Text::new(format!("H {}  L {}", degrees(weather.high), degrees(weather.low)))
        .size(12.0)
        .font(fonts::BODY)
        .color(theme.secondary_text);

    let icon = Text::new(weather.icon())
        .size(40.0)
        .font(fonts::NERD)
        .tight()
        .color(theme.accent);

    let bottom = Row::new(children![
        Column::new(children![condition, range]).gap(2.0),
        icon,
    ])
    .width(WEATHER_WIDTH - 36.0)
    .justify(SpaceBetween)
    .align(End);

    card::background(theme, WEATHER_WIDTH, WEATHER_HEIGHT).child(
        Column::new(children![temperature, bottom])
            .height(WEATHER_HEIGHT - 36.0)
            .justify(SpaceBetween),
    )
}

// "14:36" over the date, straight on the desktop without a card
fn clock(theme: &Theme) -> Rectangle {
    let clock = Clock::read();

    let time = Text::new(clock.hours_minutes(SystemTime::now()))
        .size(72.0)
        .font(fonts::BODY)
        .weight(Weight::Bold)
        .color(theme.text);

    let date = Text::new(clock.date())
        .size(16.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.secondary_text);

    Rectangle::new()
        .width(CLOCK_WIDTH)
        .height(CLOCK_HEIGHT)
        .align_child(End, End)
        .child(Column::new(children![time, date]).align(End).gap(0.0))
}

// cards side by side, sized to fit them so the row can be placed as one
fn row_of(cards: Vec<Box<dyn Widget>>) -> Rectangle {
    let count = cards.len() as f32;

    let width = card::WIDTH * count + GAP * (count - 1.0).max(0.0);

    Rectangle::new()
        .width(width)
        .height(card::HEIGHT)
        .child(Row::new(cards).gap(GAP))
}
