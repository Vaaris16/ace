use crate::utility::error::{cmd_err::CmdErr, file_err::FileErrors};

#[derive(Debug)]
pub enum AceErrors {
    File(FileErrors),
    Cmd(CmdErr),
}

impl From<FileErrors> for AceErrors {
    fn from(value: FileErrors) -> Self {
        AceErrors::File(value)
    }
}

impl From<CmdErr> for AceErrors {
    fn from(value: CmdErr) -> Self {
        AceErrors::Cmd(value)
    }
}
