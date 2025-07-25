//! This module defines the command-line interface for the application.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A text-based audio editor for precise, keyboard-driven workflows.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Import an audio file and prepare it for editing.
    Import {
        /// The path to the MP3 audio file to import.
        #[arg(required = true)]
        file_path: PathBuf,
    },
    /// Transcribe an audio project.
    Transcribe {
        /// The name of the project to transcribe.
        #[arg(required = true)]
        project_name: String,

        /// Generate mock transcription data instead of calling an API.
        #[arg(long)]
        mock: bool,

        /// Force overwrite of an existing project file.
        #[arg(long)]
        force: bool,
    },
}