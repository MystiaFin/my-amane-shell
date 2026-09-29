mod results;

use amane::{
    Apps, Center, Column, Image, Key, Padding, Parent, Pointer, Rectangle, Row, Scroll,
    Service, Stack, Start, Text, TextInput, Widget, children,
};

use super::{Overlay, PanelView, Region};
use crate::fonts;
use crate::liquid::{self, Blob};
use crate::theme::Theme;

use results::{Entry, Kind};

// the text input's name, which keeps what was typed between redraws
const INPUT: &str = "launcher";

const MAX_WIDTH: f32 = 620.0;
const MAX_HEIGHT: f32 = 620.0;
const MAX_ROWS: usize = 9;

const ROW_HEIGHT: f32 = 60.0;
const FIELD_HEIGHT: f32 = 46.0;

const RADIUS: f32 = 30.0;
const INNER_RADIUS: f32 = 12.0;

// the panel reaches this far past the screen's bottom edge, so it stays melted into it
const EDGE_OVERLAP: f32 = 40.0;

// how far past its own height it slides to let go of the edge
const HIDDEN_MARGIN: f32 = 32.0;

// a hidden launcher is a little narrower, it widens as it rises
const CLOSED_WIDTH: f32 = 0.96;

const PADDING: Padding = Padding {
    top: 12.0,
    right: 14.0,
    bottom: 56.0,
    left: 14.0,
};

// the gap between the list and the search field
const GAP: f32 = 8.0;

const FIXED_HEIGHT: f32 = PADDING.top + GAP + FIELD_HEIGHT + PADDING.bottom;

const ICON_SIZE: f32 = 38.0;

// none while fully hidden
pub fn view(overlay: &Overlay, theme: &Theme, screen: Region) -> Option<PanelView> {
    let progress = overlay.launcher.progress.value();

    if progress <= 0.001 {
        return None;
    }

    let entries = results::find(&overlay.query, &overlay.sessions);

    let full_width = MAX_WIDTH.min(screen.width - 80.0);
    let width = full_width * (CLOSED_WIDTH + (1.0 - CLOSED_WIDTH) * progress);

    let rows = visible_rows(screen, entries.len()).max(1);

    let height = FIXED_HEIGHT + rows as f32 * ROW_HEIGHT;

    let hidden = height + HIDDEN_MARGIN;

    let x = (screen.width - width) / 2.0;
    let y = screen.height - height + EDGE_OVERLAP + hidden * (1.0 - progress);

    let blob = Blob::new(x, y, width, height)
        .radius(RADIUS)
        .child(content(overlay, theme, &entries, width, rows));

    let input = Region {
        x,
        y,
        width,
        height,
    };

    // as tall as it can ever grow, so the window keeps its size while results change
    let tallest = FIXED_HEIGHT + max_rows(screen) as f32 * ROW_HEIGHT;

    let reach_height = tallest - EDGE_OVERLAP + liquid::CONNECTION;
    let reach_width = full_width + liquid::CONNECTION * 2.0;

    let reach = Region {
        x: (screen.width - reach_width) / 2.0,
        y: screen.height - reach_height,
        width: reach_width,
        height: reach_height,
    };

    Some(PanelView { blob, input, reach })
}

fn max_height(screen: Region) -> f32 {
    MAX_HEIGHT.min(screen.height - 60.0)
}

fn max_rows(screen: Region) -> usize {
    let fitting = ((max_height(screen) - FIXED_HEIGHT) / ROW_HEIGHT).floor() as usize;

    fitting.clamp(1, MAX_ROWS)
}

fn visible_rows(screen: Region, count: usize) -> usize {
    count.min(max_rows(screen))
}

