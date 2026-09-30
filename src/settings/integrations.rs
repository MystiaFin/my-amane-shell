use amane::{Column, children};

use super::switch;
use crate::theme::Theme;

// other programs that take the shell's colors; each one writes files outside this config
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        switch::row(
            theme,
            width,
            "integration_gtk",
            "GTK",
            "Writes a GTK 3/4 theme, sets it through dconf, restarts the GNOME portal",
        ),
        switch::row(
            theme,
            width,
            "integration_terminal",
            "Terminals",
            "Writes kitty and foot colors and recolors open terminals",
        ),
        switch::row(
            theme,
            width,
            "integration_tmux",
            "tmux",
            "Writes tmux colors and reloads running tmux servers",
        ),
        switch::row(
            theme,
            width,
            "integration_vesktop",
            "Vesktop",
            "Rewrites a marked block in Vesktop's Quick CSS",
        ),
        switch::row(
            theme,
            width,
            "integration_spotify",
            "Spotify",
            "Writes a Spicetify stylesheet to the cache folder",
        ),
        switch::row(
            theme,
            width,
            "integration_btop",
            "btop",
            "Writes a btop theme",
        ),
        switch::row(
            theme,
            width,
            "integration_cava",
            "Cava",
            "Writes a cava color theme",
        ),
    ])
    .gap(12.0)
}
