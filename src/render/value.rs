use image::DynamicImage;
use path_image::PathImage;
use serde::{Deserialize, Serialize};

/// A value assigned to a named `TextBoxComponent`.
///
/// Components may expect either text or an image, and this enum is used to
/// store whichever type of value the caller provides.
#[derive(Clone, Serialize, Deserialize)]
pub enum TextBoxComponentValue {
    /// A textual value for a text component.
    Text(String),

    /// An image value for an image component.
    Image(PathImage)
}

impl TextBoxComponentValue {
    /// Consumes the value and returns the inner `String` if it is text.
    ///
    /// Returns `None` if the value is an image instead.
    pub fn as_text(self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text),
            Self::Image(_) => None
        }
    }

    /// Consumes the value and returns the inner `PathImage` if it is an image.
    ///
    /// Returns `None` if the value is text instead.
    pub fn as_image(self) -> Option<PathImage> {
        match self {
            Self::Text(_) => None,
            Self::Image(image) => Some(image)
        }
    }
}