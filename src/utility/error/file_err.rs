use std::{fmt, io, path::PathBuf};

pub enum FileErrors {
    FileExists { path: PathBuf },

    NotFound { path: PathBuf },

    PermissionDenied { path: PathBuf },

    FailedRead { path: PathBuf, source: io::Error },

    FailedWrite { path: PathBuf, source: io::Error },
}

impl fmt::Display for FileErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileErrors::FileExists { path } => {
                write!(f, "file already exists at: {}", path.display())
            }
            FileErrors::NotFound { path } => write!(f, "file not found {}", path.display()),
            FileErrors::PermissionDenied { path } => {
                write!(f, "permission denied: {}", path.display())
            }
            FileErrors::FailedWrite { path, source } => {
                write!(f, "failed to write: {}:{}", path.display(), source)
            }
            FileErrors::FailedRead { path, source } => {
                write!(f, "failed to read: {}:{}", path.display(), source)
            }
        }
    }
}
