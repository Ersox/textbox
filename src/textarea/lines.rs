use crate::textarea::{TextArea, text::{Char, CharFormatting, Text}, size::Bounds};

impl TextArea {
    /// Splits text into wrapped lines based on maximum width.
    pub fn to_lines(&self, text: &Text) -> Vec<Text> {
        let chunks = text.split(" ");

        let mut lines: Vec<Text> = vec![];
        let mut component = Text::new();

        for chunk in chunks {
            if component.is_empty() {
                component = chunk;

                let Bounds { width, .. } = self.size(&component);
                if width >= self.max_width {
                    lines.push(component);
                    component = Text::new();
                    continue;
                }

                continue;
            }

            let mut text = component.clone();
            text.chars.push(Char { 
                value: ' ', formatting: CharFormatting::new() 
            });
            text.extend(chunk.clone());
            let Bounds { width, .. } = self.size(&text);

            if width >= self.max_width {
                lines.push(component);
                component = chunk;
                continue;
            }

            component = text;
        }

        if !component.is_empty() {
            lines.push(component);
        }

        lines  
    }
}