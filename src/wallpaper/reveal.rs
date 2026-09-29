use amane::{Full, Image, Rectangle};

use super::FRAME_RADIUS;

/*
 * the new wallpaper, seen through a circle growing from center; the
 * picture inside is moved back by as much as the circle is moved, so it
 * stays where the old one is while only the circle grows
 */
pub fn view(path: &str, center: (f32, f32), progress: f32, width: f32, height: f32) -> Rectangle {
    let (x, y) = center;

    // far enough to reach the farthest corner, and a little past it
    let farthest_x = f32::max(x, width - x);
    let farthest_y = f32::max(y, height - y);

    let covering = f32::hypot(farthest_x, farthest_y) + 2.0;

    let radius = covering * progress;

    let picture = Rectangle::new()
        .width(width)
        .height(height)
        .radius(FRAME_RADIUS)
        .fill(Image::cover(path))
        .translate(radius - x, radius - y);

    Rectangle::new()
        .width(radius * 2.0)
        .height(radius * 2.0)
        .radius(Full)
        .translate(x - radius, y - radius)
        .clip()
        .child(picture)
}
