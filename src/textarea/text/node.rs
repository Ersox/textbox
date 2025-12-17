use std::error::Error;

use markdown::mdast::Node;
use pixelset::Color;

use crate::textarea::text::{Text, char::Char, error::AddNodeError, formatting::CharFormatting};

impl Text {
    /// Recursively adds the content of a Markdown AST `Node` to this `Text`,
    /// applying the given formatting and preserving styles such as bold and italic.
    pub fn add_node(
        &mut self, 
        node: &Node, 
        formatting: CharFormatting
    ) -> Result<(), Box<dyn Error>> {
        match node {
            Node::Root(root) => {
                for child in &root.children {
                    self.add_node(child, formatting)?;
                }
            },
            Node::Paragraph(paragraph) => {
                for child in &paragraph.children {
                    self.add_node(child, formatting)?;
                }
            },
            Node::Text(text) => {
                for char in text.value.chars() {
                    self.chars.push(Char::new(char, formatting));
                }
            },
            Node::Strong(strong) => {
                for child in &strong.children {
                    self.add_node(child, formatting.bold())?;
                }
            },
            Node::Emphasis(emphasis) => {
                for child in &emphasis.children {
                    self.add_node(child, formatting.italic())?;
                }
            },
            Node::Link(link) => {
                let color = Color::hex(&link.url)?;
                for child in &link.children {
                    self.add_node(child, formatting.color(color))?;
                }
            }
            _ => {
                return Err(Box::new(
                    AddNodeError::UnprocessedNode(node.clone())
                ));
            }
        }

        Ok(())
    }
}