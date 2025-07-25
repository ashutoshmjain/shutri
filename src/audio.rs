//! This module handles all audio processing tasks.

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MIN_SPLIT_DURATION_SECS: f64 = 6.0;

/// Represents a single audio split.
#[derive(Debug, Clone)]
struct Split {
    path: PathBuf,
    duration: f64,
}

/// Manages the collection of splits for a project.
#[derive(Debug)]
struct SplitManifest {
    splits: Vec<Split>,
    project_dir: PathBuf,
}

impl SplitManifest {
    /// Creates a new manifest from a directory of split files.
    fn from_directory(project_dir: &Path) -> Result<Self> {
        let splits_dir = project_dir.join("splits");
        let mut paths: Vec<PathBuf> = fs::read_dir(&splits_dir)
            .context("Could not read splits directory")?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == "mp3"))
            .collect();

        paths.sort();

        let mut splits = Vec::new();
        for path in paths {
            let duration_str = String::from_utf8(
                Command::new("soxi")
                    .arg("-D")
                    .arg(&path)
                    .output()
                    .context("Failed to execute soxi command.")?
                    .stdout,
            )?;
            let duration = duration_str.trim().parse::<f64>()?;
            splits.push(Split { path, duration });
        }

        Ok(SplitManifest {
            splits,
            project_dir: project_dir.to_path_buf(),
        })
    }

    /// Merges short splits together until all are above the minimum duration.
    fn merge_short_splits(&mut self) -> Result<()> {
        loop {
            // If there's only one split (or none), we can't merge.
            if self.splits.len() <= 1 {
                break;
            }

            let Some(index) = self
                .splits
                .iter()
                .position(|s| s.duration < MIN_SPLIT_DURATION_SECS) else {
                break; // No more short splits found
            };

            // Determine which split to merge with based on the rules in the spec
            let (victim_idx, target_idx) = if index == 0 {
                // First split: merge with next
                (index, index + 1)
            } else if index == self.splits.len() - 1 {
                // Last split: merge with previous
                (index, index - 1)
            } else {
                // Middle split: merge with shorter neighbor
                let prev_duration = self.splits[index - 1].duration;
                let next_duration = self.splits[index + 1].duration;
                if prev_duration <= next_duration {
                    (index, index - 1)
                } else {
                    (index, index + 1)
                }
            };
            
            let victim = self.splits[victim_idx].clone();
            let target = self.splits[target_idx].clone();

            // Use a temporary file for the merge to avoid corruption
            let temp_output = self.project_dir.join("temp_merge.mp3");

            let merge_cmd = Command::new("sox")
                .arg(&target.path)
                .arg(&victim.path)
                .arg(&temp_output)
                .output()
                .context("Failed to execute SoX merge command.")?;

            if !merge_cmd.status.success() {
                return Err(anyhow!("SoX merge command failed"));
            }

            // Replace the target file with the merged file
            fs::rename(&temp_output, &target.path)?;
            // Remove the victim file
            fs::remove_file(&victim.path)?;

            println!("Merged {:?} into {:?}", victim.path.file_name().unwrap(), target.path.file_name().unwrap());

            // Reload the manifest from the directory to reflect changes
            *self = Self::from_directory(&self.project_dir)?;
        }
        Ok(())
    }
}

/// Returns the path to the main `~/.shutri` directory.
fn get_shutri_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory.")?;
    Ok(home.join(".shutri"))
}

/// Imports an audio file, creates a project structure, splits the audio by silence,
/// and then merges short splits together to create intelligently-sized chunks.
///
/// This function is the entry point for the `shutri --import` command. It performs
/// the following steps:
/// 1.  Validates the input file path.
/// 2.  Creates a project directory structure within `~/.shutri`.
/// 3.  Copies the original audio file to the `imports` directory.
/// 4.  Uses the `sox` command-line tool to split the audio into multiple MP3 files
///     based on detected silences.
/// 5.  Creates a `SplitManifest` to analyze the generated splits.
/// 6.  Calls `merge_short_splits` to consolidate splits smaller than the defined
///     minimum duration, ensuring the final splits are meaningful.
pub fn import(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Err(anyhow!("Input file does not exist."));
    }

    let project_name = file_path
        .file_stem()
        .context("Could not determine file stem.")?
        .to_string_lossy();

    println!("Creating project: {}", project_name);

    let shutri_dir = get_shutri_dir()?;
    let projects_dir = shutri_dir.join("projects");
    let project_dir = projects_dir.join(project_name.as_ref());
    let splits_dir = project_dir.join("splits");
    fs::create_dir_all(&splits_dir)
        .with_context(|| format!("Could not create project directory at {:?}", splits_dir))?;

    let imports_dir = shutri_dir.join("imports");
    fs::create_dir_all(&imports_dir)
        .with_context(|| format!("Could not create imports directory at {:?}", imports_dir))?;

    let import_path = imports_dir.join(file_path.file_name().unwrap());
    fs::copy(file_path, &import_path)
        .with_context(|| format!("Failed to copy file to {:?}", import_path))?;
    println!("Copied source file to {:?}", import_path);

    let sox_cmd = Command::new("sox")
        .arg(&import_path)
        .arg(splits_dir.join("split.mp3")) // Base name for sox
        .arg("silence")
        .arg("1")
        .arg("0.1")
        .arg("1%")
        .arg("1")
        .arg("0.6")
        .arg("1%")
        .arg(":")
        .arg("newfile")
        .arg(":")
        .arg("restart")
        .output()
        .context("Failed to execute SoX command.")?;

    if !sox_cmd.status.success() {
        let stderr = String::from_utf8_lossy(&sox_cmd.stderr);
        return Err(anyhow!("SoX command failed:\n{}", stderr));
    }

    // Rename the files created by SoX to the correct format
    let mut split_files: Vec<PathBuf> = fs::read_dir(&splits_dir)
        .context("Could not read splits directory for renaming")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "mp3"))
        .collect();
    
    split_files.sort();

    for (i, old_path) in split_files.iter().enumerate() {
        let new_name = format!("split-{:03}.mp3", i + 1);
        let new_path = splits_dir.join(new_name);
        fs::rename(old_path, new_path)?;
    }

    println!("Initial split successful. Now merging short splits...");

    let mut manifest = SplitManifest::from_directory(&project_dir)?;
    manifest.merge_short_splits()?;

    println!("Project '{}' created and processed successfully.", project_name);

    Ok(())
}