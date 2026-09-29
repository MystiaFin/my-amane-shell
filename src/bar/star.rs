use amane::{Canvas, Color, Path, Shape, shapes};

// how far the tips reach from the middle
const TIP: f32 = 7.5;

// how close the curved sides pull in toward the middle
const WAIST: f32 = 1.2;

/*
 * a four pointed star, drawn as a shape instead of a font glyph:
 * a glyph is never exactly centered in its box, so turning it
 * swings it off center, while a shape turns around its true middle
 */
pub fn view(size: f32, degrees: f32, color: Color) -> Canvas {
    let middle = size / 2.0;

    let turn = degrees.to_radians();

    // a point at `distance` from the middle, `angle` degrees clockwise from straight up
    let point = |angle: f32, distance: f32| {
        let angle = angle.to_radians() + turn;

        let x = middle + distance * angle.sin();
        let y = middle - distance * angle.cos();

        (x, y)
    };

    let (start_x, start_y) = point(0.0, TIP);

    let mut star = Path::new().move_to(start_x, start_y);

    for tip in 1..=4 {
        let angle = tip as f32 * 90.0;

        // the handle sits between two tips, close to the middle, so the side curves inward
        let (handle_x, handle_y) = point(angle - 45.0, WAIST);
        let (tip_x, tip_y) = point(angle, TIP);

        star = star.quad_to(handle_x, handle_y, tip_x, tip_y);
    }

    Canvas::new()
        .width(size)
        .height(size)
        .shapes(shapes![star.close().fill(color)])
}
