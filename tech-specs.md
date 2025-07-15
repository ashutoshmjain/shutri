# `shutri` - Technical Specification

## 1. Overview

### 1.1. Core Concept

`shutri` is a Rust application for text-based audio editing, integrating the audio processing power of SoX (Sound eXchange) with the efficiency of the Vim editor. The core concept is to transform the traditionally cumbersome process of waveform manipulation into a precise, text-driven workflow.

The primary workflow consists of four main stages:

1.  **Import:** A user-provided MP3 audio file is imported into the `shutri` library. To ensure timestamp accuracy and enable parallel processing, the audio is immediately split into fixed-duration (e.g., 30-second) "chunks."

2.  **Transcribe:** Each audio chunk is sent to the Gemini API for transcription. This chunk-based approach mitigates the timestamp drift often found in long-form transcriptions and allows for faster, parallelized API requests. The result is a series of clips—text snippets with corresponding start and end times—grouped under their parent chunk.

3.  **Edit:** The transcription is presented to the user as a structured text file inside Vim. In this interface, the user can finely adjust the timestamps of each clip, delete unwanted clips by deleting lines, and navigate the audio by searching for text. The chunk timestamps are fixed, but the clip timestamps within them are fully editable. The Vim plugin allows the user to play back both the original audio chunks (for context) and the edited clips (for preview).

4.  **Export:** Once editing is complete, `shutri` reads the modified text file. It uses the final timestamps to extract the corresponding audio segments from the original file and concatenates them into a new, seamless MP3 audio file.

### 1.2. The Problem with Waveform-Based Editing

Traditional Digital Audio Workstations (DAWs) and audio editors rely on a visual representation of the audio waveform. This approach has several drawbacks, especially for long-form content like podcasts, interviews, or lectures:

*   **Difficult Navigation:** Navigating long recordings using a waveform is often imprecise and slow.
*   **Inflexible Markers:** Markers or regions are often cumbersome to manage and lack the flexibility of text-based search and manipulation.
*   **High Cognitive Load:** The waveform is a graphical abstraction that requires visual interpretation but does not directly represent the spoken content, which is the primary focus of the editing task.

### 1.3. The Opportunity: Text-Based Editing

By converting audio to a time-stamped transcript, we transform the editing process into a text-manipulation task. This allows us to leverage the powerful and highly efficient text-editing capabilities of Vim, including:

*   **Advanced Search and Navigation:** Instantly jump to any part of the audio by searching for specific words or phrases.
*   **Keyboard-Centric Workflow:** A faster and more ergonomic workflow for users familiar with Vim's keybindings.

---

## 2. System Architecture

The `shutri` system is composed of three main components:

1.  **`shutri` Core Library:** A Rust library that contains the core logic for project management, audio processing, transcription, and file I/O.
2.  **`shutri` CLI:** A command-line interface that exposes the core library's functionality to the user.
3.  **Vim Plugin:** A Vim plugin that integrates `shutri` with the Vim editor, providing a seamless editing experience.

### 2.1. Component Interaction Diagram

```
+-----------------+      +-----------------+      +-----------------+
|   `shutri` CLI  |----->|`shutri` Core Lib|----->|   Vim Plugin    |
+-----------------+      +-----------------+      +-----------------+
        |                      |                      |
        |                      |                      |
        v                      v                      v
+-----------------+      +-----------------+      +-----------------+
|   User Shell    |      |  File System    |      |   Vim Editor    |
+-----------------+      +-----------------+      +-----------------+
                               |
                               |
                               v
                        +-------------+
                        |     SoX     |
                        +-------------+
                               |
                               |
                               v
                        +-------------+
                        |   Gemini    |
                        +-------------+
```

---

## 3. Dependencies

### 3.1. Runtime Dependencies (for Users)

*   **SoX (Sound eXchange):** Required for all audio manipulation tasks (clipping, concatenation, playback). Must be installed and available in the system's `PATH`. For Version 1, `shutri` will focus exclusively on the MP3 format.
*   **Vim/Neovim:** A compatible version of Vim or Neovim is required for the editing workflow.

### 3.2. Development Dependencies (for the Worker)

*   **Rust Toolchain:** The latest stable version of the Rust toolchain (`rustc`, `cargo`).
*   **`rust-analyzer`:** Recommended for IDE support.
*   **Crates (Rust Libraries):**
    *   `clap`: For parsing command-line arguments.
    *   `serde`: For serialization and deserialization of project files and API responses.
    *   `tokio`: For asynchronous operations (e.g., API calls).
    *   `reqwest`: For making HTTP requests to the Gemini API.
    *   `anyhow`, `thiserror`: For robust error handling.
    *   `log`, `env_logger`: For logging.