fn content(
    overlay: &Overlay,
    theme: &Theme,
    entries: &[Entry],
    width: f32,
    rows: usize,
) -> Rectangle {
    let inner_width = width - PADDING.left - PADDING.right;

    let list_height = rows as f32 * ROW_HEIGHT;

    let field = Rectangle::new()
        .width(inner_width)
        .height(FIELD_HEIGHT)
        .radius(INNER_RADIUS)
        .fill(theme.surface)
        .padding(Padding {
            top: 0.0,
            right: 14.0,
            bottom: 0.0,
            left: 14.0,
        })
        .align_child(Start, Center)
        .child(
            TextInput::new(INPUT)
                .size(15.0)
                .color(theme.text)
                .placeholder("Type > for command palette...")
                .focused()
                .on_change(query_changed)
                .on_submit(|_| launch_selected()),
        );

    let list = list(overlay, theme, entries, inner_width, list_height);

    Rectangle::new()
        .width(width)
        .height(Parent)
        .padding(PADDING)
        .child(Column::new(children![list, field]).gap(GAP))
}

// the rows that fit, with the selection highlight sliding behind them
fn list(
    overlay: &Overlay,
    theme: &Theme,
    entries: &[Entry],
    width: f32,
    height: f32,
) -> Rectangle {
    let area = Rectangle::new()
        .width(width)
        .height(height)
        .clip()
        .on_scroll(scrolled);

    if entries.is_empty() {
        let empty = Text::new(results::empty_text(&overlay.query))
            .size(14.0)
            .font(fonts::BODY)
            .color(theme.secondary_text);

        return area.align_child(Center, Center).child(empty);
    }

    let highlight = Rectangle::new()
        .width(width)
        .height(ROW_HEIGHT)
        .radius(INNER_RADIUS)
        .fill(theme.selected_surface)
        .translate(0.0, overlay.highlight.value() * ROW_HEIGHT);

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    let last = (overlay.first + MAX_ROWS).min(entries.len());

    for index in overlay.first..last {
        rows.push(Box::new(row(overlay, theme, &entries[index], index, width)));
    }

    area.child(Stack::new(children![highlight, Column::new(rows)]))
}

fn row(overlay: &Overlay, theme: &Theme, entry: &Entry, index: usize, width: f32) -> Rectangle {
    let hovered = overlay.hovered_row == Some(index) && overlay.selected != index;

    let fill = if hovered {
        theme.hover_surface
    } else {
        amane::Color::TRANSPARENT
    };

    let name = Text::new(&entry.name)
        .size(15.0)
        .font(fonts::BODY)
        .color(theme.text)
        .elide();

    let name = Rectangle::new()
        .width(width - 62.0 - 12.0)
        .height(ROW_HEIGHT)
        .align_child(Start, Center)
        .child(name);

    Rectangle::new()
        .width(width)
        .height(ROW_HEIGHT)
        .radius(INNER_RADIUS)
        .fill(fill)
        .cursor(Pointer)
        .on_hover(move |inside| hover_row(index, inside))
        .on_click(move |_| {
            Overlay::write().selected = index;

            launch_selected();
        })
        .padding(Padding {
            top: 0.0,
            right: 12.0,
            bottom: 0.0,
            left: 12.0,
        })
        .align_child(Start, Center)
        .child(Row::new(children![icon(entry, theme), name]).gap(12.0).align(Center))
}

// the app's own icon, a glyph for commands, or empty space to keep names lined up
fn icon(entry: &Entry, theme: &Theme) -> Rectangle {
    let slot = Rectangle::new()
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .align_child(Center, Center);

    if let Some(glyph) = entry.glyph {
        return slot.child(Text::new(glyph).size(24.0).font(fonts::NERD).color(theme.text));
    }

    let Kind::App(index) = entry.kind else {
        return slot;
    };

    let apps = Apps::read();

    let Some(path) = apps.list()[index].icon_path() else {
        return slot;
    };

    // amane only decodes png and jpeg so far, most svg icons stay empty
    let readable = matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("png" | "jpg" | "jpeg")
    );

    if !readable {
        return slot;
    }

    slot.fill(Image::contain(path))
}

