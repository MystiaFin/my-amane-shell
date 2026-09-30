use amane::{Column, children};

use super::slider::{self, Range};
use super::switch;
use crate::theme::Theme;

// how solid the shell's surfaces are
pub fn view(theme: &Theme, width: f32) -> Column {
    Column::new(children![
        slider::row(
            theme,
            width,
            "surface_opacity",
            "Surface opacity",
            "Make the shell's surfaces more or less see-through",
            Range {
                min: 0.6,
                max: 1.0,
                step: 0.02,
                label: slider::plain,
            },
        ),
        slider::row(
            theme,
            width,
            "blur_strength",
            "Blur strength",
            "How much the wallpaper blurs behind the wallpaper picker",
            Range {
                min: 0.0,
                max: 1.0,
                step: 0.05,
                label: slider::plain,
            },
        ),
        switch::row(
            theme,
            width,
            "reduce_transparency",
            "Reduce transparency",
            "Opaque surfaces and no wallpaper blur",
        ),
    ])
    .gap(12.0)
}
