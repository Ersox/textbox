use ab_glyph::{FontArc, PxScale};

pub use crate::textarea::align::Align;

mod lines;
mod size;
mod align;
mod draw;

/// A configurable text drawing area with wrapping and alignment.
pub struct TextArea {
    /// Starting X coordinate of the text box.
    pub x: u32,
    /// Starting Y coordinate of the text box.
    pub y: u32,
    /// Maximum width allowed before wrapping.
    pub max_width: u32,
    /// Font scale used for rendering text.
    pub scale: PxScale,
    /// Font used when drawing text.
    pub font: FontArc,
    /// Horizontal alignment of each line.
    pub align: Align,
}

impl TextArea {
    /// Creates a new `TextArea` at the given position and width using the specified font and scale.
    pub fn new(
        (x, y): (u32, u32), 
        max_width: u32, 
        font: FontArc, 
        scale: impl Into<PxScale>,
        align: Align
    ) -> Self {
        Self {
            x,
            y,
            max_width,
            font,
            scale: scale.into(),
            align
        }
    }
}
