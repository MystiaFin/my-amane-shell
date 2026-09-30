use amane::Service;

use crate::overlay::Overlay;

// whether the pointer is on the bar, which an auto-hiding bar slides out for
#[derive(Default)]
pub struct Reveal {
    pub inside: bool,
}

impl Service for Reveal {
    fn new() -> Self {
        Self::default()
    }

    // it only changes through input
    fn listen() {}
}

// panels opened from the bar also need to know the pointer is on it
pub fn hover(inside: bool) {
    Reveal::write().inside = inside;

    Overlay::hover_bar(inside);
}
