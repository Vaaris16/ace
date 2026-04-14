use std::{
    fmt::{self},
    io,
    path::PathBuf,
};

#[derive(Debug)]
pub enum FileErrors {
    FileExists {
        path: PathBuf,
    },

    NotFound {
        path: PathBuf,
    },

    PermissionDenied {
        path: PathBuf,
    },

    FailedRead {
        path: PathBuf,
        source: io::Error,
    },

    FailedWrite {
        path: PathBuf,
        source: io::Error,
    },

    FailedParse {
        path: PathBuf,
        source: toml::de::Error,
    },
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

            FileErrors::FailedParse { path, source } => {
                write!(f, "failed to parse: {}:{}", path.display(), source)
            }
        }
    }
}
