use std::{fs, path::Path};

use crate::utility::error::file_err::FileErrors;

pub fn edit_file(needle: &str, path: &Path, insert: &str) -> Result<(), FileErrors> {
    let content = fs::read_to_string(&path).map_err(|e| FileErrors::FailedRead {
        path: path.to_path_buf(),
        source: e,
    })?;

    let new_content = if let Some(pos) = content.find(needle) {
        let index = pos + needle.len();
        format!("{}{}{}", &content[..index], &insert, &content[index..],)
    } else {
        content
    };

    fs::write(&path, new_content).map_err(|e| FileErrors::FailedWrite {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(())
}
