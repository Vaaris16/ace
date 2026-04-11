use std::fmt;

pub enum CmdErr {
    NotFound { cmd: String },

    Failed { cmd: String, code: Option<i32> },

    SpawnError { cmd: String, source: std::io::Error },

    InvalidOutput { cmd: String, output: String },
}

impl fmt::Display for CmdErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdErr::NotFound { cmd } => write!(f, "`{}` not found in PATH", cmd),
            CmdErr::Failed { cmd, code } => write!(f, "`{}` failed with code: {:?}", cmd, code),
            CmdErr::SpawnError { cmd, source } => {
                write!(f, "failed to spawn `{}`:{}", cmd, source)
            }
            CmdErr::InvalidOutput { cmd, output } => {
                write!(f, "`{}` returned invalid output: {}", cmd, output)
            }
        }
    }
}
