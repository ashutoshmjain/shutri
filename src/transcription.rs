//! This module handles the transcription of audio projects.

use crate::project::{Chunk, Clip};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Generates a mock `.shutri` project file with dummy transcription data.
///
/// This function is used for testing the editing workflow without making actual
/// API calls. It creates plausible timestamps and placeholder text.
pub fn generate_mock(project_name: &str) -> Result<()> {
    let project_dir = get_project_dir(project_name)?;
    let splits_dir = project_dir.join("splits");

    // 1. Read the splits directory and create clips
    let mut clips = create_clips_from_splits(&splits_dir)?;

    // 2. Group clips into chunks
    let chunks = chunk_clips(&mut clips);

    // 3. Format the output
    let output = format_shutri_file(project_name, &chunks);

    // 4. Write the .shutri file
    let shutri_path = project_dir.join(format!("{}.shutri", project_name));
    let mut file = fs::File::create(&shutri_path)
        .with_context(|| format!("Could not create shutri file at {:?}", shutri_path))?;
    file.write_all(output.as_bytes())?;

    println!("Successfully generated mock transcription at {:?}", shutri_path);

    Ok(())
}

/// Creates a vector of `Clip`s from a directory of split audio files.
fn create_clips_from_splits(splits_dir: &Path) -> Result<Vec<Clip>> {
    let mut paths: Vec<_> = fs::read_dir(splits_dir)
        .context("Could not read splits directory")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "mp3"))
        .collect();
    paths.sort();

    let mut clips = Vec::new();
    let mut current_time_ms = 0;

    for (i, path) in paths.iter().enumerate() {
        let duration_str = String::from_utf8(
            Command::new("soxi")
                .arg("-D")
                .arg(path)
                .output()
                .context("Failed to execute soxi command.")?
                .stdout,
        )?;
        let duration_secs: f64 = duration_str.trim().parse()?;
        let duration_ms = (duration_secs * 1000.0) as u64;

        let end_time_ms = current_time_ms + duration_ms;

        clips.push(Clip {
            start_time_ms: current_time_ms,
            end_time_ms,
            text: format!("This is a mock transcription for split #{}.", i + 1),
        });

        current_time_ms = end_time_ms;
    }

    Ok(clips)
}

/// Groups a list of `Clip`s into `Chunk`s based on the 60-second rule.
fn chunk_clips(clips: &mut Vec<Clip>) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    if clips.is_empty() {
        return chunks;
    }

    let mut current_chunk_clips: Vec<Clip> = Vec::new();
    let mut current_chunk_duration = 0;

    for clip in clips.drain(..) {
        let clip_duration = clip.end_time_ms - clip.start_time_ms;

        // If the current chunk is not empty and adding the next clip would exceed
        // the target duration, finalize the current chunk.
        if !current_chunk_clips.is_empty() && current_chunk_duration + clip_duration > 60000 {
            // It's guaranteed that current_chunk_clips is not empty here.
            let first_clip = current_chunk_clips.first().unwrap();
            let last_clip = current_chunk_clips.last().unwrap();
            
            chunks.push(Chunk {
                start_time_ms: first_clip.start_time_ms,
                end_time_ms: last_clip.end_time_ms,
                clips: std::mem::take(&mut current_chunk_clips),
            });
            
            current_chunk_duration = 0;
        }

        // Add the current clip to the new or existing chunk.
        current_chunk_duration += clip_duration;
        current_chunk_clips.push(clip);
    }

    // After the loop, add the last remaining chunk if it's not empty.
    if !current_chunk_clips.is_empty() {
        let first_clip = current_chunk_clips.first().unwrap();
        let last_clip = current_chunk_clips.last().unwrap();

        chunks.push(Chunk {
            start_time_ms: first_clip.start_time_ms,
            end_time_ms: last_clip.end_time_ms,
            clips: current_chunk_clips,
        });
    }

    chunks
}

/// Formats the chunks and clips into the final `.shutri` file content.
fn format_shutri_file(project_name: &str, chunks: &[Chunk]) -> String {
    let mut output = String::new();
    output.push_str(&format!("\" Project: {}.mp3\n", project_name));
    output.push_str("\"\n");
    output.push_str("\" Keybindings:\n");
    output.push_str("\"   <Leader>p : Play current clip (preview your edit)\n");
    output.push_str("\"   <Leader>c : Play original chunk (hear the 'before')\n");
    output.push_str("\"   <Leader>C : Play edited chunk (hear the 'after')\n");
    output.push_str("\"   <Leader>s : Stop all playback\n");
    output.push_str("\"   <Leader>[ : Nudge start time of current clip earlier\n");
    output.push_str("\"   <Leader>] : Nudge start time of current clip later\n");
    output.push_str("\"   <Leader>{ : Nudge end time of current clip earlier\n");
    output.push_str("\"   <Leader>} : Nudge end time of current clip later\n");
    output.push_str("\" =============================================================================\n\n");

    for (i, chunk) in chunks.iter().enumerate() {
        output.push_str(&format!(
            "// --- CHUNK {} ({} - {}) ---\n",

            i + 1,
            format_timestamp(chunk.start_time_ms),
            format_timestamp(chunk.end_time_ms)
        ));
        for clip in &chunk.clips {
            output.push_str(&format!(
                "[{}] {} [{}]\n",
                format_timestamp(clip.start_time_ms),
                clip.text,
                format_timestamp(clip.end_time_ms)
            ));
        }
        output.push('\n');
    }

    output
}

/// Formats milliseconds into a `MM:SS.ms` string.
fn format_timestamp(ms: u64) -> String {
    let secs = ms / 1000;
    let millis = ms % 1000;
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
}

/// Returns the path to a project's directory.
fn get_project_dir(project_name: &str) -> Result<std::path::PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory.")?;
    Ok(home.join(".shutri").join("projects").join(project_name))
}