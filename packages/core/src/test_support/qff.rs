use tempfile::TempDir;

use crate::fluent::QualfiedFluentFile;

pub fn qff(locale: &str, ftl: &str) -> QualfiedFluentFile {
    use std::io::Write;

    let dir = TempDir::new().expect("failed to create temp dir");
    let path = dir.path().join(format!("{locale}.ftl"));
    let mut file = std::fs::File::create(path.clone()).expect("failed to create temp file");

    let _ = writeln!(file, "{}", ftl.trim());
    QualfiedFluentFile::try_from(path.as_path()).expect("")
    // Note: file n/a on return - only AST
}
