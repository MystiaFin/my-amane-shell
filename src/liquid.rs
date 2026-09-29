use std::env;

use amane::{Color, Parent, Rectangle, Stack, Widget};

// the shader has room for this many blobs
const MAX_BLOBS: usize = 8;

const EDGE_OFFSET: f32 = 2.0;
const CONNECTION: f32 = 36.0;

// a rounded rectangle of liquid with content on it, like a panel growing out of an edge
pub struct Blob {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    content: Option<Box<dyn Widget>>,
}

impl Blob {
    // placed from the liquid's top-left corner, animate x or y to make it flow
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            radius: 0.0,
            content: None,
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;

        self
    }

    pub fn child(mut self, content: impl Widget + 'static) -> Self {
        self.content = Some(Box::new(content));

        self
    }
}

/*
 * the shader draws every blob melted into the
 * others and into the window's edges, then each blob's content is laid on top
 */
pub fn view(color: Color, blobs: Vec<Blob>) -> Stack {
    assert!(blobs.len() <= MAX_BLOBS, "the liquid holds at most {MAX_BLOBS} blobs");

    let surface = Rectangle::new()
        .width(Parent)
        .height(Parent)
        .shader(shader_path())
        .shader_values(values(color, &blobs));

    let mut layers: Vec<Box<dyn Widget>> = vec![Box::new(surface)];

    for blob in blobs {
        let Some(content) = blob.content else {
            continue;
        };

        // the content is already boxed, and a stack is what takes boxed widgets
        let holder = Rectangle::new()
            .width(blob.width)
            .height(blob.height)
            .translate(blob.x, blob.y)
            .child(Stack::new(vec![content]).width(Parent).height(Parent));

        layers.push(Box::new(holder));
    }

    Stack::new(layers).width(Parent).height(Parent)
}

// laid out the way liquid.wgsl reads them, one row of four numbers at a time
fn values(color: Color, blobs: &[Blob]) -> Vec<[f32; 4]> {
    let color = [
        f32::from(color.red()) / 255.0,
        f32::from(color.green()) / 255.0,
        f32::from(color.blue()) / 255.0,
        f32::from(color.alpha()) / 255.0,
    ];

    let count = blobs.len() as f32;

    let mut values = vec![color, [EDGE_OFFSET, CONNECTION, count, 0.0]];

    let mut radii = [[0.0; 4]; 2];

    for index in 0..MAX_BLOBS {
        let Some(blob) = blobs.get(index) else {
            values.push([0.0; 4]);

            continue;
        };

        values.push([blob.x, blob.y, blob.width, blob.height]);

        radii[index / 4][index % 4] = blob.radius;
    }

    values.extend(radii);

    values
}

// the shader lives next to the config's src folder
fn shader_path() -> String {
    let home = env::var("HOME").expect("failed to find home: HOME is not set");

    format!("{home}/.config/amane/shaders/liquid.wgsl")
}
