use amane::{Color, Image, Parent, Pointer, Rectangle, Service};

use super::Picker;

/*
 * thumbnails stay cached so the picker opens fast, all of them at once,
 * so they are only as big as the largest card
 */
// ponytail: one screen pixel per card pixel, double it for a scaled screen
const THUMBNAIL_WIDTH: u32 = super::CARD_WIDTH as u32;
const THUMBNAIL_HEIGHT: u32 = super::CARD_HEIGHT as u32;

const RADIUS: f32 = 16.0;

// how dark a card at the very edge gets
const EDGE_SHADE: f32 = 0.78;

// where a card sits, from the middle of the band
pub struct Place {
    pub x: f32,
    pub y: f32,

    pub width: f32,
    pub height: f32,

    // 1 in the middle, 0 at either end of the carousel
    pub prominence: f32,

    // how many cards it is from the middle one, to slide there when clicked
    pub offset: i64,
}

// the middle card is big and bright, the ones further out small and shaded
pub fn view(path: &str, place: Place) -> Rectangle {
    let prominence = place.prominence;

    let scale = 0.4 + prominence * 0.95;
    let opacity = 0.55 + prominence * 0.45;
    let shade = (1.0 - prominence) * EDGE_SHADE;

    /*
     * fading the card as well as shading it would draw each card on a canvas
     * of its own, far too slow for a carousel, and the band behind is dark
     * anyway, so the fade is folded into the shade
     */
    let darkness = 1.0 - opacity * (1.0 - shade);

    let shading_alpha = (darkness * 255.0).round() as u8;

    let offset = place.offset;

    let picture = Image::cover(path).thumbnail(THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT);

    let shading = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .radius(RADIUS)
        .fill(Color::rgba(0, 0, 0, shading_alpha));

    Rectangle::new()
        .width(place.width)
        .height(place.height)
        .radius(RADIUS)
        .fill(picture)
        .scale(scale)
        .translate(place.x - place.width / 2.0, place.y - place.height / 2.0)
        .cursor(Pointer)
        .on_click(move |_| Picker::write().slide(offset))
        .child(shading)
}