---

## 4. Solution & Workflow

### 4.1. The Project Library

A central directory at `~/.shutri/` will be used to manage all assets, organized as follows:

*   `~/.shutri/projects/`: Contains the `.shutri` project files.
*   `~/.shutri/imports/`: Stores the original audio files and their chunks.
*   `~/.shutri/exports/`: The output directory for final, rendered audio files.
*   `~/.shutri/cache/`: Stores transcription results to avoid redundant API calls.

#### 4.1.1. Project Files (`.shutri`)

A `.shutri` file is a text file that represents the state of an editing project. To make navigation easier, the file is visually structured into the chunks that were used for transcription.

Lines starting with `//` are treated as comments. They are used to delineate the chunks and to provide informational notes to the user. The chunk-level timestamps in these comments are for reference only and should not be edited.

Each editable line corresponds to an audio clip and follows this format:
`[start_time] text [end_time]`

*   **`start_time` / `end_time`:** Time-stamps in `MM:SS.ms` format (e.g., `00:01.234`). **Important:** All timestamps are absolute, relative to the beginning of the original audio file.
*   **`text`:** The transcribed text for the clip.

**Example Vim Interface (`.shutri` file):**

```vim
" Project: podcast_episode_1.mp3
"
" Keybindings:
"  <Leader>p : Play current clip (preview your edit)
"  <Leader>c : Play current chunk (hear the original audio)
" =============================================================================

// --- CHUNK 1 (00:00.000 - 00:30.000) ---
[00:01.123] This is a valid clip. [00:05.450]
[00:28.200] This clip extends beyond the chunk boundary due to AI drift. [00:31.500] // INFO: Review recommended.

// --- CHUNK 2 (00:30.000 - 01:00.000) ---
[00:30.500] ...and from there I moved to the city. [00:36.900]
```
*The Vim plugin will highlight the line containing the `// INFO` comment to draw the user's attention to it for review.*

### 4.2. Operations

#### 4.2.1. Import (`shutri -i <file.mp3>`)

1.  The user specifies an MP3 audio file to import.
2.  `shutri` copies the file to `~/.shutri/imports/`.
3.  The audio file is split into smaller chunks (e.g., 30 seconds each) using SoX.

**Pseudocode (Rust):**

```rust
mod audio {
    fn import_audio(file_path: &Path) -> Result<Project, Error> {
        // 1. Validate file is in MP3 format.
        // 2. Create a new project directory in `~/.shutri/projects/`
        // 3. Copy the original file to `~/.shutri/imports/`
        // 4. Use SoX to split the audio into chunks
        //    - `sox <input.mp3> <output_chunk.mp3> trim <start> <duration>`
        // 5. Return a new `Project` struct
    }
}
```

#### 4.2.2. Transcribe (`shutri -t <file>`)

1.  `shutri` sends each audio chunk to the Gemini API for transcription.
2.  The transcription results are cached in `~/.shutri/cache/`.
3.  Before writing the `.shutri` file, a boundary check is performed. If a clip's end time exceeds its chunk's end time, an informational comment (`// INFO: Review recommended.`) is appended to the line.
4.  The final, structured `.shutri` project file is generated.

**Pseudocode (Rust):**

```rust
mod transcription {
    async fn transcribe_project(project: &mut Project) -> Result<(), Error> {
        // 1. For each audio chunk in the project:
        // 2.   - Get transcription from cache or Gemini API.
        // 3. For each clip in the transcription:
        // 4.   - If clip_end_time > chunk_end_time, append informational comment.
        // 5. Write the `Project` data to a `.shutri` file.
    }
}
```

#### 4.2.3. Edit (`shutri -v <file>`)

1.  `shutri` invokes Vim, opening the `.shutri` project file.
2.  The user edits the file, adjusting clip time-stamps, deleting lines, or adding personal comments.
3.  The Vim plugin provides keybindings for an enhanced editing workflow:
    *   **Playback Controls:**
        *   `<Leader>c`: **Play Chunk**. Plays the original, unmodified audio chunk.
        *   `<Leader>p`: **Play Clip**. Plays the audio segment for the current line to preview an edit.
        *   `<Leader>s`: Stop all playback.
    *   **Timestamp Nudging:**
        *   `<Leader>[`, `<Leader>]`: Nudge the start time of the current clip.
        *   `<Leader>{`, `<Leader>}`: Nudge the end time of the current clip.

**Vimscript (for the plugin):**

