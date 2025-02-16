use super::error::{Result, WriterError};
use super::writer::Writer;

use crate::domain::Analysis;

use std::path::PathBuf;

pub struct AnalysisWriter<'a> {
    analysis: &'a Analysis,
    writer: Writer,
}

impl<'a> AnalysisWriter<'a> {
    pub fn new(analysis: &'a Analysis, writer: Writer) -> Self {
        Self { analysis, writer }
    }

    pub fn write(&self) -> Result<()> {
        let reference_path = self.analysis.reference_path();
        self.output_check("Reference:", reference_path)?;

        let mut target_paths = self.analysis.target_paths();

        target_paths.sort();
        target_paths
            .iter()
            .try_for_each(|p| self.output_check("Target:", p))
    }

    pub fn output_check(&self, title: &str, path: &PathBuf) -> Result<()> {
        let path_string = path.to_string_lossy();
        let check = self
            .analysis
            .check(path)
            .ok_or_else(|| WriterError::InternalIssue(format!("Cannot check {}", path_string)))?;
        let mut check = Vec::from(check);
        check.sort();

        let mut stdout = (*self.writer).borrow_mut();

        writeln!(
            stdout,
            "{} {}{}",
            title,
            path.to_string_lossy(),
            if check.is_empty() { " - Ok" } else { "" }
        )
        .map_err(|e| WriterError::WriteFailed(e.to_string()))?;

        let mut current_category = "";
        check
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
