use std::fmt::{self, Display, Formatter, Write};

use crate::textarea::text::formatting::CharFormatting;

/// A single character along with its associated formatting.
///
/// Combines a `char` value with style information such as bold 
/// or italic.
#[derive(Clone, Copy, Debug)]
pub struct Char {
    /// The unicode character value.
    pub value: char,
    
    /// Formatting applied to this character.
    pub formatting: CharFormatting
}

impl Char {
    /// Creates a new `Char` with the given value and formatting.
    pub fn new(value: char, formatting: CharFormatting) -> Self {
        Self { 
            value, 
            formatting 
        }
    }
}

impl Display for Char {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_char(self.value)?;
        Ok(())
    }
}