```vim
" Play the current clip (previews the edit)
nnoremap <Leader>p :call ShutriPlayClip()<CR>

" Play the current chunk (provides context)
nnoremap <Leader>c :call ShutriPlayChunk()<CR>

" Highlight lines needing review
highlight ShutriReview ctermbg=yellow guibg=yellow
match ShutriReview /\/\/ INFO: Review recommended./
```

#### 4.2.4. Export (`shutri -e <file>`)

1.  `shutri` reads the edited `.shutri` project file, ignoring all comment lines.
2.  It uses SoX to extract each audio clip from the original imported file based on the final time-stamps.
3.  The extracted clips are concatenated in order.
4.  The final, combined audio is saved to the `~/.shutri/exports/` directory as an MP3 file.

**Pseudocode (Rust):**

```rust
mod audio {
    fn export_project(project: &Project) -> Result<(), Error> {
        // 1. Read the `.shutri` file.
        // 2. For each line (clip) in the file:
        // 3.   - Parse the start and end times.
        // 4.   - Use SoX to extract the audio segment:
        //        `sox <original.mp3> <clip.mp3> trim <start> =<end>`
        // 5. Create a list of the extracted clip files.
        // 6. Use SoX to concatenate the clips:
        //    `sox <clip1.mp3> <clip2.mp3> ... <output.mp3>`
        // 7. Clean up the temporary clip files.
    }
}
```

---

## 5. Invocation and Options

### 5.1. Invocation

The primary invocation is `shutri <file.mp3>`. This will:

1.  Detect if a project for the file already exists.
2.  If not, it will automatically import and transcribe the file.
3.  Once transcription is complete, it will open the project in Vim.

#### 5.1.1. User Experience

For long-running operations like import and transcription, the CLI must provide clear, continuous feedback to the user.

*   **Status Updates:** Display simple, human-readable messages for each major step (e.g., "Importing audio...", "Splitting into 3 chunks...", "Transcribing chunk 1 of 3...").
*   **Engaging Feedback:** During the transcription phase, which can be time-consuming, the CLI should display a series of engaging, humorous, or informative messages to keep the user entertained and aware that the process is still running. This is similar to the experience provided by modern interactive CLIs.

### 5.2. Command-Line Options

*   `shutri -i, --import <file.mp3>`: Import an audio file.
*   `shutri -t, --transcribe <project>`: Transcribe an imported project.
*   `shutri -e, --export <project>`: Export a project to a final audio file.
*   `shutri -v, --edit <project>`: Open a project in Vim for editing.
*   `--no-cache`: Force re-transcription, ignoring any cached results.
*   `--debug`: Enable verbose logging for debugging purposes.

---

## 6. Rust Module Structure

The `shutri` crate will be organized into the following modules:

*   `main.rs`: Entry point of the application, handles command-line argument parsing and dispatches to the appropriate modules.
*   `cli.rs`: Defines the command-line interface using `clap`.
*   `project.rs`: Defines the `Project` struct and related functions for managing project state.
*   `audio.rs`: Contains all logic related to audio processing using SoX.
*   `transcription.rs`: Handles communication with the Gemini API and manages the transcription cache.
*   `vim.rs`: Logic for interacting with the Vim editor and the associated plugin.
*   `config.rs`: Manages application configuration (e.g., API keys, paths).
*   `error.rs`: Defines custom error types for the application.

---

## 7. Error Handling

Errors will be handled using the `anyhow` and `thiserror` crates. A custom `Error` enum will be defined to represent all possible error conditions, such as:

*   File not found or invalid format (not MP3).
*   SoX command failed.
*   API request failed.
*   Invalid project file format.

Errors will be logged to a debug file (if enabled) and presented to the user in a clear and informative way.

---

## 8. Configuration

Application configuration will be stored in a file at `~/.config/shutri/config.toml`. This file will contain:

*   **Gemini API Key:** The user's API key for the transcription service.
*   **Paths:** The paths to the `shutri` library directories (e.g., `projects`, `imports`).
*   **Editor:** The command to invoke the user's preferred editor (e.g., `vim`, `nvim`).

---

## 9. Testing Strategy

The testing strategy will include:

*   **Unit Tests:** For individual functions in each module (e.g., parsing time-stamps, validating project files).
*   **Integration Tests:** For workflows that involve multiple modules (e.g., the full import-transcribe-export process). These tests will use mock objects for the Gemini API to avoid making real network requests.
*   **End-to-End Tests:** A suite of shell scripts that will test the `shutri` CLI from the user's perspective, using small, sample MP3 files.

---

## 10. Development Plan

This project will be developed in a series of testable milestones. Each milestone builds directly on the previous one, ensuring a logical and verifiable development process that delivers interactive value early.

