
mod char;
mod formatting;
mod node;
mod text_ops;
mod split;
mod error;

pub use char::Char;
pub use formatting::CharFormatting;

/// A sequence of `Char`s representing styled text.
///
/// `Text` is a higher-level container over individual `Char`s,
/// supporting parsing from Markdown, appending characters, and iteration.
#[derive(Clone, Debug)]
pub struct Text {
    /// The underlying vector of characters with formatting.
    pub chars: Vec<Char>
}

impl Text {
    /// Creates an empty `Text`.
    pub fn new() -> Self {
        Self { chars: vec![] }
    }
}