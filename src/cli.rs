//! CLI definition for #SMS (Shutri Media Solution)

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// #SMS: A professional media workflow engine for high-fidelity research publishing.
#[derive(Parser, Debug)]
#[command(name = "sms", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Ingest raw assets (Text, Images, Video) into the MK-indexed directory.
    Ingest {
        /// The Master Key (Episode/Chapter number)
        #[arg(long)]
        mk: u32,

        /// The path to the source file
        #[arg(short, long)]
        source: PathBuf,
    },

    /// Transform raw assets into platform-specific formats.
    Massage {
        /// The Master Key (Episode/Chapter number)
        #[arg(long)]
        mk: u32,

        /// The target platform format
        #[arg(short, long)]
        target: TargetPlatform,
    },

    /// Publish the formatted content to the destination repository.
    Publish {
        /// The Master Key (Episode/Chapter number)
        #[arg(long)]
        mk: u32,

        /// The target platform format to publish
        #[arg(short, long)]
        target: TargetPlatform,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TargetPlatform {
    /// mdBook format for research publishing
    Mdbook,
    /// LinkedIn post format (summarized/pruned)
    Linkedin,
    /// Nostr long-form content (NIP-23)
    Nostr,
}
