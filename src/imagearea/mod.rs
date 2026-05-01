use image::{DynamicImage, imageops::{FilterType, overlay}};
use serde::{Deserialize, Serialize};

/// A configurable image overlaying area with resizing.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ImageArea {
    /// Starting X coordinate of the text box.
    pub x: u32,
    /// Starting Y coordinate of the text box.
    pub y: u32,
    /// Width of the overlay area in pixels.
    pub width: u32,
    /// Height of the overlay area in pixels.
    pub height: u32
}

impl ImageArea {
    /// Creates a new `ImageArea` at the given `(x, y)` coordinate.
    pub fn new((x, y): (u32, u32), (width, height): (u32, u32))-> Self {
        Self { x, y, width, height }
    }

    /// Overlays an image (`added`) onto another (`image`) at this area's position.
    pub fn overlay(&self, image: &mut DynamicImage, added: &DynamicImage) {
        let ImageArea { x, y, width, height } = *self;
        let added = added.resize_exact(width, height, FilterType::Nearest);
        overlay(image, &added, x as i64, y as i64);
    }
}