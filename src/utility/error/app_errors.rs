use crate::utility::error::{cmd_err::CmdErr, file_err::FileErrors};

#[derive(Debug)]
pub enum DashErrors {
    File(FileErrors),
    Cmd(CmdErr),
}

impl From<FileErrors> for DashErrors {
    fn from(value: FileErrors) -> Self {
        DashErrors::File(value)
    }
}

impl From<CmdErr> for DashErrors {
    fn from(value: CmdErr) -> Self {
        DashErrors::Cmd(value)
    }
}
