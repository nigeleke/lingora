use std::{cell::OnceCell, path::Path, sync::Arc};

use fluent4rs::{ast::Resource, prelude::Walker};

use crate::{
    domain::Locale,
    error::LingoraError,
    fluent::{
        Definitions, FluentDocument, FluentFile, QualifiedIdentifier, definitions::Signature,
    },
};

#[derive(Debug)]
struct Analysis {
    document: Result<FluentDocument, LingoraError>,
    definitions: OnceCell<Definitions>,
}

#[derive(Clone, Debug)]
pub struct QualifiedFluentFile {
    file: Arc<FluentFile>,
    analysis: Arc<Analysis>,
}

impl QualifiedFluentFile {
    pub fn locale(&self) -> &Locale {
        &self.file.locale
    }

    pub fn is_well_formed(&self) -> bool {
        self.analysis.document.is_ok()
    }

    pub fn resource(&self) -> Result<&Resource, &LingoraError> {
        self.analysis.document.as_ref().map(|d| d.resource())
    }

    fn definitions(&self) -> &Definitions {
        self.analysis.definitions.get_or_init(|| {
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

    pub fn identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().identifiers()
    }

    pub fn references(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.definitions().references()
    }
}

impl PartialEq for QualifiedFluentFile {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Eq for QualifiedFluentFile {}

impl From<FluentFile> for QualifiedFluentFile {
    fn from(file: FluentFile) -> Self {
        let document = FluentDocument::try_from(&file);
        let definitions = OnceCell::default();
        let analysis = Analysis {
            document,
            definitions,
        };

        Self {
            file: Arc::new(file),
            analysis: Arc::new(analysis),
        }
    }
}

impl TryFrom<&Path> for QualifiedFluentFile {
    type Error = LingoraError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let file = FluentFile::try_from(value)?;
        Ok(QualifiedFluentFile::from(file))
    }
}
