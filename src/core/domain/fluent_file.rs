use fluent4rs::prelude::*;

use thiserror::*;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum FluentFileError {
    #[error("fluent file read failed - reason: {0}")]
    UnableToRead(String),

    #[error("fluent file parse failed - reason: {0}")]
    UnableToParse(String),
}

pub struct FluentFile {
    resource: Resource,
}

impl FluentFile {
    fn new(resource: Resource) -> Self {
        Self { resource }
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}

impl TryFrom<&PathBuf> for FluentFile {
    type Error = FluentFileError;

    fn try_from(value: &PathBuf) -> std::result::Result<Self, Self::Error> {
        let content = std::fs::read_to_string(value)
            .map_err(|e| FluentFileError::UnableToRead(e.to_string()))?;

        let resource = Parser::parse(content.as_str())
            .map_err(|e| FluentFileError::UnableToParse(e.to_string()))?;

        Ok(Self::new(resource))
    }
}
