//! # textbox
//!
//! An elegant utility for slotting text and images into named slots in templates.
//!
//! ## Quick Start
//!
//! ```no_run
//! use textbox::{TextBox, TextArea, ImageArea, TextBoxRender, Align};
//! use image::open;
//! use pixelset::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load your template image
//! let template = open("template.png")?;
//!
//! // Create a TextBox with named components
//! let textbox = TextBox::new(template)
//!     .text_component("title", TextArea::new(
//!         (10, 10),
//!         500,
//!         Color::rgb(0, 0, 0),
//!         ab_glyph::FontArc::try_from_slice(include_bytes!("font.ttf"))?,
//!         20.0,
//!         Align::Center
//!     ));
//!
//! // Populate components and render
//! let render = TextBoxRender::new()
//!     .text("title", "Hello, World!");
//!
//! let result = textbox.render(render)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Overview
//!
//! A `TextBox` consists of:
//! - A **template image** — your background or base design
//! - A list of **components** — named placeholders defining where content goes
//!
//! Components can be either text areas or image overlays. At render time, you provide
//! values for each named component, and `TextBox` composites them onto the template.

use path_image::PathImage;
use serde::{Deserialize, Serialize};
use crate::{component::{ImageComponent, TextBoxComponent, TextComponent}, error::ComponentNotFoundError, imagearea::ImageArea, textarea::TextArea};

pub use crate::render::TextBoxRender;

pub mod textarea;
pub mod imagearea;

mod component;
mod render;
mod error;

/// A layout container consisting of named text and image components that
/// can be applied to a template.
/// 
/// ## Overview
///
/// A `TextBox` consists of:
///
/// - A **template image** — your background.
/// - A list of **components** — named placeholders defining *where* content goes.
///
/// A `TextBox` acts as a declarative template, allowing you to insert text and
/// images into predefined slots to render an image.
#[derive(Clone, Serialize, Deserialize)]
pub struct TextBox {
    /// The template that components are rendered onto.
    pub template: PathImage,
    /// The list of components that make up this template.
    pub components: Vec<TextBoxComponent>
}

impl TextBox {
    /// Creates an empty `TextBox` from a template with no components.
    pub fn new(template: PathImage) -> Self {
        Self {
            template,
            components: vec![]
        }
    }

    /// Adds a text component with the given `name` and rendering `area`.
    pub fn text_component(mut self, name: &str, area: TextArea) -> Self {
        self.components.push(TextBoxComponent::Text(TextComponent {
            name: name.to_owned(),
            area
        }));
        self
    }

    /// Adds an image component with the given `name` and overlay `area`.
    pub fn image_component(mut self, name: &str, area: ImageArea) -> Self {
        self.components.push(TextBoxComponent::Image(ImageComponent {
            name: name.to_owned(),
            area
        }));
        self
    }

    /// Retrieves a reference to a component by its name.
    pub fn component(&self, name: &str) -> Result<&TextBoxComponent, ComponentNotFoundError> {
        let component = self.components.iter()
            .find(|comp| comp.name() == name)
        .ok_or(ComponentNotFoundError::new(name))?;

        Ok(component)
    }
}