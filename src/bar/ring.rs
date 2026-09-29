use amane::{Arc, Canvas, Cap, Color, Shape, shapes};

// a thin track with the value drawn over it, from the top clockwise
pub fn view(size: f32, thickness: f32, value: f32, color: Color, track: Color) -> Canvas {
    let middle = size / 2.0;

    // the stroke is centered on the radius, so half of it sits outside
    let radius = middle - thickness / 2.0;

    let sweep = 360.0 * value.clamp(0.0, 1.0);

    Canvas::new().width(size).height(size).shapes(shapes![
        Arc::new()
            .center(middle, middle)
            .radius(radius)
            .stroke(thickness, track),
        Arc::new()
            .center(middle, middle)
            .radius(radius)
            .sweep(sweep)
            .stroke(thickness, color)
            .cap(Cap::Round),
    ])
}
