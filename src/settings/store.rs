use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use amane::Service;

// every setting and the value it has until changed
const DEFAULTS: [(&str, &str); 63] = [
    // appearance
    ("blur_strength", "1"),
    ("surface_opacity", "0.94"),
    ("reduce_transparency", "false"),
    // colors
    ("scheme", "dynamic"),
    ("manual_accent", "false"),
    ("accent", "#89b4fa"),
    ("saturation", "1"),
    ("contrast", "1"),
    ("follow_wallpaper", "true"),
    // bar
    ("bar_position", "top"),
    ("bar_height", "40"),
    ("bar_auto_hide", "false"),
    ("workspace_style", "pill"),
    ("bar_logo", "true"),
    ("bar_workspaces", "true"),
    ("bar_workspace_name", "true"),
    ("bar_audio", "true"),
    ("bar_media", "true"),
    ("bar_clock", "true"),
    ("bar_battery", "true"),
    ("bar_memory", "true"),
    ("bar_tray", "true"),
    ("clock_24_hour", "false"),
    ("clock_seconds", "false"),
    // launcher
    ("launcher_width", "620"),
    ("launcher_rows", "9"),
    ("launcher_descriptions", "false"),
    ("launcher_icons", "true"),
    ("launcher_remember_query", "false"),
    ("launcher_commands", "true"),
    ("command_settings", "true"),
    ("command_colors", "true"),
    ("command_tmux", "true"),
    ("command_wallpapers", "true"),
    // wallpaper
    ("wallpaper_folder", "~/Pictures/Wallpapers"),
    ("wallpaper_transition", "circle"),
    ("wallpaper_duration", "1200"),
    ("wallpaper_shuffle", "false"),
    ("wallpaper_shuffle_minutes", "30"),
    // behavior
    ("launcher_close_on_launch", "true"),
    ("launcher_escape_clears", "true"),
    ("click_outside_dismiss", "false"),
    ("animation_speed", "1"),
    ("reduce_motion", "false"),
    // floating widgets
    ("floating_visibility", "desktop"),
    ("floating_scale", "1"),
    ("floating_opacity", "1"),
    ("floating_lock_placement", "false"),
    ("widget_clock", "true"),
    ("widget_weather", "true"),
    ("widget_cpu_temperature", "true"),
    ("widget_cpu_usage", "true"),
    ("widget_gpu_temperature", "true"),
    ("widget_uv", "true"),
    ("widget_humidity", "true"),
    ("widget_air_quality", "true"),
    // integrations
    ("integration_gtk", "false"),
    ("integration_terminal", "false"),
    ("integration_tmux", "false"),
    ("integration_vesktop", "false"),
    ("integration_spotify", "false"),
    ("integration_btop", "false"),
    ("integration_cava", "false"),
];

// the shell's settings, kept across restarts as lines like "bar_height=40"
pub struct Settings {
    values: HashMap<String, String>,
}

impl Service for Settings {
    fn new() -> Self {
        let mut values = HashMap::new();

        for (key, value) in DEFAULTS {
            values.insert(String::from(key), String::from(value));
        }

        let saved = fs::read_to_string(path()).unwrap_or_default();

        // keys that are no longer settings are dropped
        for line in saved.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            if let Some(slot) = values.get_mut(key) {
                *slot = String::from(value);
            }
        }

        Self { values }
    }

    // it only changes through input
    fn listen() {}
}

impl Settings {
    pub fn text(&self, key: &str) -> &str {
        self.values.get(key).expect("failed to find setting: unknown key")
    }

    pub fn flag(&self, key: &str) -> bool {
        self.text(key) == "true"
    }

    // a value that doesn't parse falls back to 1, which every number here can live with
    pub fn number(&self, key: &str) -> f32 {
        self.text(key).parse().unwrap_or(1.0)
    }

    pub fn set(key: &str, value: impl ToString) {
        let mut settings = Self::write();

        let slot = settings.values.get_mut(key).expect("failed to find setting: unknown key");

        *slot = value.to_string();

        settings.save();
    }

    pub fn toggle(key: &str) {
        let on = Self::read().flag(key);

        Self::set(key, !on);
    }

    // every setting back to its default
    pub fn reset() {
        let mut settings = Self::write();

        for (key, value) in DEFAULTS {
            settings.values.insert(String::from(key), String::from(value));
        }

        settings.save();
    }

    // losing the file only means the next start uses the defaults
    fn save(&self) {
        let mut text = String::new();

        for (key, _) in DEFAULTS {
            text.push_str(&format!("{key}={}\n", self.values[key]));
        }

        let path = path();

        let folder = path.parent().expect("failed to find the state folder");

        let _ = fs::create_dir_all(folder);
        let _ = fs::write(&path, text);
    }
}

fn path() -> PathBuf {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    PathBuf::from(format!("{home}/.local/state/amane/settings"))
}
