use amane::{
    Center, Color, Column, End, Padding, Pointer, Rectangle, Row, Service, Stack, Start, Text,
    Weight, Widget, children,
};

use super::{hover, hovered};
use crate::clock::{self, Clock};
use crate::fonts;
use crate::motion::{self, DEFAULT_SPATIAL};
use crate::overlay::Overlay;
use crate::theme::Theme;

pub const HEIGHT: f32 = 260.0;

const PADDING: f32 = 12.0;

const TITLE_HEIGHT: f32 = 28.0;
const WEEKDAYS_HEIGHT: f32 = 18.0;
const GAP: f32 = 7.0;

const CELL_GAP: f32 = 2.0;
const ROWS: usize = 6;

const NAVIGATION_SIZE: f32 = 28.0;

const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const PREVIOUS_ICON: &str = "󰁍";
const NEXT_ICON: &str = "󰁔";

// one day in the grid
struct Day {
    number: i64,

    // days of the months before and after are faded
    this_month: bool,

    today: bool,
}

/*
 * months sit a full width apart and slide to the chosen one, so while
 * sliding both the old and the new month are drawn
 */
pub fn view(overlay: &Overlay, theme: &Theme, width: f32) -> Rectangle {
    let inner_width = width - PADDING * 2.0;
    let inner_height = HEIGHT - PADDING * 2.0;

    let chosen = overlay.calendar_month as f32;

    let shown = motion::follow("calendar-month", chosen, DEFAULT_SPATIAL);

    let today = Clock::read().today();

    let mut pages: Vec<Box<dyn Widget>> = Vec::new();

    let nearest = shown.floor() as i64;

    for offset in [nearest, nearest + 1] {
        let distance = offset as f32 - shown;

        if distance.abs() >= 1.0 {
            continue;
        }

        let page = month(overlay, theme, today, offset, inner_width, inner_height)
            .translate(distance * inner_width, 0.0);

        pages.push(Box::new(page));
    }

    let mut viewport = Rectangle::new()
        .width(inner_width)
        .height(inner_height)
        .child(Stack::new(pages));

    // a clip is a gpu layer, so only while a month slides past the edge
    if shown != chosen {
        viewport = viewport.clip();
    }

    let navigation = Row::new(children![
        navigation_button(overlay, theme, "previous", PREVIOUS_ICON, -1),
        navigation_button(overlay, theme, "next", NEXT_ICON, 1),
    ])
    .gap(6.0);

    let navigation = Rectangle::new()
        .width(inner_width)
        .height(NAVIGATION_SIZE)
        .align_child(End, Start)
        .child(navigation);

    Rectangle::new()
        .width(width)
        .height(HEIGHT)
        .radius(16.0)
        .fill(theme.surface)
        .padding(Padding {
            top: PADDING,
            right: PADDING,
            bottom: PADDING,
            left: PADDING,
        })
        .child(Stack::new(children![viewport, navigation]))
}

// the title, the weekday names and six weeks of days
fn month(
    overlay: &Overlay,
    theme: &Theme,
    today: (i64, i64, i64),
    offset: i64,
    width: f32,
    height: f32,
) -> Rectangle {
    let (today_year, today_month, _) = today;

    // counted in months from year 0, so adding the offset crosses years by itself
    let index = today_year * 12 + (today_month - 1) + offset;

    let year = index.div_euclid(12);
    let month = index.rem_euclid(12) + 1;

    let title = Text::new(format!("{} {year}", MONTHS[(month - 1) as usize]))
        .size(15.0)
        .font(fonts::BODY)
        .weight(Weight::SemiBold)
        .color(theme.text);

    let mut title = Rectangle::new()
        .width(width - NAVIGATION_SIZE * 2.0 - 12.0)
        .height(TITLE_HEIGHT)
        .align_child(Start, Center)
        .child(title);

    // the title leads back to this month
    if offset != 0 {
        title = title
            .cursor(Pointer)
            .on_click(|_| Overlay::write().calendar_month = 0);
    }

    let grid_height = height - TITLE_HEIGHT - WEEKDAYS_HEIGHT - GAP * 2.0;

    let column = Column::new(children![
        title,
        weekdays(theme, width),
        grid(
            overlay,
            theme,
            &days(today, year, month),
            offset,
            width,
            grid_height
        ),
    ])
    .gap(GAP);

    Rectangle::new().width(width).height(height).child(column)
}

