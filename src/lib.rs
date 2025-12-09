use std::error::Error;
use image::DynamicImage;

use crate::{component::{ImageComponent, TextBoxComponent, TextComponent}, imagearea::ImageArea, textarea::TextArea};

pub use crate::render::TextBoxRender;

pub mod textarea;
pub mod imagearea;

mod component;
mod render;

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
pub struct TextBox {
    /// The template that components are rendered onto.
    pub template: DynamicImage,
    /// The list of components that make up this template.
    pub components: Vec<TextBoxComponent>
}

impl TextBox {
    /// Creates an empty `TextBox` from a template with no components.
    pub fn new(template: DynamicImage) -> Self {
        Self {
            template,
            components: vec![]
        }
    }

    /// Adds a text component with the given `name` and rendering `area`.
    pub fn add_text_component(mut self, name: &str, area: TextArea) -> Self {
        self.components.push(TextBoxComponent::Text(TextComponent {
            name: name.to_owned(),
            area
        }));
        self
    }

    /// Adds an image component with the given `name` and overlay `area`.
    pub fn add_image_component(mut self, name: &str, area: ImageArea) -> Self {
        self.components.push(TextBoxComponent::Image(ImageComponent {
            name: name.to_owned(),
            area
        }));
        self
    }

    /// Retrieves a reference to a component by its name.
    pub fn component(&self, name: &str) -> Result<&TextBoxComponent, Box<dyn Error>> {
        let component = self.components.iter()
            .find(|comp| comp.name() == name)
            .ok_or(format!("Could not find component {}", name))?;

        Ok(component)
    }
}