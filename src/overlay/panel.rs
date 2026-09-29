use std::time::Duration;

use amane::{Animation, Easing};

const DURATION: Duration = Duration::from_millis(350);

// one panel that grows out of a screen edge
pub struct Panel {
    pub shown: bool,

    // whether the pointer is on it, and whether it has been since it opened
    pub hovered: bool,
    pub was_hovered: bool,

    // 0 when hidden behind the edge, 1 when fully out
    pub progress: Animation,
}

impl Panel {
    pub fn new() -> Self {
        let progress = Animation::new(0.0).duration(DURATION).easing(Easing::Out);

        Self {
            shown: false,
            hovered: false,
            was_hovered: false,
            progress,
        }
    }

    pub fn show(&mut self) {
        self.shown = true;
        self.was_hovered = false;

        self.progress.to(1.0);
    }

    pub fn hide(&mut self) {
        self.shown = false;

        self.progress.to(0.0);
    }

    pub fn toggle(&mut self) {
        if self.shown {
            self.hide();
        } else {
            self.show();
        }
    }
}
