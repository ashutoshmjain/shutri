use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

pub mod cli;
pub mod core;
pub mod massage;
pub mod config;
pub mod publish;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load_config()?;

    match &cli.command {
        Commands::Ingest { mk, source } => {
            println!("🚀 [Ingest] Processing Master Key: #{}", mk);
            println!("   Source asset: {:?}", source);
            
            let dest_path = core::migrate_asset(*mk, source)?;
            println!("   ✅ Asset migrated to: {:?}", dest_path);
        }
        Commands::Massage { mk, target } => {
            println!("🎨 [Massage] Formatting #{} for platform: {:?}", mk, target);
            
            let mk_dir = core::get_mk_dir(*mk)?;
            let dest_path = massage::massage_text(*mk, &mk_dir, target)?;
            println!("   ✅ Content massaged to: {:?}", dest_path);
        }
        Commands::Publish { mk, target } => {
            println!("📡 [Publish] Deploying #{} to destination: {:?}", mk, target);
            
            let mk_dir = core::get_mk_dir(*mk)?;
            publish::publish_assets(*mk, &mk_dir, target, &config)?;
        }
    }

    Ok(())
}
