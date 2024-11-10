use crate::rendering::canvas::{Canvas, Color, Rect};
use crate::rendering::Paint;

pub struct SolidColor {
    pub(crate) color: Color,
    pub(crate) rect: Rect,
}

impl Paint for SolidColor {
    fn paint(&self, canvas: &mut Canvas) {
        let x0 = self.rect.x.clamp(0, canvas.width);
        let y0 = self.rect.y.clamp(0, canvas.height);
        let x1 = (self.rect.x + self.rect.w).clamp(0, canvas.width);
        let y1 = (self.rect.y + self.rect.h).clamp(0, canvas.height);

        for y in y0 .. y1 {
            for x in x0 .. x1 {
                canvas.pixels[x + y * canvas.width] = self.color;
            }
        }
    }
}