fn weekdays(theme: &Theme, width: f32) -> Row {
    let cell_width = width / 7.0;

    let mut names: Vec<Box<dyn Widget>> = Vec::new();

    for name in WEEKDAYS {
        let text = Text::new(name)
            .size(9.0)
            .font(fonts::BODY)
            .color(theme.muted_text);

        let cell = Rectangle::new()
            .width(cell_width)
            .height(WEEKDAYS_HEIGHT)
            .align_child(Center, Center)
            .child(text);

        names.push(Box::new(cell));
    }

    Row::new(names)
}

fn grid(
    overlay: &Overlay,
    theme: &Theme,
    days: &[Day],
    offset: i64,
    width: f32,
    height: f32,
) -> Column {
    let cell_width = (width - CELL_GAP * 6.0) / 7.0;
    let cell_height = (height - CELL_GAP * (ROWS - 1) as f32) / ROWS as f32;

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    for week in 0..ROWS {
        let mut cells: Vec<Box<dyn Widget>> = Vec::new();

        for weekday in 0..7 {
            let index = week * 7 + weekday;

            // hover only reaches the month in place, not one sliding past
            let hover_name = format!("day:{offset}:{index}");

            let cell = cell(
                overlay,
                theme,
                &days[index],
                hover_name,
                cell_width,
                cell_height,
            );

            cells.push(Box::new(cell));
        }

        rows.push(Box::new(Row::new(cells).gap(CELL_GAP)));
    }

    Column::new(rows).gap(CELL_GAP)
}

fn cell(
    overlay: &Overlay,
    theme: &Theme,
    day: &Day,
    hover_name: String,
    width: f32,
    height: f32,
) -> Rectangle {
    let fill = if day.today {
        theme.accent
    } else if day.this_month && hovered(overlay, &hover_name) {
        theme.selected_surface
    } else {
        Color::TRANSPARENT
    };

    let color = if day.today {
        theme.on_accent
    } else if day.this_month {
        theme.text
    } else {
        theme.border
    };

    let weight = if day.today {
        Weight::SemiBold
    } else {
        Weight::Light
    };

    let number = Text::new(day.number.to_string())
        .size(10.0)
        .font(fonts::BODY)
        .weight(weight)
        .color(color);

    Rectangle::new()
        .width(width)
        .height(height)
        .radius(8.0)
        .fill(fill)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .align_child(Center, Center)
        .child(number)
}

// six full weeks from the sunday on or before the 1st
fn days(today: (i64, i64, i64), year: i64, month: i64) -> Vec<Day> {
    let first = clock::days_since_1970(year, month, 1);

    // 1970-01-01 was a thursday, 4 days after a sunday
    let weekday = (first + 4).rem_euclid(7);

    let start = first - weekday;

    let (today_year, today_month, today_day) = today;
    let today = clock::days_since_1970(today_year, today_month, today_day);

    let mut days = Vec::new();

    for index in 0..(ROWS * 7) as i64 {
        let date = start + index;

        let (_, date_month, date_day) = clock::civil_date(date);

        days.push(Day {
            number: date_day,
            this_month: date_month == month,
            today: date == today,
        });
    }

    days
}

fn navigation_button(
    overlay: &Overlay,
    theme: &Theme,
    name: &str,
    icon: &str,
    step: i64,
) -> Rectangle {
    let hover_name = format!("calendar:{name}");

    let fill = if hovered(overlay, &hover_name) {
        theme.border
    } else {
        theme.selected_surface
    };

    Rectangle::new()
        .width(NAVIGATION_SIZE)
        .height(NAVIGATION_SIZE)
        .radius(8.0)
        .fill(fill)
        .cursor(Pointer)
        .on_hover(move |inside| hover(hover_name.clone(), inside))
        .on_click(move |_| Overlay::write().calendar_month += step)
        .align_child(Center, Center)
        .child(
            Text::new(icon)
                .size(14.0)
                .font(fonts::NERD)
                .tight()
                .color(theme.text),
        )
}
