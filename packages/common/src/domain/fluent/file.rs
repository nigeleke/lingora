use std::{collections::HashSet, fs, path::Path};

use fluent4rs::{ast::*, prelude::*};

use crate::{
    LingoraError,
    domain::{fluent::identifier_visitor::IdentifierVisitor, identifier::Identifier},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File(Resource);

impl File {
    pub fn resource(&self) -> &Resource {
        &self.0
    }

    pub fn identifiers(&self) -> HashSet<Identifier> {
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&self.0, &mut visitor);
        visitor.identifiers()
    }

    pub fn identifier_usage(&self, identifier: &Identifier) -> HashSet<Entry> {
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&self.0, &mut visitor);
        visitor.usages(identifier)
    }
}

impl TryFrom<&Path> for File {
    type Error = LingoraError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(value)?;

        let resource = Parser::parse(content.as_str())?;

        Ok(Self(resource))
    }
}
