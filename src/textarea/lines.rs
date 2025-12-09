use crate::textarea::{TextArea, size::Bounds};

impl TextArea {
    /// Splits text into wrapped lines based on maximum width.
    pub fn to_lines(&self, text: &str) -> Vec<String> {
        let chunks = text.split(" ");

        let mut lines: Vec<String> = vec![];
        let mut component = String::new();

        for chunk in chunks {
            if component.is_empty() {
                component = chunk.to_string();

                let Bounds { width, .. } = self.size(&component);
                if width >= self.max_width {
                    lines.push(component);
                    component = String::new();
                    continue;
                }

                component = chunk.to_string();
                continue;
            }

            let text = format!("{} {}", component, chunk);
            let Bounds { width, .. } = self.size(&text);

            if width >= self.max_width {
                lines.push(component);
                component = chunk.to_string();
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