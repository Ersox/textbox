use markdown::mdast::Node;
use thiserror::Error;

/// Errors that can occur when adding a Markdown AST node to `Text`.
#[derive(Debug, Error)]
pub enum AddNodeError {
    #[error("Unprocessed Markdown node: {0:?}")]
    UnprocessedNode(Node),
}