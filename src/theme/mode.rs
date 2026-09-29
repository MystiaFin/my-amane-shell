use amane::Service;

// light or dark as chosen in the utility center, none to follow the wallpaper
#[derive(Default)]
pub struct Mode {
    pub light: Option<bool>,
}

impl Service for Mode {
    fn new() -> Self {
        Self::default()
    }

    // it only changes through input
    fn listen() {}
}

impl Mode {
    // flips whatever is showing now, so the first click always changes something
    pub fn toggle(showing_light: bool) {
        Self::write().light = Some(!showing_light);
    }
}
