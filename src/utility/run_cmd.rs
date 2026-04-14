use crate::utility::error::cmd_err::CmdErr;
use std::process::Command;

pub fn run_cmd(command: &str, arg: &str) -> Result<(), CmdErr> {
    let args: Vec<&str> = arg.split_whitespace().collect();
    let status = Command::new(command)
        .args(args)
        .env("PATH", std::env::var("PATH").unwrap())
        .status()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                CmdErr::NotFound {
                    cmd: (command.into()),
                }
            } else {
                CmdErr::SpawnError {
                    cmd: command.into(),
                    source: e,
                }
            }
        })?;

    println!("{}{}", command, arg);

    if !status.success() {
        return Err(CmdErr::Failed {
            cmd: format!("{}{}", command, arg),
            code: status.code(),
        });
    }
    Ok(())
}
