use pixelset::Color;

/// Formatting options for a single character, including
/// bold and italic.
#[derive(Clone, Copy, Debug)]
pub struct CharFormatting {
    /// Whether the character is bold.
    pub bold: bool,

    /// Whether the character is italic.
    pub italic: bool,

    /// What color the character should be.
    pub color: Option<Color>
}

impl CharFormatting {
    /// Creates a new `CharFormatting` with no styles applied.
    pub fn new() -> CharFormatting {
        CharFormatting {
            bold: false,
            italic: false,
            color: None
        }
    }

    /// Returns a copy of this formatting with bold enabled.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Returns a copy of this formatting with italic enabled.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Returns a copy of this formatting with a given color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}
