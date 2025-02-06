use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ResolvedArgs {
    reference: PathBuf,
    targets: Vec<PathBuf>,
}

impl ResolvedArgs {
    pub fn new(reference: PathBuf, targets: Vec<PathBuf>) -> Self {
        Self { reference, targets }
    }

    pub fn reference(&self) -> &PathBuf {
        &self.reference
    }

    pub fn targets(&self) -> &[PathBuf] {
        self.targets.as_slice()
    }
}
