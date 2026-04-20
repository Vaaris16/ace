use crate::utility::error::{cmd_err::CmdErr, file_err::FileErrors};

#[derive(Debug)]
pub enum FizzErrors {
    File(FileErrors),
    Cmd(CmdErr),
}

impl From<FileErrors> for FizzErrors {
    fn from(value: FileErrors) -> Self {
        FizzErrors::File(value)
    }
}

impl From<CmdErr> for FizzErrors {
    fn from(value: CmdErr) -> Self {
        FizzErrors::Cmd(value)
    }
}
