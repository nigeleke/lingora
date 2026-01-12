use std::{cell::OnceCell, path::Path};

use fluent4rs::{ast::Resource, prelude::Walker};

use crate::{
    domain::Locale,
    error::LingoraError,
    fluent::{
        Definitions, FluentDocument, FluentFile, QualifiedIdentifier, definitions::Signature,
    },
};

#[derive(Debug)]
pub struct QualfiedFluentFile {
    pub(crate) file: FluentFile,
    pub(crate) document: Result<FluentDocument, LingoraError>,
    definitions: OnceCell<Definitions>,
}

impl QualfiedFluentFile {
    pub fn locale(&self) -> &Locale {
        &self.file.locale
    }

    pub fn resource(&self) -> Result<&Resource, &LingoraError> {
        self.document.as_ref().map(|d| d.resource())
    }

    fn definitions(&self) -> &Definitions {
        self.definitions.get_or_init(|| {
            let mut collector = Definitions::default();
            if let Ok(resource) = self.resource() {
                Walker::walk(resource, &mut collector);
            }
            collector
        })
    }

    pub fn duplicate_identifiers(&self) -> Vec<QualifiedIdentifier> {
        self.definitions().duplicate_identifiers()
    }

    pub fn entry_identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().entry_identifiers()
    }

    pub fn signature(&self, identifier: &QualifiedIdentifier) -> Option<&Signature> {
        self.definitions().signature(identifier.path())
    }
}

impl PartialEq for QualfiedFluentFile {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Eq for QualfiedFluentFile {}

impl From<FluentFile> for QualfiedFluentFile {
    fn from(file: FluentFile) -> Self {
        let document = FluentDocument::try_from(&file);
        let identifiers = OnceCell::default();
        Self {
            file,
            document,
            definitions: identifiers,
        }
    }
}

impl TryFrom<&Path> for QualfiedFluentFile {
    type Error = LingoraError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let file = FluentFile::try_from(value)?;
        Ok(QualfiedFluentFile::from(file))
    }
}
