use anyhow::Result;
use clap::Parser;
use cli::Commands;

pub mod audio;
pub mod cli;
pub mod project;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        Commands::Import { file_path } => {
            println!("Importing file: {:?}", file_path);
            // This is where we will call the audio import function.
            audio::import(file_path)?;
        }
    }

    Ok(())
}