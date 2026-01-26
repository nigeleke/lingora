use tempfile::TempDir;

use crate::fluent::FluentFile;

pub fn with_temp_fluent_files<F>(file_spec: &[(&str, &str)], f: F)
where
    F: FnOnce(&[FluentFile]),
{
    use std::io::Write;

    let dir = TempDir::new().expect("failed to create temp dir");

    let fluent_files = file_spec
        .iter()
        .map(|(locale, ftl)| {
            let path = dir.path().join(format!("{locale}.ftl"));
            let mut file = std::fs::File::create(&path).expect("failed to create temp fluent file");

            writeln!(file, "{}", ftl.trim()).expect("failed to write ftl content");

            FluentFile::try_from(path.as_path()).expect("failed to parse fluent file")
        })
        .collect::<Vec<_>>();

    f(&fluent_files);
    // dir dropped on return
}
