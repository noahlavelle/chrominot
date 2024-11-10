use crate::rendering::canvas::Canvas;
use crate::rendering::Paint;
use crate::rendering::render_commands::SolidColor;

pub enum RenderCommand {
    SolidColor(SolidColor),
}

impl Paint for RenderCommand {
    fn paint(&self, canvas: &mut Canvas) {
        match self {
            RenderCommand::SolidColor(solid_color_command) => solid_color_command.paint(canvas),
        }
    }
}