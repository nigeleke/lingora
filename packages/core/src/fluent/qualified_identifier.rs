use std::rc::Rc;

use regex::Regex;

use crate::{
    error::LingoraError,
    fluent::path::{Path, PathSegment},
};

/// A fully qualified Fluent identifier, identifying a message, term,
/// attribute or variable within a Fluent resource.
///
/// Examples of valid qualified identifiers:
/// - `hello`                  → message `hello`
/// - `user-name`              → message `user-name`
/// - `-brand`                 → term `-brand`
/// - `greeting.title`         → attribute `.title` on message `greeting`
/// - `-error.auth.invalid`    → attribute `.invalid` on term `-error.auth`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedIdentifier(Rc<Path>);

impl QualifiedIdentifier {
    /// Returns a reference to the underlying `Path` (sequence of segments).
    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Returns a new `QualifiedIdentifier` with the path normalized.
    ///
    /// In most cases `self.normalized()` is equivalent to `self`, but this method
    /// ensures consistency when comparing or serializing, i.e, it does not differentiate
    /// between default and non-default variants.
    pub fn normalized(&self) -> QualifiedIdentifier {
        Self::from(&self.0.normalized())
    }

    /// Returns the normalized string representation.
    pub fn to_normalized_string(&self) -> String {
        self.normalized().to_meta_string()
    }

    /// Returns the string representation of the identifier.
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
