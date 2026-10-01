use std::thread;
use std::time::Duration;

use amane::{Lock, Service};

use crate::motion::{self, Glide};

const FADE: u64 = 300;
const DARKEN: u64 = 400;

const PAM_POLL: Duration = Duration::from_millis(50);

/*
 * the desktop around a lock: its items fade out, then the wallpaper
 * darkens to the lock screen's dim, so the lock screen opens seamlessly;
 * unlocking plays it backwards
 */
pub struct Curtain {
    // the bar, panels, cards and screen mask, 1 while shown
    pub items: Glide,

    // how far the wallpaper is darkened, 1 matches the lock screen
    pub dark: Glide,

    // set from the fade out until the desktop is back
    closing: bool,
}

impl Service for Curtain {
    fn new() -> Self {
        Self {
            items: motion::effects(1.0, FADE),
            dark: motion::effects(0.0, DARKEN),
            closing: false,
        }
    }

    // it only changes when locking and unlocking
    fn listen() {}
}

// blocks until the session is locked; a lock already on its way is left alone
pub fn close() {
    {
        let mut curtain = Curtain::write();

        if curtain.closing {
            return;
        }

        curtain.closing = true;
        curtain.items.to(0.0);
    }

    thread::sleep(motion::paced(FADE));

    Curtain::write().dark.to(1.0);

    thread::sleep(motion::paced(DARKEN));

    Lock::start();
}

// waits on pam, then brings the desktop back if the password was right
pub fn open_when_accepted() {
    while Lock::read().checking() {
        thread::sleep(PAM_POLL);
    }

    if Lock::read().failed() {
        return;
    }

    Curtain::write().dark.to(0.0);

    thread::sleep(motion::paced(DARKEN));

    let mut curtain = Curtain::write();

    curtain.items.to(1.0);
    curtain.closing = false;
}
