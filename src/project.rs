//! Defines the core data structures for managing project state.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents the entire state of a shutri project.
///
/// This struct holds all the information related to a single audio editing
/// project. It is the top-level data structure that gets serialized to
/// and deserialized from a project file.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    /// The name of the project, derived from the original audio file.
    /// e.g., "podcast_episode_1"
    pub name: String,
    /// The absolute path to the original imported audio file.
    pub original_audio_path: PathBuf,
    /// A list of paths to the audio chunks generated from the original file.
    pub chunks: Vec<PathBuf>,
    /// The structured content of the .shutri file, including all chunks and clips.
    pub script: Script,
}

/// Represents the content of a .shutri file, structured into chunks and clips.
///
/// This is the main container for the editable content that the user interacts with.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Script {
    /// A vector of chunks that make up the script. The order of chunks should
    /// match the chronological order of the audio.
    pub chunks: Vec<Chunk>,
}

/// Represents a chunk of audio, which is a segment of the original file.
///
/// Chunks are used to break down a long audio file into smaller, manageable
/// pieces for transcription and processing.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Chunk {
    /// The path to the audio file for this chunk.
    pub audio_path: PathBuf,
    /// The start time of the chunk relative to the beginning of the original
    /// audio file, in milliseconds.
    pub start_time_ms: u64,
    /// The end time of the chunk relative to the beginning of the original
    /// audio file, in milliseconds.
    pub end_time_ms: u64,
    /// A vector of clips that belong to this chunk.
    pub clips: Vec<Clip>,
}

/// Represents a single editable clip of audio with corresponding text.
///
/// This is the fundamental unit of editing in `shutri`. Each clip corresponds
/// to a line in the `.shutri` file that the user can manipulate.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clip {
    /// The start time of the clip relative to the beginning of the original
    /// audio file, in milliseconds.
    pub start_time_ms: u64,
    /// The end time of the clip relative to the beginning of the original
    /// audio file, in milliseconds.
    pub end_time_ms: u64,
    /// The transcribed text of the clip.
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Tests that a `Project` struct can be serialized to and deserialized from
    /// a JSON string, verifying that the data remains intact.
    #[test]
    fn test_project_serialization_deserialization() {
        let project = Project {
            name: "test_project".to_string(),
            original_audio_path: PathBuf::from("/path/to/audio.mp3"),
            chunks: vec![PathBuf::from("/path/to/chunk1.mp3")],
            script: Script {
                chunks: vec![Chunk {
                    audio_path: PathBuf::from("/path/to/chunk1.mp3"),
                    start_time_ms: 0,
                    end_time_ms: 30000,
                    clips: vec![Clip {
                        start_time_ms: 1000,
                        end_time_ms: 5000,
                        text: "Hello world".to_string(),
                    }],
                }],
            },
        };

        let serialized = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&serialized).unwrap();

        assert_eq!(project, deserialized);
    }
}