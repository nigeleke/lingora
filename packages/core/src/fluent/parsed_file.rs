use std::{fs, path::Path};

use fluent4rs::{
    ast::Resource,
    prelude::{Fluent4rsError, Parser},
};

use crate::{
    domain::{HasLocale, Locale},
    error::LingoraError,
    fluent::FluentFile,
};

pub struct ParsedFluentFile {
    file: FluentFile,
    resource: Result<Resource, Fluent4rsError>,
}

impl ParsedFluentFile {
    pub fn fluent_file(&self) -> &FluentFile {
        &self.file
    }

    pub fn path(&self) -> &Path {
        self.file.path()
    }

    pub fn locale(&self) -> &Locale {
        self.file.locale()
    }

    pub fn resource(&self) -> Option<&Resource> {
        self.resource.as_ref().ok()
    }

    pub fn error_description(&self) -> String {
        self.resource
            .as_ref()
            .map_err(|e| e.to_string())
            .err()
            .unwrap_or(String::default())
    }
}

impl TryFrom<&FluentFile> for ParsedFluentFile {
    type Error = LingoraError;

    fn try_from(file: &FluentFile) -> Result<Self, Self::Error> {
        let file = file.clone();
        let content = fs::read_to_string(file.path())?;
        let resource = Parser::parse(content.as_str());
        Ok(Self { file, resource })
    }
}

impl HasLocale for ParsedFluentFile {
    fn locale(&self) -> &Locale {
        self.file.locale()
    }
}
