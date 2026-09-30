use super::super::page::Page;
use super::super::slider::{self, Range};
use super::super::switch;

pub fn build(page: &mut Page) {
    slider::add(
        page,
        "blur_strength",
        "Blur strength",
        "Control wallpaper blur behind the wallpaper picker",
        Range {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            label: slider::plain,
        },
    );

    slider::add(
        page,
        "surface_opacity",
        "Surface opacity",
        "Make shell surfaces more or less transparent",
        Range {
            min: 0.6,
            max: 1.0,
            step: 0.02,
            label: slider::plain,
        },
    );

    switch::add(
        page,
        "reduce_transparency",
        "Reduce transparency",
        "Force opaque surfaces and disable wallpaper blur",
    );

    page.end_group();
}
