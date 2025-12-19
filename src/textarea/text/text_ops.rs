use std::{error::Error, fmt::{self, Display, Formatter, Write}, vec};

use markdown::to_mdast;

use crate::textarea::text::{Text, char::Char, formatting::CharFormatting};

impl Text {
    /// Parses a string (Markdown supported) into a `Text` structure.
    ///
    /// Applies formatting such as bold/italic according to Markdown syntax.
    pub fn parse(text: &str) -> Result<Self, Box<dyn Error>> {
        let ast = to_mdast(text, &markdown::ParseOptions::default())
            .unwrap();

        let mut text = Self::new();
        text.add_node(&ast, CharFormatting::new())?;

        Ok(text)
    }

    /// Returns `true` if this `Text` contains no characters.
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// Appends an iterator of `Char`s to this `Text`.
    pub fn extend(&mut self, chars: impl IntoIterator<Item = Char>) {
        self.chars.extend(chars);
    }
}

impl Display for Text {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        for character in &self.chars {
            formatter.write_char(character.value)?;
        }
        Ok(())
    }
}

impl IntoIterator for Text {
    type IntoIter = vec::IntoIter<Char>;
    type Item = Char;
    fn into_iter(self) -> Self::IntoIter {
        self.chars.into_iter()
    }
}