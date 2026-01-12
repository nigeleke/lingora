use std::fs;

use fluent4rs::{ast::*, prelude::*};

use crate::{error::LingoraError, fluent::FluentFile};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FluentDocument(Resource);

impl FluentDocument {
    pub fn resource(&self) -> &Resource {
        &self.0
    }
}

impl TryFrom<&FluentFile> for FluentDocument {
    type Error = LingoraError;

    fn try_from(value: &FluentFile) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(value.path())?;

        let resource = Parser::parse(content.as_str())?;

        Ok(Self(resource))
    }
}