### Milestone 1: Project Setup and Core Data Structures

*   **Goal:** Initialize the Rust project and define the core data structures.
*   **Tasks:**
    *   Run `cargo init` to create the project structure.
    *   Add initial dependencies to `Cargo.toml`.
    *   Define the `Project` struct and other core data types.
*   **Testable Outcome:** The project compiles successfully. Unit tests for the data structures pass.

### Milestone 2: Audio Import and Chunking

*   **Goal:** Implement the ability to import an MP3 file and split it into manageable chunks.
*   **Tasks:**
    *   Implement the `import_audio` function in `audio.rs`.
    *   Use `std::process::Command` to call the `sox` command-line tool.
    *   Implement the `shutri -i, --import` CLI command.
*   **Testable Outcome:** Running `shutri --import <path/to/audio.mp3>` correctly creates a project with audio chunks in the `~/.shutri` directory.

### Milestone 3: Mocked Transcription File Generation

*   **Goal:** Generate a `.shutri` project file with valid, mock data corresponding to a real audio project.
*   **Tasks:**
    *   Implement a mock transcription function that generates dummy text but with **valid timestamps** that fall within the chunk boundaries of a real project from Milestone 2.
    *   Implement the boundary check logic to append informational comments.
*   **Testable Outcome:** Running `shutri --transcribe --mock <project_name>` generates a correctly formatted `.shutri` file that is ready for interactive use.

### Milestone 4: Vim Integration & Playback

*   **Goal:** Create the core interactive editing loop within Vim.
*   **Tasks:**
    *   Create the basic Vim plugin (`shutri.vim`) with highlight and match rules.
    *   Implement the `ShutriPlayClip()` and `ShutriPlayChunk()` functions in Vimscript, which will call the main `shutri` binary.
    *   Implement the `shutri -v, --edit` command.
*   **Testable Outcome:** Running `shutri --edit <project_name>` opens Vim. Boundary-crossing clips are highlighted. `<Leader>p` and `<Leader>c` play the correct audio from the real audio file.

### Milestone 5: Audio Export

*   **Goal:** Combine the edited audio clips into a final MP3 file.
*   **Tasks:**
    *   Implement the `export_project` function in `audio.rs`.
    *   Implement the `shutri -e, --export` CLI command.
*   **Testable Outcome:** Running `shutri --export <project_name>` on a (mock or real) edited project generates a final MP3 file. The audio content matches the edits made.

### Milestone 6: Real Transcription Service

*   **Goal:** Replace the mock transcription service with a real implementation using the Gemini API.
*   **Tasks:**
    *   Use the `reqwest` crate to make HTTP requests to the Gemini API.
    *   Implement API key management and configuration.
    *   Implement caching logic.
*   **Testable Outcome:** Running `shutri --transcribe <project_name> --no-cache` populates the `.shutri` file with a real transcription.

### Milestone 7: Polish and Finalize

*   **Goal:** Finalize the CLI, implement robust error handling, and improve the user experience.
*   **Tasks:**
    *   Implement the main `shutri <file.mp3>` invocation.
    *   Implement the engaging command-line feedback as described in Section 5.1.1.
    *   Add comprehensive error handling and user-friendly error messages.
    *   Write end-to-end tests and create documentation.
*   **Testable Outcome:** The application is fully functional, robust, and user-friendly. The end-to-end test suite passes.
---

## 11. Future Directions

This section outlines potential features and enhancements that could be considered for future versions of `shutri`, beyond the core functionality described in this document.

### 11.1. Programmable Editing & Effects

While the initial version focuses on manual, precise editing, the text-based nature of the `.shutri` file opens up powerful possibilities for automation. Future versions could introduce features for "programmable editing," where the user can apply changes to multiple clips at once using scripts or commands.

Examples include:

*   **Automated Filler Word Removal:** A command to find and delete all clips that only contain "um," "uh," or other specified filler words.
*   **Silence Adjustment:** A function to automatically shorten or lengthen silences between clips to meet a specific duration.
*   **Applying Audio Effects:** The `.shutri` format could be extended to support applying SoX effects to specific clips.

### 11.2. Advanced Vim Integration

The Vim plugin could be enhanced with more sophisticated features, such as:

*   **Visual Highlighting:** Highlighting the currently playing clip or chunk in the Vim buffer.
*   **Multi-Clip Operations:** Allowing users to visually select multiple lines (clips) and perform actions on them, such as playing them in sequence or deleting them all at once.
*   **Speaker Identification:** If the transcription service provides speaker diarization, this information could be displayed in the `.shutri` file, allowing for speaker-specific edits.
