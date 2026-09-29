// the window's edges and up to 8 rounded rectangles, melted together

// how far point is outside the rounded rectangle, negative inside
fn rounded_rectangle(point: vec2<f32>, rect: vec4<f32>, radius: f32) -> f32 {
    let half = rect.zw * 0.5;
    let center = rect.xy + half;

    let safe_radius = min(radius, min(half.x, half.y));

    let offset = abs(point - center) - half + safe_radius;

    let inside = min(max(offset.x, offset.y), 0.0);
    let outside = length(max(offset, vec2<f32>(0.0)));

    return inside + outside - safe_radius;
}

// like min(), but the two shapes flow into each other within radius of where they meet
fn smooth_union(first: f32, second: f32, radius: f32) -> f32 {
    let safe_radius = max(radius, 0.001);

    let blend = clamp(0.5 + 0.5 * (second - first) / safe_radius, 0.0, 1.0);

    return mix(second, first, blend) - safe_radius * blend * (1.0 - blend);
}

/*
 * values, one row of four each, as liquid.rs writes them:
 * 0 the color, 1 edge offset, connection radius and blob count,
 * 2 to 9 the blobs as x, y, width, height, 10 and 11 their radii
 */
@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color = values[0];

    let edge_offset = values[1].x;
    let connection = values[1].y;
    let count = u32(values[1].z);

    let point = uv * size;

    // the edges sit just outside the window, so a blob only joins them once it comes near
    let edge = min(min(point.x, size.x - point.x), min(point.y, size.y - point.y));

    var distance = edge + edge_offset;

    for (var index = 0u; index < count; index++) {
        let radius = values[10u + index / 4u][index % 4u];

        let blob = rounded_rectangle(point, values[2u + index], radius);

        distance = smooth_union(distance, blob, connection);
    }

    // about one pixel of soft edge, however the shape is scaled
    let softness = max(fwidth(distance), 0.001);

    let coverage = 1.0 - smoothstep(-softness, softness, distance);

    return vec4<f32>(color.rgb, color.a * coverage);
}
