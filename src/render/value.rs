use image::DynamicImage;

/// A value assigned to a named `TextBoxComponent`.
///
/// Components may expect either text or an image, and this enum is used to
/// store whichever type of value the caller provides.
pub enum TextBoxComponentValue {
    /// A textual value for a text component.
    Text(String),

    /// An image value for an image component.
    Image(DynamicImage)
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

    /// Consumes the value and returns the inner `DynamicImage` if it is an image.
    ///
    /// Returns `None` if the value is text instead.
    pub fn as_image(self) -> Option<DynamicImage> {
        match self {
            Self::Text(_) => None,
            Self::Image(image) => Some(image)
        }
    }
}