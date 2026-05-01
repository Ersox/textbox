use serde::{Deserialize, Serialize};

/// Horizontal alignment options for text.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Align {
    /// Align lines to the left.
    Left,
    /// Align lines to the right.
    Right,
    /// Center each line.
    Center,
}