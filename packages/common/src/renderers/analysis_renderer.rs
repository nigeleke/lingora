use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{Analysis, LingoraError};

pub struct AnalysisRenderer {
    reference: PathBuf,
    analysis: Analysis,
}

impl AnalysisRenderer {
    pub fn new(reference: &Path, analysis: Analysis) -> Self {
        Self {
            reference: reference.to_path_buf(),
            analysis,
        }
    }

    pub fn render<W: io::Write>(&self, out: &mut W) -> Result<(), LingoraError> {
        let reference_path = self.reference.as_path();

        self.output_check(out, "Reference:", reference_path)?;

        let mut paths = self
            .analysis
            .paths()
            .into_iter()
            .filter(|p| p != &reference_path)
            .collect::<Vec<_>>();
        paths.sort();

        paths
            .iter()
            .try_for_each(|f| self.output_check(out, "Target:", f))
    }

    fn output_check<W: io::Write>(
        &self,
        out: &mut W,
        title: &str,
        path: &Path,
    ) -> Result<(), LingoraError> {
        let path_string = path.to_string_lossy();
        let mut checks = self.analysis.checks(path).clone();
        checks.sort();

        writeln!(
            out,
            "{} {}{}",
            title,
            path_string,
            if checks.is_empty() { " - Ok" } else { "" }
        )?;

        let mut current_category = "";
        checks.iter().try_for_each(|c| {
            if current_category != c.category_str() {
                current_category = c.category_str();
                writeln!(out, "    {}", c)
            } else {
                writeln!(
                    out,
                    "    {}  {}",
                    " ".repeat(c.category_str().len()),
                    c.value_str()
                )
            }
        })?;

        Ok(())
    }
}
