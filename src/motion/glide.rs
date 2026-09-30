use std::time::{Duration, Instant};

/*
 * moves to its target in a fixed time along a cubic bezier curve, like
 * css's cubic-bezier(); a new target starts from where it is right now
 */
pub struct Glide {
    from: f32,
    target: f32,

    started: Instant,
    duration: Duration,

    // the curve's two handles, its ends are fixed at (0, 0) and (1, 1)
    handles: [f32; 4],
}

impl Glide {
    pub fn new(value: f32, duration: Duration, handles: [f32; 4]) -> Self {
        Self {
            from: value,
            target: value,
            started: Instant::now(),
            duration,
            handles,
        }
    }

    pub fn to(&mut self, target: f32) {
        if target == self.target {
            return;
        }

        self.from = self.current();
        self.target = target;
        self.started = Instant::now();
    }

    // read in the view, and while it is still moving the window keeps drawing frames
    pub fn value(&self) -> f32 {
        if self.progress() < 1.0 {
            amane::request_frame();
        }

        self.current()
    }

    fn current(&self) -> f32 {
        let eased = ease(self.progress(), self.handles);

        self.from + (self.target - self.from) * eased
    }

    // 0 when it has just started, 1 once it has arrived
    fn progress(&self) -> f32 {
        if self.duration.is_zero() || super::reduced() {
            return 1.0;
        }

        // the animation speed setting makes time pass faster or slower
        let elapsed = self.started.elapsed().as_secs_f32() * super::speed();
        let total = self.duration.as_secs_f32();

        f32::min(elapsed / total, 1.0)
    }
}

/*
 * the curve is drawn by a parameter t, not by time, so first find the t
 * whose x is the time that has passed, then its y is how far along it is
 */
fn ease(progress: f32, [x1, y1, x2, y2]: [f32; 4]) -> f32 {
    if progress <= 0.0 || progress >= 1.0 {
        return progress.clamp(0.0, 1.0);
    }

    // x only grows along the curve, so halving the range always closes in on t
    let mut low = 0.0;
    let mut high = 1.0;

    for _ in 0..24 {
        let middle = (low + high) / 2.0;

        if bezier(middle, x1, x2) < progress {
            low = middle;
        } else {
            high = middle;
        }
    }

    let t = (low + high) / 2.0;

    bezier(t, y1, y2)
}

// one coordinate of a cubic bezier from 0 to 1, with its two handles at first and second
fn bezier(t: f32, first: f32, second: f32) -> f32 {
    let rest = 1.0 - t;

    3.0 * rest * rest * t * first + 3.0 * rest * t * t * second + t * t * t
}