pub fn ipc(arguments: &[String]) -> String {
    let command = arguments.first().map(String::as_str).unwrap_or("toggle");

    let mut overlay = Overlay::write();

    match command {
        "show" => show(&mut overlay),
        "hide" => overlay.launcher.hide(),
        _ if overlay.launcher.shown => overlay.launcher.hide(),
        _ => show(&mut overlay),
    }

    String::from("ok")
}

// always opens on an empty search, at the top of the list
fn show(overlay: &mut Overlay) {
    overlay.power_menu.hide();
    overlay.launcher.show();

    overlay.query.clear();

    TextInput::set_text(INPUT, "");

    select(overlay, 0, 0);
}

// the window's keys while the launcher is open; letters and enter go to the search field
pub fn key_pressed(key: Key) {
    let mut overlay = Overlay::write();

    if !overlay.launcher.shown {
        return;
    }

    let count = results::find(&overlay.query, &overlay.sessions).len();

    match key {
        Key::Down if count > 0 => {
            let next = (overlay.selected + 1) % count;

            select(&mut overlay, next, count);
        }

        Key::Up if count > 0 => {
            let previous = (overlay.selected + count - 1) % count;

            select(&mut overlay, previous, count);
        }

        // the first escape clears the search, the second closes
        Key::Escape if !overlay.query.is_empty() => {
            overlay.query.clear();

            TextInput::set_text(INPUT, "");

            select(&mut overlay, 0, 0);
        }

        Key::Escape => overlay.launcher.hide(),

        _ => {}
    }
}

/*
 * moves the selection and scrolls just enough to keep it in the list;
 * the highlight slides to the selection's row
 */
fn select(overlay: &mut Overlay, index: usize, count: usize) {
    overlay.selected = index;

    if index < overlay.first {
        overlay.first = index;
    }

    if index >= overlay.first + MAX_ROWS {
        overlay.first = index + 1 - MAX_ROWS;
    }

    // a shorter list may leave the old top past its end
    overlay.first = overlay.first.min(count.saturating_sub(1));

    let row = (index - overlay.first.min(index)) as f32;

    overlay.highlight.to(row);
}

fn query_changed(query: String) {
    let mut overlay = Overlay::write();

    // tmux sessions are listed once, when the search switches to them
    if query.starts_with('!') && !overlay.query.starts_with('!') {
        overlay.sessions = results::read_sessions();
    }

    overlay.query = query;

    overlay.first = 0;

    select(&mut overlay, 0, 0);
}

// wheel down shows later results, the selection stays where it was
fn scrolled(scroll: Scroll) {
    let mut overlay = Overlay::write();

    let count = results::find(&overlay.query, &overlay.sessions).len();

    let last_top = count.saturating_sub(MAX_ROWS);

    if scroll.y > 0.0 {
        overlay.first = (overlay.first + 1).min(last_top);
    } else {
        overlay.first = overlay.first.saturating_sub(1);
    }

    let row = overlay.selected as f32 - overlay.first as f32;

    overlay.highlight.to(row);
}

fn hover_row(index: usize, inside: bool) {
    let mut overlay = Overlay::write();

    if inside {
        overlay.hovered_row = Some(index);
    } else if overlay.hovered_row == Some(index) {
        overlay.hovered_row = None;
    }
}

fn launch_selected() {
    let mut overlay = Overlay::write();

    let entries = results::find(&overlay.query, &overlay.sessions);

    let Some(entry) = entries.get(overlay.selected) else {
        return;
    };

    match &entry.kind {
        Kind::App(index) => {
            Apps::read().list()[*index].launch();

            overlay.launcher.hide();
        }

        Kind::TmuxCommand => {
            overlay.query = String::from("!");
            overlay.sessions = results::read_sessions();

            TextInput::set_text(INPUT, "!");

            select(&mut overlay, 0, 0);
        }

        Kind::Tmux(session) => {
            if results::attach(session) {
                overlay.launcher.hide();
            }
        }
    }
}
