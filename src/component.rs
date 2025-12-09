use crate::{imagearea::ImageArea, textarea::TextArea};

pub struct TextComponent {
    pub name: String,
    pub area: TextArea
}

pub struct ImageComponent {
    pub name: String,
    pub area: ImageArea
}

pub enum TextBoxComponent {
    Text(TextComponent),
    Image(ImageComponent)
}

impl TextBoxComponent {
    pub fn name(&self) -> &str {
        match self {
            Self::Text(text) => &text.name,
            Self::Image(image) => &image.name
        }
    }
}