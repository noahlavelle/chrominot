mod canvas;
mod command_buffer;
mod render_command;
mod render_commands;

use crate::rendering::command_buffer::build_command_buffer;
use crate::rendering::canvas::{Canvas, Color, Rect, Paint};
use crate::rendering::render_command::RenderCommand;

pub fn paint() -> Canvas {
    let command_buffer = build_command_buffer();
    let mut canvas = Canvas::new(1920, 1080);
    for item in command_buffer {
        item.paint(&mut canvas);
    }
    canvas
}