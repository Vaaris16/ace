use clap::{Parser, Subcommand};

use crate::{
    commands::add::packages::install_package::install_package, framework::framework::Frameworks,
    utility::error::app_errors::FizzErrors,
};

mod commands;
mod framework;
mod utility;

#[derive(Parser)]
#[command(name = "Fizz")]
#[command(about = "Fizz cli")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        package: String,
        framework: Frameworks,
    },
}

fn main() -> Result<(), FizzErrors> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Add { package, framework } => match package.as_str() {
            "tailwindcss" => install_package("tailwind", framework)?,
            _ => eprintln!("package does not exist"),
        },
    }

    Ok(())
}
