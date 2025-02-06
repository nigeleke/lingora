use super::fluent_file::FluentFile;
use super::integrity_check::IntegrityCheck;
use super::integrity_cross_check::IntegrityCrossCheck;

use thiserror::*;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ComparisonError {
    #[error("cannot read fluent file - reason: {0}")]
    CannotReadFluentFile(String),
}

type Result<T> = std::result::Result<T, ComparisonError>;

#[derive(Clone, Debug)]
pub struct Comparison {
    reference: PathBuf,
    target: PathBuf,
    reference_check: IntegrityCheck,
    target_check: IntegrityCheck,
    cross_check: IntegrityCrossCheck,
}

impl Comparison {
    pub fn try_new(reference: PathBuf, target: PathBuf) -> Result<Self> {
        let reference_file = FluentFile::try_from(&reference)
            .map_err(|e| ComparisonError::CannotReadFluentFile(e.to_string()))?;
        let reference_check = IntegrityCheck::from(reference_file.resource());

        let target_file = FluentFile::try_from(&target)
            .map_err(|e| ComparisonError::CannotReadFluentFile(e.to_string()))?;
        let target_check = IntegrityCheck::from(target_file.resource());

        let cross_check =
            IntegrityCrossCheck::new(reference_file.resource(), target_file.resource());

        Ok(Self {
            reference,
            target,
            reference_check,
            target_check,
            cross_check,
        })
    }
}
