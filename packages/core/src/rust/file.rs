use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::error::LingoraError;

#[derive(Debug)]
pub struct RustFile(PathBuf);

impl TryFrom<&Path> for RustFile {
    type Error = LingoraError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let is_rust_ext = |path: &Path| path.extension() == Some(OsStr::new("rs"));
        let is_rust_file = |path: &Path| path.is_file() && is_rust_ext(path);

        if is_rust_file(path) {
            let path = path.to_path_buf();
            Ok(Self(path))
        } else {
            Err(LingoraError::InvalidRustPath(path.to_path_buf()))
        }
    }
}
