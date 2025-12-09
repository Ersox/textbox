use image::DynamicImage;
use imageproc::drawing::draw_text_mut;
use pixelset::color::BLACK;

pub use crate::textarea::align::Align;
use crate::textarea::{TextArea, size::Bounds};

impl TextArea {
    /// Draws the given text inside the configured area with wrapping.
    pub fn draw(&self, image: &mut DynamicImage, text: &str) {
        let lines = self.to_lines(text);

        let mut y = self.y;
        for line in lines {
            let Bounds { width, height } = self.size(&line);

            let x = match self.align {
                Align::Left => {
                    self.x
                },
                Align::Right => {
                    self.x + self.max_width - width
                },
                Align::Center => {
                    self.x + (self.max_width / 2) - (width / 2)
                }
            };

            draw_text_mut(
                image, 
                BLACK.0, 
                x as i32, y as i32, 
                self.scale, 
                &self.font, 
                &line
            );

            let line_space = (self.scale.y / 4.) as u32;
            
            y += height;
            y += line_space;
        }
    }
}