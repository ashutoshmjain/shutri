# `shutri` - Technical Specification

## 1. Overview

### 1.1. Core Concept

`shutri` is a Rust application designed for text-based audio editing. It integrates the audio processing capabilities of SoX (Sound eXchange) with the text-editing power of Vim (Vi Improved). The core idea is to replace the traditional, often cumbersome, waveform-based audio editing with a more efficient and precise text-based workflow.

The primary workflow is as follows:

1.  **Import & Transcribe:** The user provides an audio file (e.g., `.mp3`). `shutri` uses an external transcription service (initially, Gemini) to convert the audio into a time-stamped transcript. Each segment of transcribed text is associated with a precise start and end time, forming a "clip."
2.  **Edit:** The user edits this transcript file within Vim. The editing process primarily involves manipulating the time-stamps to adjust the audio clips. The transcribed text serves as a searchable and navigable guide to the audio content.
3.  **Export:** Once the editing is complete, `shutri` uses SoX to concatenate the edited audio clips into a final, seamless audio file.

### 1.2. The Problem with Waveform-Based Editing

Traditional Digital Audio Workstations (DAWs) and audio editors rely on a visual representation of the audio waveform. This approach has several drawbacks, especially for long-form content like podcasts, interviews, or lectures:

*   **Difficult Navigation:** Navigating long recordings using a waveform is often imprecise and slow.
*   **Inflexible Markers:** Markers or regions are often cumbersome to manage and lack the flexibility of text-based search and manipulation.
*   **High Cognitive Load:** The waveform is a graphical abstraction that requires visual interpretation but does not directly represent the spoken content, which is the primary focus of the editing task.

### 1.3. The Opportunity: Text-Based Editing

By converting audio to a time-stamped transcript, we transform the editing process into a text-manipulation task. This allows us to leverage the powerful and highly efficient text-editing capabilities of Vim, including:

*   **Advanced Search and Navigation:** Instantly jump to any part of the audio by searching for specific words or phrases.
*   **Programmable Editing:** Use Vim's macros and scripting capabilities to automate repetitive editing tasks.
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

*   **SoX (Sound eXchange):** Required for all audio manipulation tasks (clipping, concatenation, playback). Must be installed and available in the system's `PATH`.
*   **Vim/Neovim:** A compatible version of Vim or Neovim is required for the editing workflow.
*   **FFmpeg (Optional but Recommended):** While SoX will be the primary tool for audio manipulation, FFmpeg may be used for initial import and conversion from a wider range of audio/video formats.

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

A `.shutri` file is a text file that represents the state of an editing project. Each line in the file corresponds to an audio clip and follows this format:

```
[start_time] text [end_time]
```

*   **`start_time` / `end_time`:** Time-stamps in `MM:SS.ms` format (e.g., `00:01.234`).
*   **`text`:** The transcribed text for the clip.

Lines starting with `//` are treated as comments and are ignored during processing.

### 4.2. Operations

#### 4.2.1. Import (`shutri -i <file>`)

1.  The user specifies an audio file to import.
2.  `shutri` copies the file to `~/.shutri/imports/`.
3.  The audio file is split into smaller chunks (e.g., 30 seconds each) using SoX. This is done to improve the efficiency of transcription and to allow for parallel processing.

**Pseudocode (Rust):**

```rust
mod audio {
    fn import_audio(file_path: &Path) -> Result<Project, Error> {
        // 1. Validate file format (mp3, wav, etc.)
        // 2. Create a new project directory in `~/.shutri/projects/`
        // 3. Copy the original file to `~/.shutri/imports/`
        // 4. Use SoX to split the audio into chunks
        //    - `sox <input_file> <output_chunk> trim <start> <duration>`
        // 5. Return a new `Project` struct
    }
}
```

#### 4.2.2. Transcribe (`shutri -t <file>`)

