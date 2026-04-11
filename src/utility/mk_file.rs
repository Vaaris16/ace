use std::{
    fs::OpenOptions,
    io::{ErrorKind, Write},
    path::Path,
};

use crate::utility::error::file_err::FileErrors;

pub fn mk_file(path: &Path, contents: &str) -> Result<(), FileErrors> {
    if path.exists() {
        return Err(FileErrors::FileExists {
            path: path.to_path_buf(),
        });
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => FileErrors::NotFound {
                path: path.to_path_buf(),
            },
            ErrorKind::PermissionDenied => FileErrors::PermissionDenied {
                path: path.to_path_buf(),
            },
            _ => FileErrors::FailedWrite {
                path: path.to_path_buf(),
                source: e,
            },
        })?;

    file.write_all(contents.as_bytes())
        .map_err(|e| FileErrors::FailedWrite {
            path: path.to_path_buf(),
            source: e,
        })?;

    Ok(())
}
