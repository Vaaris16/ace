use crate::commands::add::packages::packages::Packages;
use crate::utility::error::file_err::FileErrors;
use std::path::PathBuf;

pub fn get_instructions(name: &str) -> Result<Packages, FileErrors> {
    let content = match name {
        "tailwind" => include_str!("../packages/tailwind/tailwind_instructions.toml"),
        _ => {
            return Err(FileErrors::NotFound {
                path: PathBuf::from(name),
            });
        }
    };

    let package = toml::from_str(&content).map_err(|e| FileErrors::FailedParse {
        path: PathBuf::from(&name),
        source: e,
    })?;

    Ok(package)
}
