use imageproc::drawing::text_size;

use crate::textarea::TextArea;

/// The bounds of generated text.
pub struct Bounds {
    /// Width of the given text.
    pub width: u32,
    /// Height of the given text.
    pub height: u32,
}

impl TextArea {
    /// Returns the pixel size of the given text at this scale.
    pub fn size(&self, text: &str) -> Bounds {
        let (width, height) = text_size(self.scale, &self.font, text);
        Bounds {
            width,
            height
        }
    }
}