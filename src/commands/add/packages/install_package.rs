use std::path::Path;

use crate::{
    commands::add::packages::get_instructions::get_instructions, framework::framework::Frameworks, utility::{
        edit_file::edit_file, error::app_errors::AceErrors, mk_file::mk_file, run_cmd::run_cmd,
    }
};

pub fn install_package(name: &str, framework: Frameworks) -> Result<(), AceErrors> {
    let package = get_instructions(name, framework)?;

    for step in package.steps {
        match step.cmd.as_str() {
            "run_cmd" => run_cmd(&step.command.unwrap(), &step.arg.unwrap())?,
            "mk_file" => {
                println!("{:?}", step.content);
                mk_file(Path::new(&step.path.unwrap()), &step.content.unwrap())?
            }
            "edit_file" => edit_file(
                &step.needle.unwrap(),
                Path::new(&step.path.unwrap()),
                &step.insert.unwrap(),
            )?,
            _ => eprintln!("unknown cmd in instructions"),
        }
    }

    Ok(())
}
