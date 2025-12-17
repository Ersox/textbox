use std::error::Error;

use image::DynamicImage;
use imageproc::drawing::draw_text_mut;
use ab_glyph::{Font, FontVec, VariableFont};

pub use crate::textarea::align::Align;
use crate::textarea::{TextArea, text::Text, size::Bounds};

fn has_variation(font: &FontVec, variation: &[u8; 4]) -> bool {
    font.variations().iter().any(|var| &var.tag == variation) 
}

impl TextArea {
    fn draw_line(
        &self,
        image: &mut DynamicImage,
        mut x: i32,
        y: i32,
        line: &Text
    ) -> Result<(), Box<dyn Error>> {
        let mut font = FontVec::try_from_vec(self.font.font_data().to_vec())?;

        for char in &line.chars {
            let Bounds { width, .. } = self.size(char);

            if has_variation(&font, b"wght") {
                let weight = if char.formatting.bold { 700.0 } else { 400.0 };
                font.set_variation(b"wght", weight);
            }

            if has_variation(&font, b"ital") {
                let italic = if char.formatting.italic { 1.0 } else { 0.0 };
                font.set_variation(b"ital", italic);
            } else if has_variation(&font, b"slnt") {
                let slant = if char.formatting.italic { -10.0 } else { 0.0 };
                font.set_variation(b"slnt", slant);
            }

            draw_text_mut(
                image, 
                char.formatting.color.unwrap_or(self.color).0, 
                x, y, 
                self.scale, 
                &font, 
                &char.value.to_string()
            );
            x += width as i32;
        }

        Ok(())
    }

    /// Draws the given text inside the configured area.
    /// 
    /// Parses and styles the given text as markdown, and
    /// wraps and aligns the text as designated.
    pub fn draw(&self, image: &mut DynamicImage, text: &str) -> Result<(), Box<dyn Error>> {
        let text = Text::parse(text)?;

        let lines = self.to_lines(&text);

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

            self.draw_line(image, x as i32, y as i32, &line)?;

            let line_space = (self.scale.y / 4.) as u32;
            
            y += height;
            y += line_space;
        }

        Ok(())
    }
}