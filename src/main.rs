use std::fs::File;
use std::io::BufWriter;
use image::{ImageBuffer, Rgba};
use image::DynamicImage::ImageRgba8;
use image::ImageFormat::Png;
use crate::rendering::paint;

mod rendering;
mod dom;
mod parsing;

fn main() {
    let filename = "output.png";
    let mut file = BufWriter::new(File::create(&filename).unwrap());
    if write_to_output(&mut file) {
        println!("Successfully wrote to {}", filename);
    } else {
        println!("Error writing to {}", filename);
    }
}

fn write_to_output(file: &mut BufWriter<File>) -> bool {
    let canvas = paint();
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = ImageBuffer::from_fn(w, h, move |x, y| {
        let color = &canvas.pixels[(y * w + x) as usize];
        Rgba([color.r, color.g, color.b, color.a])
    });
    ImageRgba8(img).write_to(file, Png).is_ok()
}
