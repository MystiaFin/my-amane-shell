use amane::{Column, children};

use super::slider::{self, Range};
use super::{choice, switch};
use crate::theme::Theme;

// the cards on the desktop
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        choice::row(
            theme,
            width,
            "floating_visibility",
            "Visibility",
            "Show the cards on an empty desktop, always, or never",
            &[("Desktop", "desktop"), ("Always", "always"), ("Hidden", "hidden")],
        ),
        slider::row(
            theme,
            width,
            "floating_scale",
            "Scale",
            "The cards' size",
            Range {
                min: 0.7,
                max: 1.35,
                step: 0.05,
                label: slider::times,
            },
        ),
        slider::row(
            theme,
            width,
            "floating_opacity",
            "Opacity",
            "How solid the whole card layer is",
            Range {
                min: 0.35,
                max: 1.0,
                step: 0.05,
                label: slider::plain,
            },
        ),
        switch::row(
            theme,
            width,
            "floating_lock_placement",
            "Lock placement",
            "Keep the cards where they are when the wallpaper changes",
        ),
        switch::row(theme, width, "widget_clock", "Clock", "The large desktop clock"),
        switch::row(theme, width, "widget_weather", "Weather", "Current weather"),
        switch::row(theme, width, "widget_cpu_temperature", "CPU temperature", "Processor heat"),
        switch::row(theme, width, "widget_cpu_usage", "CPU usage", "How busy the processor is"),
        switch::row(
            theme,
            width,
            "widget_gpu_temperature",
            "GPU temperature",
            "Only on machines with a GPU sensor",
        ),
        switch::row(theme, width, "widget_uv", "UV index", "From the weather data"),
        switch::row(theme, width, "widget_humidity", "Humidity", "From the weather data"),
        switch::row(theme, width, "widget_air_quality", "Air quality", "From the weather data"),
    ])
    .gap(12.0)
}
