use markdown::mdast::Node;
use thiserror::Error;

/// Errors that can occur when adding a Markdown AST node to `Text`.
///
/// Returned when the Markdown parser encounters node types that are not currently supported.
/// Currently supported nodes are: text, paragraph, strong (bold), emphasis (italic), and links.
#[derive(Debug, Error)]
pub enum AddNodeError {
    /// A Markdown node type that cannot be processed was encountered.
    #[error("Unprocessed Markdown node: {0:?}")]
    UnprocessedNode(Node),
}