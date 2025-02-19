use std::path::PathBuf;

use super::{
    error::{Result, WriterError},
    writer::Writer,
};
use crate::domain::Analysis;

pub struct AnalysisWriter<'a> {
    reference: &'a PathBuf,
    analysis: &'a Analysis,
    writer: Writer,
}

impl<'a> AnalysisWriter<'a> {
    pub fn new(reference: &'a PathBuf, analysis: &'a Analysis, writer: Writer) -> Self {
        Self {
            reference,
            analysis,
            writer,
        }
    }

    pub fn write(&self) -> Result<()> {
        self.output_check("Reference:", self.reference)?;

        let mut paths = self
            .analysis
            .paths()
            .into_iter()
            .filter(|p| p != &self.reference)
            .collect::<Vec<_>>();
        paths.sort();

        paths
            .iter()
            .try_for_each(|f| self.output_check("Target:", f))
    }

    pub fn output_check(&self, title: &str, path: &PathBuf) -> Result<()> {
        let path_string = path.to_string_lossy();
        let mut checks = self.analysis.checks(path).clone();
        checks.sort();

        let mut stdout = (*self.writer).borrow_mut();

        writeln!(
            stdout,
            "{} {}{}",
            title,
            path_string,
            if checks.is_empty() { " - Ok" } else { "" }
        )
        .map_err(|e| WriterError::WriteFailed(e.to_string()))?;

        let mut current_category = "";
        checks
            .iter()
            .try_for_each(|c| {
                if current_category != c.category_str() {
                    current_category = c.category_str();
                    writeln!(stdout, "    {}", c)
                } else {
                    writeln!(
                        stdout,
                        "    {}  {}",
                        " ".repeat(c.category_str().len()),
                        c.value_str()
                    )
                }
            })
            .map_err(|e| WriterError::WriteFailed(e.to_string()))?;

        Ok(())
    }
}
