use super::args::CommandLineArgs;
use super::args::ResolvedArgsBuilder;
use super::reports::Analysis;

use thiserror::*;

use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("integrity errors detected")]
    IntegrityErrorsDetected,

    #[error("internal problem: {0}; raise issue")]
    InternalIssue(String),

    #[error("write failed: {0}")]
    WriteFailed(String),
}

type Result<T> = std::result::Result<T, AppError>;

pub struct App(Analysis);

impl App {
    pub fn try_new(args: &CommandLineArgs) -> Result<Self> {
        let builder = ResolvedArgsBuilder::default();

        let args = builder
            .build(args)
            .map_err(|e| AppError::InvalidArguments(e.to_string()))?
            .to_owned();

        let analysis =
            Analysis::try_from(&args).map_err(|e| AppError::AnalysisFailed(e.to_string()))?;

        Ok(Self(analysis))
    }

    pub fn output_dioxus_i18n(&self, _config_rs_file: &Path) -> Result<()> {
        unimplemented!()
    }

    pub fn output_analysis(&self, stdout: Rc<RefCell<dyn Write>>) -> Result<()> {
        let reference_path = self.0.reference_path();
        self.output_check("Reference:", reference_path, &stdout)?;

        self.0
            .target_paths_by_locale()
            .iter()
            .try_for_each(|p| self.output_check("Target:", p, &stdout))
    }

    pub fn output_check(
        &self,
        title: &str,
        path: &PathBuf,
        stdout: &Rc<RefCell<dyn Write>>,
    ) -> Result<()> {
        let path_string = path.to_string_lossy();
        let check = self
            .0
            .check(path)
            .ok_or_else(|| AppError::InternalIssue(format!("Cannot check {}", path_string)))?;
        let mut check = Vec::from(check);
        check.sort();

        let mut stdout = (*stdout).borrow_mut();

        writeln!(
            stdout,
            "{} {}{}",
            title,
            path.to_string_lossy(),
            if check.is_empty() { " - Ok" } else { "" }
        )
        .map_err(|e| AppError::WriteFailed(e.to_string()))?;

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
            .map_err(|e| AppError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    pub fn show_gui(&self) {
        unimplemented!()
    }

    pub fn exit_status(&self) -> Result<()> {
        if self.0.is_ok() {
            Ok(())
        } else {
            Err(AppError::IntegrityErrorsDetected)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    fn do_output_analysis(args: &str) -> String {
        let stdout_buffer = Vec::new();
        let stdout = Rc::new(RefCell::new(std::io::BufWriter::new(stdout_buffer)));

        let args = CommandLineArgs::from_str(args).unwrap();
        let app = App::try_new(&args).unwrap();

        let _ = app.output_analysis(stdout.clone()).unwrap();

        let stdout = stdout.borrow();
        let bytes = stdout.buffer();
        String::from_utf8_lossy(&bytes).to_string()
    }

    #[test]
    fn app_will_output_checks_when_no_errors() {
        let result = do_output_analysis(
            "app_name -r tests/data/cross_check/reference_matching.ftl -t tests/data/cross_check/target_matching.ftl");
        insta::assert_snapshot!(result, @r"
        Reference: tests/data/cross_check/reference_matching.ftl - Ok
        Target: tests/data/cross_check/target_matching.ftl - Ok
        ");
    }

    #[test]
    fn app_will_output_checks_when_errors() {
        let result = do_output_analysis("app_name -r tests/data/cross_check/reference_missing.ftl -t tests/data/cross_check/target_superfluous.ftl");
        insta::assert_snapshot!(result, @r"
        Reference: tests/data/cross_check/reference_missing.ftl - Ok
        Target: tests/data/cross_check/target_superfluous.ftl
            Missing translation: -missing-term
                                 missing-message
            Superfluous translation: -superfluous-term
                                     superfluous-message
        ");
    }
}
