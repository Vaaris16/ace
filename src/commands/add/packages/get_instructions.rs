use crate::commands::add::packages::packages_instruction::Packages;
use crate::commands::add::packages::tailwind::tailwindcss::tailwindcss;
use crate::utility::error::file_err::FileErrors;
use std::path::PathBuf;
use crate::framework::framework::Frameworks;

pub fn get_instructions(name: &str, framework: Frameworks) -> Result<Packages, FileErrors> {
    let content = match name {
        "tailwind" => tailwindcss(framework),
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
