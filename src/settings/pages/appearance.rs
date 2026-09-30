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

    switch::add(
        page,
        "reduce_transparency",
        "Reduce transparency",
        "Solid wallpaper picker background with no blur",
    );

    page.end_group();
}
