use serde::{Deserialize, Serialize, ser};

use crate::{imagearea::ImageArea, textarea::TextArea};

/// A named text component within a `TextBox` template.
///
/// Defines where and how text should be rendered when populating a `TextBox`.
#[derive(Clone, Serialize, Deserialize)]
pub struct TextComponent {
    /// The unique identifier for this component.
    pub name: String,
    /// The rendering configuration for the text (position, font, alignment, etc.).
    pub area: TextArea
}

/// A named image component within a `TextBox` template.
///
/// Defines where and how an image should be overlaid when populating a `TextBox`.
#[derive(Clone, Serialize, Deserialize)]
pub struct ImageComponent {
    /// The unique identifier for this component.
    pub name: String,
    /// The rendering configuration for the image (position and size).
    pub area: ImageArea
}

/// A component within a `TextBox` that can hold either text or an image.
///
/// Each component is identified by name and can accept a corresponding value
/// when rendering a `TextBox`.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TextBoxComponent {
    /// A text rendering component.
    Text(TextComponent),
    /// An image overlay component.
    Image(ImageComponent)
}

impl TextBoxComponent {
    /// Returns the name of this component.
    pub fn name(&self) -> &str {
        match self {
            Self::Text(text) => &text.name,
            Self::Image(image) => &image.name
        }
    }
}