1.  `shutri` sends each audio chunk to the Gemini API for transcription.
2.  The transcription results, including time-stamps, are cached in `~/.shutri/cache/`. The cache key will be a hash of the audio chunk's content.
3.  A `.shutri` project file is generated from the cached transcriptions.

**Pseudocode (Rust):**

```rust
mod transcription {
    async fn transcribe_project(project: &mut Project) -> Result<(), Error> {
        // 1. For each audio chunk in the project:
        // 2.   - Calculate the hash of the chunk file.
        // 3.   - Check if a cached transcription exists for this hash.
        // 4.   - If not, send the chunk to the Gemini API.
        // 5.   - Store the transcription result in the cache.
        // 6.   - Add the transcription to the `Project` struct.
        // 7. Write the `Project` data to a `.shutri` file.
    }
}
```

#### 4.2.3. Edit (`shutri -v <file>`)

1.  `shutri` invokes Vim, opening the `.shutri` project file.
2.  The user edits the file, adjusting time-stamps, deleting lines (clips), or adding comments.
3.  The Vim plugin provides keybindings for enhanced functionality:
    *   `<Leader>p`: Play the current clip.
    *   `<Leader>P`: Play all clips from the current one to the end.
    *   `<Leader>s`: Stop playback.
    *   `<Leader>[`, `<Leader>]`: Nudge the start time of the current clip.
    *   `<Leader>{`, `<Leader>}`: Nudge the end time of the current clip.

**Vimscript (for the plugin):**

```vim
" Play the current clip
nnoremap <Leader>p :call ShutriPlayClip()<CR>

function! ShutriPlayClip()
    " Get the current line
    let line = getline('.')
    " Parse start and end times
    " ...
    " Call SoX to play the clip
    " system('sox <input_file> -d trim <start> =<end>')
endfunction
```

#### 4.2.4. Export (`shutri -e <file>`)

1.  `shutri` reads the edited `.shutri` project file.
2.  It uses SoX to extract each audio clip from the original imported file based on the (potentially modified) time-stamps.
3.  The extracted clips are concatenated in the order they appear in the `.shutri` file.
4.  The final, combined audio is saved to the `~/.shutri/exports/` directory.

**Pseudocode (Rust):**

```rust
mod audio {
    fn export_project(project: &Project) -> Result<(), Error> {
        // 1. Read the `.shutri` file.
        // 2. For each line (clip) in the file:
        // 3.   - Parse the start and end times.
        // 4.   - Use SoX to extract the audio segment:
        //        `sox <original_file> <clip_file> trim <start> =<end>`
        // 5. Create a list of the extracted clip files.
        // 6. Use SoX to concatenate the clips:
        //    `sox <clip1> <clip2> ... <output_file>`
        // 7. Clean up the temporary clip files.
    }
}
```

---

## 5. Invocation and Options

### 5.1. Invocation

The primary invocation is `shutri <file>`. This will:

1.  Detect if a project for `<file>` already exists.
2.  If not, it will automatically import and transcribe the file in the background, showing progress on the command line.
3.  Once transcription is complete, it will open the project in Vim.

### 5.2. Command-Line Options

*   `shutri -i, --import <file>`: Import an audio file.
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

*   File not found.
*   Invalid audio format.
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
*   **End-to-End Tests:** A suite of shell scripts that will test the `shutri` CLI from the user's perspective, using small, sample audio files.

---

## 10. Development Plan

This project will be developed in a series of testable milestones. Each milestone will result in a functional piece of the application that can be tested independently via the CLI.

### Milestone 1: Project Setup and Core Data Structures

*   **Goal:** Initialize the Rust project and define the core data structures.
*   **Tasks:**
    *   Run `cargo init` to create the project structure.
    *   Add initial dependencies to `Cargo.toml` (`clap`, `serde`, `anyhow`, `thiserror`).
    *   Create the module files as outlined in Section 6.
    *   Define the `Project` struct in `project.rs` and other core data types.
