use clap::{Parser, Subcommand};

mod utility;

#[derive(Parser)]
#[command(name = "Phantom")]
#[command(about = "Ace cli")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { package: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Add { package } => match package.as_str() {},
    }
}
