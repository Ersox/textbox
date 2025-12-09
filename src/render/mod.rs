use std::{collections::HashMap, error::Error};
use image::DynamicImage;
use crate::{TextBox, component::TextBoxComponent, render::value::TextBoxComponentValue};

mod value;

/// A builder-like structure used when populating a `TextBox` with runtime
/// data before rendering.
///
/// This maps component names to the `TextBoxComponentValue` that will be
/// substituted during rendering.
pub struct TextBoxRender {
    /// Mapping of component names to their assigned values.
    pub map: HashMap<String, TextBoxComponentValue>
}

impl TextBoxRender {
    /// Creates an empty `TextBoxRender`.
    pub fn new() -> Self {
        Self { 
            map: HashMap::new()
        }
    }

    /// Inserts a text value for the component with the given name.
    ///
    /// Returns `self` to support method chaining.
    pub fn put_text(mut self, name: &str, value: &str) -> Self {
        self.map.insert(
            name.to_owned(),
            TextBoxComponentValue::Text(value.to_owned())
        );
        self
    }

    /// Inserts an image value for the component with the given name.
    ///
    /// Returns `self` to support method chaining.
    pub fn put_image(mut self, name: &str, image: DynamicImage) -> Self {
        self.map.insert(
            name.to_owned(),
            TextBoxComponentValue::Image(image)
        );
        self
    }
}

impl TextBox {
    /// Renders this `TextBox` onto the template image, applying all
    /// component values specified in the provided `TextBoxRender`.
    ///
    /// Each entry in the render map is matched to a component by name.  
    /// A type mismatch (e.g., providing text for an image component) results in an error.
    /// 
    /// Returns the rendered `DynamicImage`.
    pub fn render(&self, render: TextBoxRender) -> Result<DynamicImage, Box<dyn Error>> {
        let mut image = self.template.clone();
        for (name, value) in render.map {
            let component = self.component(&name)?;

            match component {
                TextBoxComponent::Text(component) => {
                    let value = value.as_text()
                        .ok_or("Misalignment")?;
                    component.area.draw(&mut image, &value);
                },
                TextBoxComponent::Image(component) => {
                    let value = value.as_image()
                        .ok_or("Misalignment")?;
                    component.area.overlay(&mut image, &value);
                }
            }
        }

        Ok(image)
    }
}