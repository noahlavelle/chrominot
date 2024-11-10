#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

pub(crate) trait Paint {
    fn paint(&self, canvas: &mut Canvas);
}

pub struct Canvas {
    pub(crate) pixels: Vec<Color>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let white = Color { r: 255, g: 255, b: 255, a: 255 };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }
}