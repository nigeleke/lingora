use std::rc::Rc;

use regex::{Captures, Regex, bytes::CaptureMatches};

use crate::{
    error::LingoraError,
    fluent::path::{Path, PathSegment},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedIdentifier(Rc<Path>);

impl QualifiedIdentifier {
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn normalized(&self) -> QualifiedIdentifier {
        Self::from(&self.0.normalized())
    }

    pub fn to_normalized_string(&self) -> String {
        self.normalized().to_meta_string()
    }

    pub fn to_meta_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<&Path> for QualifiedIdentifier {
    fn from(value: &Path) -> Self {
        Self(Rc::new(value.clone()))
    }
}

impl std::str::FromStr for QualifiedIdentifier {
    type Err = LingoraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // (<term> | <message>) opt(<attribute>)
        let regex =
            Regex::new(r"^(?:(?P<term>-[\w-]+)|(?P<message>[\w-]+))(?P<attribute>\.[\w-]+)?$")
                .expect("required valid regex for identifier");

        let captures = regex
            .captures(s)
            .ok_or_else(|| LingoraError::MalformedIdentifierLiteral(s.into()))?;

        let mut segments = Vec::with_capacity(2);

        if let Some(m) = captures.name("message") {
            segments.push(PathSegment::Message(m.as_str().into()));
        } else if let Some(m) = captures.name("term") {
            segments.push(PathSegment::Term(m.as_str().into()));
        }

        if let Some(m) = captures.name("attribute") {
            segments.push(PathSegment::Attribute(m.as_str().into()));
        }

        let path = Path::from(segments.as_slice());
        Ok(Self::from(&path))
    }
}
