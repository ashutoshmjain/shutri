use anyhow::Result;
use clap::Parser;
use cli::Commands;

pub mod audio;
pub mod cli;
pub mod project;
pub mod transcription;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match &cli.command {
        Commands::Import { file_path } => {
            println!("Importing file: {:?}", file_path);
            audio::import(file_path)?;
        }
        Commands::Transcribe { project_name, mock } => {
            #[cfg(debug_assertions)]
            if *mock {
                transcription::generate_mock(project_name)?;
            } else {
                println!("Transcribing project: {}", project_name);
                // To be implemented in a future milestone
            }
            #[cfg(not(debug_assertions))]
            {
                println!("Transcribing project: {}", project_name);
                // To be implemented in a future milestone
            }
        }
    }

    Ok(())
}