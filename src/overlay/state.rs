use std::time::Duration;

use amane::{Animation, Easing, Service};

use super::panel::Panel;

const FILL_DURATION: Duration = Duration::from_millis(200);

// which panels are out, shared by the bar that opens them and the overlay that draws them
pub struct Overlay {
    pub power_menu: Panel,

    // how far each power menu button's hover color has filled it, 0 to 1
    pub action_fills: Vec<Animation>,

    // leaving a panel for the bar keeps it open
    pub bar_hovered: bool,

    pub launcher: Panel,

    pub query: String,

    // the chosen result, and the first result the list shows
    pub selected: usize,
    pub first: usize,

    pub hovered_row: Option<usize>,

    // where the selection highlight is, in rows from the top of the list
    pub highlight: Animation,

    pub sessions: Vec<String>,
}

impl Service for Overlay {
    fn new() -> Self {
        let mut action_fills = Vec::new();

        for _ in 0..super::power_menu::ACTION_COUNT {
            action_fills.push(fast(0.0));
        }

        Self {
            power_menu: Panel::new(),
            action_fills,
            bar_hovered: false,
            launcher: Panel::new(),
            query: String::new(),
            selected: 0,
            first: 0,
            hovered_row: None,
            highlight: fast(0.0),
            sessions: Vec::new(),
        }
    }

    // only changes through input
    fn listen() {}
}

impl Overlay {
    // only one panel is out at a time
    pub fn toggle_power_menu() {
        let mut overlay = Self::write();

        overlay.launcher.hide();
        overlay.power_menu.toggle();
    }

    pub fn hide_power_menu() {
        Self::write().power_menu.hide();
    }

    pub fn hover_power_menu(inside: bool) {
        let mut overlay = Self::write();

        overlay.power_menu.hovered = inside;

        if inside {
            overlay.power_menu.was_hovered = true;
        }

        overlay.dismiss_if_left();
    }

    pub fn hover_bar(inside: bool) {
        let mut overlay = Self::write();

        overlay.bar_hovered = inside;

        overlay.dismiss_if_left();
    }

    // a panel closes once the pointer has been on it and then left it and the bar
    fn dismiss_if_left(&mut self) {
        let menu = &self.power_menu;

        let left = menu.was_hovered && !menu.hovered && !self.bar_hovered;

        if menu.shown && left {
            self.power_menu.hide();
        }
    }
}

pub fn fast(value: f32) -> Animation {
    Animation::new(value)
        .duration(FILL_DURATION)
        .easing(Easing::Out)
}
