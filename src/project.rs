//! Defines the core data structures for managing project state.
//
// This module contains the primary structs that represent the lifecycle of an
// audio editing project in `shutri`. The data model is designed to support
// a three-phase workflow:
//
// 1.  **Pre-processing:** An audio file is broken into `Split`s based on silence.
//     These are then merged into consolidated `Split`s.
// 2.  **Transcription:** The audio from each `Split` is transcribed into a `Clip`.
// 3.  **Structuring:** `Clip`s are grouped into logical `Chunk`s for presentation.
//
// The top-level `Project` struct orchestrates this entire process.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// --- Core Data Structures ---

/// Represents the entire state of a `shutri` project.
///
/// This is the main, top-level struct that holds all data related to a single
/// audio editing project. It tracks the original source file and the evolution
/// of the audio data through splits, clips, and chunks.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    /// The name of the project, typically derived from the original audio file.
    /// e.g., "podcast_episode_1"
    pub name: String,
    /// The absolute path to the original imported audio file. This is the source
    /// of truth for all audio operations.
    pub original_audio_path: PathBuf,
    /// The manifest of audio splits after pre-processing. This is the definitive
    /// list of audio segments that will be transcribed.
    pub splits: Vec<Split>,
    /// The list of transcribed clips generated from the splits.
    pub clips: Vec<Clip>,
    /// The final, structured list of chunks that group clips for presentation.
    pub chunks: Vec<Chunk>,
}

/// Represents a segment of audio created by splitting the source file at silences.
///
/// A `Split` is a physical audio file accompanied by metadata that tracks its
/// position within the original audio. This is the fundamental unit for transcription.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Split {
    /// The path to the audio file for this split.
    /// e.g., "/home/user/.shutri/imports/project_name/split001.mp3"
    pub file_path: PathBuf,
    /// The duration of the split in milliseconds.
    pub duration_ms: u64,
    /// The start time of the split relative to the beginning of the original
    /// audio file, in milliseconds.
    pub start_time_ms: u64,
    /// The end time of the split relative to the beginning of the original
    /// audio file, in milliseconds.
    pub end_time_ms: u64,
}

/// Represents a transcribed piece of text corresponding to a `Split`.
///
/// A `Clip` is the text content that the user will edit. Its timestamps are
/// inherited directly from the `Split` it was generated from, ensuring accuracy.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

/// Represents a logical grouping of `Clip`s for presentation in the editor.
///
/// A `Chunk` is not a separate audio file but a container that groups clips
/// into manageable sections, typically around one minute in length. This helps
/// organize the editing interface for the user.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Chunk {
    /// The start time of the chunk, determined by the first clip it contains.
    pub start_time_ms: u64,
    /// The end time of the chunk, determined by the last clip it contains.
    pub end_time_ms: u64,
    /// The vector of clips that belong to this chunk.
    pub clips: Vec<Clip>,
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::path::PathBuf;

    /// Creates a mock `Project` instance for use in tests.
    fn create_mock_project() -> Project {
        let split1 = Split {
            file_path: PathBuf::from("/path/to/split1.mp3"),
            duration_ms: 10000,
            start_time_ms: 0,
            end_time_ms: 10000,
        };
        let clip1 = Clip {
            start_time_ms: 0,
            end_time_ms: 10000,
            text: "This is the first clip.".to_string(),
        };
        let chunk1 = Chunk {
            start_time_ms: 0,
            end_time_ms: 10000,
            clips: vec![clip1.clone()],
        };

        Project {
            name: "test_project".to_string(),
            original_audio_path: PathBuf::from("/path/to/audio.mp3"),
            splits: vec![split1],
            clips: vec![clip1],
            chunks: vec![chunk1],
        }
    }

    /// Tests that a `Project` struct can be created successfully.
    #[test]
    fn test_project_creation() {
        let project = create_mock_project();
        assert_eq!(project.name, "test_project");
        assert_eq!(project.splits.len(), 1);
        assert_eq!(project.clips.len(), 1);
        assert_eq!(project.chunks.len(), 1);
        assert_eq!(
            project.clips[0].text,
            "This is the first clip.".to_string()
        );
    }

    /// Tests that a `Project` struct can be serialized to a JSON string and
    /// then deserialized back into a `Project` struct, ensuring that the
    /// data remains identical. This validates the data model's integrity.
    #[test]
    fn test_project_serialization_deserialization() {
        let original_project = create_mock_project();

        // Serialize the project to a JSON string.
        let serialized = serde_json::to_string_pretty(&original_project)
            .expect("Failed to serialize project.");
        assert!(!serialized.is_empty());

        // Deserialize the JSON string back to a Project struct.
        let deserialized_project: Project = serde_json::from_str(&serialized)
            .expect("Failed to deserialize project.");

        // Assert that the original and deserialized projects are identical.
        assert_eq!(original_project, deserialized_project);
    }
}