*   **Testable Outcome:**
    *   The project compiles successfully.
    *   Unit tests for the `Project` struct and its methods pass.

### Milestone 2: Audio Import and Chunking

*   **Goal:** Implement the ability to import an audio file and split it into manageable chunks.
*   **Tasks:**
    *   Implement the `import_audio` function in `audio.rs`.
    *   Use `std::process::Command` to call the `sox` command-line tool.
    *   Create the necessary directory structure in `~/.shutri/`.
    *   Implement the `shutri -i, --import` CLI command.
*   **Testable Outcome:**
    *   Run `shutri --import <path/to/audio.mp3>`.
    *   Verify that the audio file is copied to `~/.shutri/imports/`.
    *   Verify that the audio file is split into multiple chunk files in a corresponding project directory.

### Milestone 3: Mocked Transcription

*   **Goal:** Generate a `.shutri` project file from an imported audio project using a mocked transcription service.
*   **Tasks:**
    *   Implement the `transcribe_project` function in `transcription.rs`.
    *   Create a mock transcription function that returns dummy text and time-stamps for each audio chunk.
    *   Implement the `shutri -t, --transcribe` CLI command.
*   **Testable Outcome:**
    *   Run `shutri --transcribe <project_name>`.
    *   Verify that a `<project_name>.shutri` file is created.
    *   Verify that the `.shutri` file contains lines in the format `[start_time] text [end_time]`.

### Milestone 4: Audio Export

*   **Goal:** Combine audio clips based on a `.shutri` file to produce a final audio file.
*   **Tasks:**
    *   Implement the `export_project` function in `audio.rs`.
    *   Parse the `.shutri` file to get the list of clips and their time-stamps.
    *   Use `sox` to extract and concatenate the audio clips.
    *   Implement the `shutri -e, --export` CLI command.
*   **Testable Outcome:**
    *   Run `shutri --export <project_name>`.
    *   Verify that a final audio file is created in `~/.shutri/exports/`.
    *   Listen to the exported audio to ensure the clips are combined correctly.

### Milestone 5: Vim Integration (Basic Playback)

*   **Goal:** Allow the user to play back individual audio clips from within Vim.
*   **Tasks:**
    *   Create a basic Vim plugin (`shutri.vim`).
    *   Implement the `ShutriPlayClip()` function in Vimscript.
    *   This function will call `shutri` with special arguments to play a specific time range of the original audio file.
    *   Implement the `shutri -v, --edit` command to open the project in Vim.
*   **Testable Outcome:**
    *   Run `shutri --edit <project_name>`.
    *   Inside Vim, place the cursor on a clip line and press `<Leader>p`.
    *   Verify that the corresponding audio clip plays.

### Milestone 6: Real Transcription Service

*   **Goal:** Replace the mock transcription service with a real implementation using the Gemini API.
*   **Tasks:**
    *   Use the `reqwest` crate to make HTTP requests to the Gemini API.
    *   Implement API key management and configuration (`~/.config/shutri/config.toml`).
    *   Implement the caching logic described in Section 4.2.2.
*   **Testable Outcome:**
    *   Configure the Gemini API key.
    *   Run `shutri --transcribe <project_name> --no-cache`.
    *   Verify that the `.shutri` file is populated with a real transcription of the audio.

### Milestone 7: Polish and Finalize

*   **Goal:** Finalize the CLI, implement robust error handling, and improve the user experience.
*   **Tasks:**
    *   Implement the main `shutri <file>` invocation.
    *   Add comprehensive error handling and user-friendly error messages.
    *   Implement the remaining CLI options (`--debug`, etc.).
    *   Write end-to-end tests using shell scripts.
    *   Create documentation (README, user guide).
*   **Testable Outcome:**
    *   The application is fully functional and robust.
    *   All CLI commands and options work as expected.
    *   The end-to-end test suite passes.