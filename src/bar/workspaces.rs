use amane::{
    Center, Color, Full, Monitor, Parent, Pointer, Rectangle, Row, Service, Stack, Start, Text,
    Weight, Widget, Workspace, Workspaces, children,
};

use super::{motion, star};
use crate::fonts;
use crate::theme::Theme;

// jaqc's Icons.nixos and inactiveWorkspace, the active star is drawn in star.rs
const LOGO: &str = "\u{f1105}";
const INACTIVE: &str = "\u{f444}";

// jaqc's pill style: every workspace is a 26px slot, 4px apart, inside a 32px pill
const SLOT: f32 = 26.0;
const SLOT_GAP: f32 = 4.0;
const STRIP_HEIGHT: f32 = 32.0;
const STRIP_PADDING: f32 = 4.0;

// with the row's 10px gap this makes jaqc's 20px from the screen edge
const EDGE: f32 = 10.0;

pub fn view(monitor: &Monitor, theme: &Theme, width: f32) -> Row {
    let workspaces = Workspaces::read();

    let mut own: Vec<&Workspace> = Vec::new();

    // each bar only shows the workspaces of its own monitor
    for workspace in workspaces.list() {
        if workspace.output() == Some(monitor.name.as_str()) {
            own.push(workspace);
        }
    }

    own.sort_by_key(|workspace| workspace.index());

    let logo = Rectangle::new()
        .width(30.0)
        .height(STRIP_HEIGHT)
        .align_child(Center, Center)
        .child(Text::new(LOGO).size(28.0).font(fonts::NERD).color(theme.accent));

    let name = Text::new(active_name(&own))
        .size(14.0)
        .font(fonts::BODY)
        .weight(Weight::Medium)
        .color(theme.text);

    let items: Vec<Box<dyn Widget>> = vec![
        Box::new(Rectangle::new().width(EDGE).height(1.0)),
        Box::new(logo),
        Box::new(strip(monitor, &own, theme)),
        Box::new(name),
    ];

    Row::new(items)
        .width(width)
        .height(Parent)
        .gap(10.0)
        .justify(Start)
        .align(Center)
}

// the dots, with the sliding accent highlight drawn over them
fn strip(monitor: &Monitor, workspaces: &[&Workspace], theme: &Theme) -> Rectangle {
    let mut slots: Vec<Box<dyn Widget>> = Vec::new();

    let mut active = 0;

    for (position, workspace) in workspaces.iter().enumerate() {
        if workspace.active() {
            active = position;
        }

        slots.push(Box::new(slot(workspace, theme)));
    }

    let count = workspaces.len() as f32;

    let gaps = (count - 1.0).max(0.0);

    let content = count * SLOT + gaps * SLOT_GAP;

    let (offset, rotation) = motion::highlight(&monitor.name, active, SLOT + SLOT_GAP);

    let highlight = Rectangle::new()
        .width(SLOT)
        .height(SLOT)
        .radius(Full)
        .fill(theme.accent)
        .translate(offset, 0.0)
        .child(star::view(SLOT, rotation, theme.on_accent));

    // the highlight slides over the dots, which stay where they are
    let layers = Stack::new(children![Row::new(slots).gap(SLOT_GAP).align(Center), highlight]);

    Rectangle::new()
        .width(content + STRIP_PADDING * 2.0)
        .height(STRIP_HEIGHT)
        .radius(Full)
        .fill(theme.selected_surface)
        .align_child(Center, Center)
        .child(layers)
}

// a dot, drawn at 18px scaled to 0.6 like jaqc; the highlight covers the active one
fn slot(workspace: &Workspace, theme: &Theme) -> Rectangle {
    let id = workspace.id();

    let color = if workspace.urgent() {
        theme.danger
    } else if workspace.active() {
        Color::TRANSPARENT
    } else {
        theme.muted_text
    };

    Rectangle::new()
        .width(SLOT)
        .height(SLOT)
        .cursor(Pointer)
        .on_click(move |_| Workspaces::focus(id))
        .align_child(Center, Center)
        .child(Text::new(INACTIVE).size(11.0).font(fonts::SYMBOLS).color(color))
}

// the workspace's own name, or "Workspace 2" when it has none, like jaqc
fn active_name(workspaces: &[&Workspace]) -> String {
    for workspace in workspaces {
        if !workspace.active() {
            continue;
        }

        let index = workspace.index().to_string();

        return match workspace.name() {
            Some(name) if !name.is_empty() && name != index => String::from(name),
            _ => format!("Workspace {index}"),
        };
    }

    String::from("Desktop")
}
