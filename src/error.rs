use thiserror::Error;

/// Error returned when a named component is not found in a `TextBox`.
#[derive(Debug, Error)]
#[error("component `{name}` not found in TextBox")]
pub struct ComponentNotFoundError {
    /// The name of the component that was not found.
    pub name: String,
}

impl ComponentNotFoundError {
    /// Creates a new `ComponentNotFoundError` for the given component name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

/// Error returned during rendering when component value types don't match.
#[derive(Debug, Error)]
pub enum TextBoxRenderError {
    /// A component received a value of the wrong type (e.g., text for an image component).
    #[error(
        "component `{name}` expects {expected} but was not provided a value of that type"
    )]
    ComponentTypeMismatch {
        /// The name of the component with the type mismatch.
        name: String,
        /// The expected component type ("text" or "image").
        expected: &'static str,
    },
}
