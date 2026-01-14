use std::sync::Arc;

use crate::fluent::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedIdentifier(Arc<Path>);

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
        Self(Arc::new(value.clone()))
    }
}
