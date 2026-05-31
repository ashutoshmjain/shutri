//! Core logic for #SMS workspace and directory management

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Supported asset types in #SMS
#[derive(Debug, Clone, Copy)]
pub enum AssetType {
    Text,
    Image,
    Video,
    Unknown,
}

impl AssetType {
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()).as_deref() {
            Some("rs") | Some("md") | Some("txt") => AssetType::Text,
            Some("png") | Some("jpg") | Some("jpeg") | Some("webp") => AssetType::Image,
            Some("mp4") | Some("mkv") | Some("mov") => AssetType::Video,
            _ => AssetType::Unknown,
        }
    }

    pub fn default_name(&self) -> &'static str {
        match self {
            AssetType::Text => "text.rs",
            AssetType::Image => "image.png",
            AssetType::Video => "video.mp4",
            AssetType::Unknown => "unknown_asset",
        }
    }
}

/// Returns the base directory for #SMS assets (~/.sms)
pub fn get_base_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not identify home directory.")?;
    let base = home.join(".sms");
    if !base.exists() {
        fs::create_dir_all(&base).context("Failed to create ~/.sms base directory")?;
    }
    Ok(base)
}

/// Returns the MK-specific directory for a given Master Key (~/.sms/assets/{MK}/)
pub fn get_mk_dir(mk: u32) -> Result<PathBuf> {
    let base = get_base_dir()?;
    let mk_dir = base.join("assets").join(mk.to_string());
    if !mk_dir.exists() {
        fs::create_dir_all(&mk_dir).context(format!("Failed to create directory for MK #{}", mk))?;
    }
    Ok(mk_dir)
}

/// Migrates an asset from a source path to the MK-specific directory.
pub fn migrate_asset(mk: u32, source: &Path) -> Result<PathBuf> {
    if !source.exists() {
        return Err(anyhow!("Source file does not exist: {:?}", source));
    }

    let asset_type = AssetType::from_path(source);
    if let AssetType::Unknown = asset_type {
        return Err(anyhow!("Unsupported asset type for file: {:?}", source));
    }

    let mk_dir = get_mk_dir(mk)?;
    let dest_filename = asset_type.default_name();
    let dest_path = mk_dir.join(dest_filename);

    // If destination exists, we might want to back it up or rename it.
    // For now, we'll overwrite it to keep the "One Asset per MK" rule simple.
    fs::copy(source, &dest_path)
        .context(format!("Failed to copy asset from {:?} to {:?}", source, dest_path))?;
    
    Ok(dest_path)
}
