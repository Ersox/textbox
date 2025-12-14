use thiserror::Error;

#[derive(Debug, Error)]
#[error("component `{name}` not found in TextBox")]
pub struct ComponentNotFoundError {
    pub name: String,
}

impl ComponentNotFoundError {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Error)]
pub enum TextBoxRenderError {
    #[error(
        "component `{name}` expects {expected} but was not provided a value of that type"
    )]
    ComponentTypeMismatch {
        name: String,
        expected: &'static str,
    },
}
