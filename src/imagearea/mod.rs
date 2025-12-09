use image::{DynamicImage, imageops::overlay};

/// A rectangular anchor point for placing an image onto another image.
#[derive(Clone, Copy)]
pub struct ImageArea {
    pub x: u32,
    pub y: u32
}

impl ImageArea {
    /// Creates a new `ImageArea` at the given `(x, y)` coordinate.
    pub fn new((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }

    /// Overlays an image (`added`) onto another (`image`) at this area's position.
    pub fn overlay(&self, image: &mut DynamicImage, added: &DynamicImage) {
        let ImageArea { x, y } = *self;
        overlay(image, added, x as i64, y as i64);
    }
}