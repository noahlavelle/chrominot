use crate::rendering::{Color, Rect, RenderCommand};
use crate::rendering::render_commands::SolidColor;

type CommandBuffer = Vec<RenderCommand>;

pub fn build_command_buffer() -> CommandBuffer {
    let mut buffer = Vec::new();
    render_layout_box(&mut buffer);
    buffer
}

fn render_layout_box(buffer: &mut CommandBuffer) {
    buffer.push(RenderCommand::SolidColor(
        SolidColor {
            color: Color { r: 254, g: 0, b: 0, a: 255 },
            rect: Rect { x: 0, y: 0, w: 500, h: 500 }
        }
    ));
}
