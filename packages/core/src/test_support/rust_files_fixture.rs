use tempfile::TempDir;

use crate::rust::RustFile;

pub fn with_temp_rust_files<F>(file_spec: &[(&str, &str)], f: F)
where
    F: FnOnce(&[RustFile]),
{
    use std::io::Write;

    let dir = TempDir::new().expect("failed to create temp dir");

    let rust_files = file_spec
        .iter()
        .map(|(name, source)| {
            let path = dir.path().join(format!("{name}.rs"));
            let mut file =
                std::fs::File::create(&path).expect("failed to create temp rust source file");

            writeln!(file, "{}", source.trim()).expect("failed to write rust source content");

            RustFile::try_from(path.as_path()).expect("failed to parse fluent file")
        })
        .collect::<Vec<_>>();

    f(&rust_files);
    // dir dropped on return
}
