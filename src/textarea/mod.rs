use ab_glyph::{FontArc, PxScale};
use path_image::PathFont;
use pixelset::Color;
use serde::{Deserialize, Serialize};

pub use crate::textarea::align::Align;
pub use crate::textarea::scale::Scale;

mod scale;
mod text;
mod lines;
mod size;
mod align;
mod draw;

/// A configurable text drawing area with wrapping and alignment.
#[derive(Clone, Serialize, Deserialize)]
pub struct TextArea {
    /// Starting X coordinate of the text box.
    pub x: u32,
    /// Starting Y coordinate of the text box.
    pub y: u32,
    /// Maximum width allowed before wrapping.
    pub max_width: u32,
    /// Font scale used for rendering text.
    pub scale: Scale,
    /// Font used when drawing text.
    pub font: PathFont,
    /// Horizontal alignment of each line.
    pub align: Align,
    /// Color to draw the text in.
    pub color: Color
}

impl TextArea {
    /// Creates a new `TextArea` at the given position and width using the specified font and scale.
    pub fn new(
        (x, y): (u32, u32), 
        max_width: u32, 
        color: Color,
        font: PathFont, 
        scale: impl Into<Scale>,
        align: Align
    ) -> Self {
        Self {
            x,
            y,
            max_width,
            color,
            font,
            scale: scale.into(),
            align
        }
    }
}
