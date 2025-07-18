# `shutri` - Technical Specification

## 1. Overview

### 1.1. Core Concept

`shutri` is a Rust application for text-based audio editing, integrating the audio processing power of SoX (Sound eXchange) with the efficiency of the Vim editor. The core concept is to transform the traditionally cumbersome process of waveform manipulation into a precise, text-driven workflow.

The initial version of `shutri` is specifically targeted for **Debian-based Linux distributions** (e.g., Debian, Ubuntu). All development and testing will be focused on this platform.

The primary workflow consists of four main stages:

1.  **Import:** A user-provided MP3 audio file is imported into the `shutri` library. To ensure that edits align with natural pauses in speech and to enable parallel processing, the audio is immediately split into variable-length "chunks" based on detected silence. This approach avoids splitting words or sentences in half, which can happen with fixed-duration chunking.

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
* The crucial requirement for this approach however is accuracy of timestamps. And ability to easily adjust the timestamps of the clips.

---

## 2. System Architecture

The `shutri` system is composed of three main components:

1.  **`shutri` Core Library:** A Rust library that contains the core logic for project management, audio processing, transcription, and file I/O.
2.  **`shutri` CLI:** A command-line interface that exposes the core library's functionality to the user.
3.  **Vim Plugin:** A Vim plugin that integrates `shutri` with the Vim editor, providing a seamless editing experience.

In a way, the key deliverable of this project is the "core library". CLI is a means to test the application. And vim is the editor. The goal of the project is developers shall be able to use the library crate to integrate with any type of editor. The application needs to provide clear APIs for future GUIs or web clients ; and also for any modern editor to integrate. API documentation is crucial part of the project.

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

*   **`shutri` binary:** The compiled Rust application itself. Users will need to have this binary in their system's `PATH`.
*   **SoX (Sound eXchange):** Required for all audio manipulation tasks. Must be installed and available in the system's `PATH`. Version 14.4.2 or higher is recommended.
*   **Vim/Neovim:** A compatible version of Vim (8.0 or higher) or Neovim (0.4 or higher) is required for the editing workflow.
*   **Configuration File:** Users must configure their Gemini authentication method in `~/.config/shutri/config.toml`. This can be done by adding a developer API key directly or by running `shutri auth login` to sign in with a Google account.

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
3.  The audio is processed using a detailed, three-phase strategy to create structured, editable data.

  **Phase 1: Pre-processing (Creating Consolidated Splits)**

  1.  **Initial Split & Manifest Creation:** The source MP3 is split at every silence of **0.6 seconds** or more. A **Split Manifest** is immediately created to track the `filePath`, `duration`, and absolute `startTime` and `endTime` for each resulting `SPLIT`.

  2.  **Iterative Merging of Short Splits:** The manifest is repeatedly scanned to find any `SPLIT` shorter than **6 seconds**. These are merged based on defined rules (e.g., a short split in the middle of the list is merged with its shorter neighbor) until no splits under 6 seconds remain. The manifest is updated after each merge.

  **Phase 2: Transcription (Creating Clips)**

  3.  **Transcribe and Associate Timestamps:** Each `SPLIT` from the final manifest is sent to the Gemini API. The returned text becomes a `CLIP` and is paired with the accurate `startTime` and `endTime` from the manifest.

  **Phase 3: Structuring for Presentation (Creating Chunks)**

  4.  **Chunking Algorithm:** The `CLIPS` are grouped into logical `CHUNKS` for the user interface based on two primary rules:
      *   **Large Split Override:** If a `CLIP`'s source `SPLIT` is **longer than 60 seconds**, it becomes its own `CHUNK`.
      *   **Greedy Grouping:** Otherwise, `CLIPS` are added to the current `CHUNK` until its total duration approaches **60 seconds**. The next `CLIP` that would exceed the limit starts a new `CHUNK`.

  This strategy ensures accurate timestamp management and provides clear rules for handling all audio structuring scenarios.


4.  A preliminary `.shutri` project file is created, containing only comment lines that define the chunk boundaries (e.g., `// --- CHUNK 1 (00:00.000 - 00:28.530) ---`). This file contains no editable clips yet.

**Pseudocode (Rust):**

```rust
mod audio {
    fn import_audio(file_path: &Path) -> Result<Project, Error> {
        // 1. Validate file is in MP3 format.
        // 2. Create a new project directory in `~/.shutri/projects/`
        // 3. Copy the original file to `~/.shutri/imports/`
        // 4. Use SoX to split the audio into chunks based on "Silence".
        //    - `sox <input.mp3> <output_chunk.mp3> silence 1 0.1 1% 1 0.6 1% : newfile : restart`
        //    - This command tells SoX to create a new file every time it detects
        //      at least 0.6 seconds of silence at 1% volume threshold.
        // 5. Merge the splits into chunks of 20 to 30 seconds based on configurable "chunking strategy" above.
        // 5. Return a new `Project` struct
    }
}
```

#### 4.2.2. Transcribe (`shutri -t <file>`)

1.  `shutri` sends each audio chunk to the Gemini API for transcription.
2.  The transcription results are cached in `~/.shutri/cache/`.
3.  Before modifying the `.shutri` file, a boundary check is performed. If a clip's end time exceeds its chunk's end time, an informational comment (`// INFO: Review recommended.`) is appended to the line.
4.  The existing `.shutri` project file is updated with the transcribed clips, inserted under their corresponding chunk markers.

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
3.  The Vim plugin provides keybindings for an enhanced editing workflow. The core audio processing logic for playback resides in the Rust binary, while the Vimscript functions act as wrappers that gather context from the editor and call the binary with the appropriate arguments.

    *   **Playback Controls:**
        *   `<Leader>c`: **Play Chunk**. Plays the original, unmodified audio chunk.
        *   `<Leader>p`: **Play Clip**. Plays the audio segment for the current line to preview an edit.
        *   `<Leader>s`: Stop all playback.
    *   **Timestamp Nudging:**
        *   `<Leader>[`, `<Leader>]`: Nudge the start time of the current clip.
        *   `<Leader>{`, `<Leader>}`: Nudge the end time of the current clip.

**Vimscript and Rust Interaction:**

The Vimscript functions are lightweight. Their job is to read the current line or identify the current chunk and then make a system call to the `shutri` binary.

For example, `ShutriPlayClip()` would be implemented in Vimscript as follows:
```vim
" Get the current line, then call the shutri binary to play it.
" The Rust code handles parsing the line and calling SoX.
function! ShutriPlayClip()
    let current_line = getline('.')
    call system('shutri --play-clip "' . current_line . '" &')
endfunction

" Gathers all clip lines in the current chunk and calls the binary to play them.
function! ShutriPlayEditedChunk()
    " This function would need logic to find the start and end of the current
    " chunk and pass all clip lines to `shutri --play-edited-chunk`.
endfunction

function! ShutriStopPlayback()
    call system('shutri --stop-playback')
endfunction
```

The corresponding logic in Rust would parse the `--play-clip` argument, extract the timestamps, and then use SoX to play that specific audio segment.

**Vimscript (for the plugin):**

```vim
" Play the current clip (previews the edit)
nnoremap <Leader>p :call ShutriPlayClip()<CR>

" Play the current chunk (provides context)
nnoremap <Leader>c :call ShutriPlayChunk()<CR>
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

Just like vim, the primary invocation is `shutri <file.mp3>`. This will:

1.  Detect if a project for the file already exists.
2.  If not, it will automatically import and transcribe the file.
3.  Once transcription is complete, it will open the project in Vim.

#### 5.1.1. User Experience

For long-running operations like import and transcription, the CLI must provide clear, continuous feedback to the user.

*   **Status Updates:** Display simple, human-readable messages for each major step (e.g., "Importing audio...", "Splitting into 3 chunks...", "Transcribing chunk 1 of 3...").
*   **Engaging Feedback:** During the transcription phase, which can be time-consuming, the CLI should display a series of engaging, humorous, or informative messages to keep the user entertained and aware that the process is still running. This is similar to the experience provided by modern interactive CLIs.

### 5.2. Command-Line Options

*   `shutri -i, --import <file.mp3>`: Import an audio file.
*   `shutri -t, --transcribe <project>`: Transcribe an imported project. This takes a project that has audio chunks and generates the `.shutri` file with transcribed text and timestamps.
*   `shutri -e, --export <project>`: Export a project to a final audio file.
*   `shutri -v, --edit <project>`: Open a project in Vim for editing.
*   `shutri auth login`: Initiates an interactive OAuth 2.0 flow to sign in with a Google account.
*   `--no-cache`: Used with a transcription command. Forces re-transcription of all audio chunks, ignoring any cached results. This is useful if the initial transcription is unsatisfactory.
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
*   `config.rs`: Manages application configuration.
*   `auth.rs`: Handles the authentication logic for both API keys and the OAuth 2.0 flow.
*   `error.rs`: Defines custom error types for the application.

---

## 7. Error Handling

Errors will be handled using the `anyhow` and `thiserror` crates. A custom `Error` enum will be defined to represent all possible error conditions, such as:

*   File not found or invalid format (not MP3).
*   SoX command failed.
*   API request failed.
*   Authentication failed.
*   Invalid project file format.

Errors will be logged to a debug file (if enabled) and presented to the user in a clear and informative way.

---

## 8. Configuration and Authentication

To provide flexibility and ease of use, `shutri` supports two methods for authenticating with the Gemini API. Configuration is stored at `~/.config/shutri/config.toml`. The `install.sh` script will generate this file with commented-out templates to guide the user.

### 8.1. Method 1: Developer API Key (Manual)

For developers and users who prefer to manage keys manually, a static API key can be placed directly in the configuration file.

### 8.2. Method 2: Sign in with Google (Interactive)

For a more user-friendly experience, `shutri` provides a CLI-based OAuth 2.0 flow. The user can run `shutri auth login`, which will:
1.  Open the default web browser to a Google authentication page.
2.  Ask the user to grant `shutri` permission to access the Gemini API on their behalf.
3.  Receive an authorization token and store it securely in the `config.toml` file for future API calls.

This method avoids the need for users to generate and manage their own API keys.

### 8.3. Sample `config.toml`

The `install.sh` script will create the following file at `~/.config/shutri/config.toml`:

```toml
# This is the configuration file for shutri.
# Please choose one of the authentication methods below.

# --- Method 1: API Key ---
# For developers. Paste your Gemini API key here.
# api_key = "YOUR_API_KEY_HERE"

# --- Method 2: Sign in with Google ---
# For most users. Run 'shutri auth login' to populate this automatically.
# [oauth_token]
# access_token = "..."
# refresh_token = "..."
# expires_in = "..."

# --- General Settings ---

# The command to invoke your preferred editor (e.g., "vim", "nvim").
editor = "vim"

# Audio processing settings
[audio]
# Minimum silence duration in seconds to consider for splitting audio into chunks.
# Default: 0.5
min_silence_duration_secs = 0.5
# Target length of audio chunks in seconds after merging.
# Default: 30
target_chunk_length_secs = 30.0
```

---

## 9. Testing Strategy

The testing strategy will include:

*   **Unit Tests:** For individual functions in each module (e.g., parsing time-stamps, validating project files).
*   **Integration Tests:** For workflows that involve multiple modules (e.g., the full import-transcribe-export process). These tests will use mock objects for the Gemini API to avoid making real network requests.
*   **End-to-End Tests:** A suite of shell scripts that will test the `shutri` CLI from the user's perspective, using small, sample MP3 files. All end-to-end tests will be conducted on a reference Debian-based Linux environment to ensure stability and correctness on the target platform.

---

## 10. Quality and Defect Management

This project employs a "shift-left" quality strategy, focusing on defect prevention through rigorous, testable milestones rather than defect tracking after the fact. The development process itself is the primary tool for quality assurance.

### 10.1. Defect Measurement Strategy

*   **Primary Defect Definition:** A "primary defect" is defined as any issue that either:
    1.  Prevents the "Testable Outcome" of the current milestone from being successfully achieved.
    2.  Causes a regression by breaking the "Testable Outcome" of a previously completed and verified milestone.

*   **Zero Carry-Over Goal:** The core quality objective is to have **zero primary defects** carried over from one milestone to the next. Each milestone's "Testable Outcome" serves as a strict quality gate. Development on a new milestone does not begin until the previous one is fully functional and verified.

*   **Implicit Tracking:** Primary defects are tracked implicitly through the development and testing workflow for each milestone. The `git` commit history serves as the de facto ledger of defects identified and resolved. The number of commits required to achieve a milestone's "Testable Outcome" can be used as a rough metric for code complexity and risk.

### 10.2. Automated Process and Reporting

The project's quality process is designed for automation, providing clear, continuous feedback.

*   **Automated Verification:** The "Testable Outcome" for each milestone is verified through an automated process. This includes the full suite of unit, integration, and end-to-end tests, as well as the documentation standard check (`cargo doc --fail-on-warnings`).

*   **Continuous Integration (CI) Reporting:** In a CI/CD environment, the status of this verification process would serve as the primary quality report. A typical CI pipeline for this project would:
    1.  Run `cargo test` to execute all unit and integration tests.
    2.  Run the end-to-end test suite.
    3.  Run `cargo doc --no-deps --document-private-items --fail-on-warnings` to enforce documentation standards.
    4.  A failure in any of these steps automatically flags a primary defect and fails the build. The build remains "red" until all checks pass.

*   **Defect Reintroduction Prevention:** The comprehensive test suite is the main defense against defect reintroduction. A change that causes a previously passing test to fail is a clear signal of a regression. This provides immediate feedback, allowing for rapid correction.

---

## 11. Development Plan

This project will be developed in three distinct phases: **Prototype**, **Dev**, and **User**. Each phase consists of a series of testable milestones. This phased approach allows us to manage quality, gather feedback, and track our primary metric: **Defect Density**. Our goal is to reduce defect density by 75% in each successive phase.

### Phase 1: Prototype

This phase focuses on building the core functionality of `shutri` and validating the text-based audio editing concept. The milestones are designed to create a functional, end-to-end proof of concept.

#### Milestone 1: Project Setup and Core Data Structures

*   **Goal:** Initialize the Rust project and define the core data structures.
*   **Tasks:**
    *   Run `cargo init` to create the project structure.
    *   Add initial dependencies to `Cargo.toml`.
    *   Define the `Project` struct and other core data types.
*   **Testable Outcome:** The project compiles successfully. Unit tests for the data structures pass. The milestone's code passes the documentation standard check.

#### Milestone 2: Audio Import and Chunking

*   **Goal:** Implement the ability to import an MP3 file and split it into intelligent, variable-length chunks based on silence detection.
*   **Tasks:**
    *   Implement the `import_audio` function in `audio.rs`.
    *   Use `std::process::Command` to call the `sox` command-line tool with the `silence` effect.
    *   Implement the `shutri -i, --import` CLI command.
*   **Testable Outcome:** Running `shutri --import <path/to/audio.mp3>` correctly creates a project with audio chunks in the `~/.shutri` directory, split according to natural silences in the source file. The milestone's code passes the documentation standard check.

#### Milestone 3: Mocked Transcription File Generation

*   **Goal:** Generate a `.shutri` project file with valid, mock data corresponding to a real audio project.
*   **Tasks:**
    *   Implement a mock transcription function that generates dummy text but with **valid timestamps** that fall within the chunk boundaries of a real project from Milestone 2.
    *   Implement the boundary check logic to append informational comments.
*   **Testable Outcome:** Running `shutri --transcribe --mock <project_name>` generates a correctly formatted `.shutri` file that is ready for interactive use. The milestone's code passes the documentation standard check.

#### Milestone 4: Vim Integration & Playback

*   **Goal:** Create the core interactive editing loop within Vim.
*   **Tasks:**
    *   Create the basic Vim plugin (`shutri.vim`) with highlight and match rules.
    *   Implement the `ShutriPlayClip()` and `ShutriPlayChunk()` functions in Vimscript, which will call the main `shutri` binary.
    *   Implement the `shutri -v, --edit` command.
*   **Testable Outcome:** Running `shutri --edit <project_name>` opens Vim. Boundary-crossing clips are highlighted. `<Leader>p` and `<Leader>c` play the correct audio from the real audio file. The milestone's code passes the documentation standard check.

#### Milestone 5: Audio Export

*   **Goal:** Combine the edited audio clips into a final MP3 file.
*   **Tasks:**
    *   Implement the `export_project` function in `audio.rs`.
    *   Implement the `shutri -e, --export` CLI command.
*   **Testable Outcome:** Running `shutri --export <project_name>` on a (mock or real) edited project generates a final MP3 file. The audio content matches the edits made. The milestone's code passes the documentation standard check.

#### Milestone 6: Real Transcription Service & Authentication

*   **Goal:** Implement the real transcription service, including both manual and interactive authentication methods.
*   **Tasks:**
    *   Implement the `auth.rs` module.
    *   Implement the `shutri auth login` command with a full OAuth 2.0 flow.
    *   Update `transcription.rs` to use credentials from `config.rs`, supporting both API keys and OAuth tokens.
    *   Use the `reqwest` crate to make authenticated HTTP requests to the Gemini API.
    *   Implement caching logic.
*   **Testable Outcome:** Running `shutri --transcribe <project_name> --no-cache` populates the `.shutri` file with a real transcription, using either authentication method. The milestone's code passes the documentation standard check.

#### Milestone 7: Polish and Finalize

*   **Goal:** Finalize the CLI, implement robust error handling, and improve the user experience.
*   **Tasks:**
    *   Implement the main `shutri <file.mp3>` invocation.
    *   Implement the engaging command-line feedback as described in Section 5.1.1.
    *   Add comprehensive error handling and user-friendly error messages.
    *   Create the `install.sh` script with dependency checks.
    *   Write end-to-end tests and create documentation.
*   **Testable Outcome:** The application is fully functional, robust, and user-friendly. The end-to-end test suite passes. The milestone's code passes the documentation standard check.

### Phase 2: Dev

This phase focuses on expanding the features, stability, and reach of the application. We will build upon the validated prototype to create a more robust and feature-rich tool.

#### Milestone 8: Alpha Release & Community Feedback

*   **Goal:** Package the application for an alpha release and gather initial feedback from a small group of technical users.
*   **Tasks:**
    *   Create a stable `v0.1.0-alpha` release.
    *   Write comprehensive documentation for installation and usage.
    *   Recruit alpha testers from relevant online communities (e.g., Rust forums, podcasting groups).
*   **Testable Outcome:** At least 10 users have successfully installed and used `shutri` to edit a real audio file. A feedback survey is completed by at least 5 users.

#### Milestone 9: Feature Complete & API Stability

*   **Goal:** Implement the "Future Directions" features and stabilize the internal API.
*   **Tasks:**
    *   Implement programmable editing features (e.g., filler word removal).
    *   Enhance the Vim plugin with advanced features (e.g., visual highlighting).
    *   Refactor the codebase to establish a stable internal API for future development.
*   **Testable Outcome:** The new features are covered by integration tests. The API is documented and versioned.

#### Milestone 10: Cross-Platform Support

*   **Goal:** Add support for macOS and Windows.
*   **Tasks:**
    *   Create dedicated installation scripts (e.g., Homebrew for macOS, Chocolatey/Scoop for Windows).
    *   Set up CI/CD pipelines to test the application on all three platforms (Linux, macOS, Windows).
    *   Resolve any platform-specific issues.
*   **Testable Outcome:** The application can be successfully installed and run on all three target platforms. The end-to-end test suite passes on each platform.

### Phase 3: User

This phase focuses on preparing the application for a wider audience, including non-technical users. The emphasis is on user experience, stability, and long-term support.

#### Milestone 11: Beta Release & User Acceptance Testing (UAT)

*   **Goal:** A wider public beta to gather feedback from non-technical users.
*   **Tasks:**
    *   Create a `v0.9.0-beta` release.
    *   Simplify the installation and setup process.
    *   Create user-friendly documentation and tutorials.
    *   Actively solicit feedback through a public beta program.
*   **Testable Outcome:** The application is successfully used by at least 50 beta testers. The defect density is at least 75% lower than in the Dev phase.

#### Milestone 12: General Availability (GA) & Long-Term Support (LTS)

*   **Goal:** The official 1.0 release, with a commitment to long-term support.
*   **Tasks:**
    *   Create the `v1.0.0` release.
    *   Establish a clear process for bug reporting and feature requests.
    *   Define a long-term support (LTS) policy.
*   **Testable Outcome:** The `v1.0.0` release is published. The project has a public issue tracker and a defined support plan. The defect density is at least 75% lower than in the Beta phase.

---

## 12. Installation

To ensure a smooth setup, `shutri` will be distributed with an installation script (`install.sh`). **This script is designed specifically for Debian-based Linux distributions (e.g., Debian, Ubuntu).**

### 12.1. Installation Script (`install.sh`)

The script will perform the following steps in order:

1.  **Dependency Check:**
    *   **Check for SoX:** It will run `command -v sox` to ensure SoX is installed and in the `PATH`. If not found, it will exit with a message like: `Error: SoX is not installed. Please install it via your package manager (e.g., 'sudo apt install sox' on Debian/Ubuntu) and try again.` It will also check `sox --version` to suggest an update if the version is too old.
    *   **Check for Vim/Neovim:** It will check for `nvim` first, then `vim`. If neither is found, it will exit with an error. It will also check the version (e.g., `vim --version`) to ensure it meets the minimum requirements.

2.  **Build the Binary:**
    *   The script will run `cargo build --release` to compile the `shutri` binary. This ensures the user has the most performant version.

3.  **Install Files:**
    *   **Binary:** The compiled binary will be copied to a standard user binary location, such as `$HOME/.local/bin`. The script will check if this directory is in the user's `PATH` and provide instructions if it is not.
    *   **Vim Plugin:** The `shutri.vim` file will be copied to the appropriate Vim/Neovim plugin directory (e.g., `~/.vim/plugin/` or `~/.config/nvim/plugin/`).

4.  **Configuration:**
    *   The script will create the necessary directories under `~/.shutri/` and `~/.config/shutri/`.
    *   It will create a default `config.toml` file based on the template in Section 9.3, guiding the user on how to proceed with authentication.

### 12.2. Uninstallation

An `uninstall.sh` script will also be provided to remove the `shutri` binary, Vim plugin, and configuration files cleanly.

---

## 13. Documentation Plan

Given that `shutri` integrates several external tools and APIs (SoX, Vim, Gemini), and is intended to serve as a learning resource, documentation is a first-class deliverable, not an afterthought. Our documentation strategy is designed to make the codebase exceptionally clear, particularly for novice Rust developers or those unfamiliar with the integrated components.

### 13.1. Philosophy: Documentation as a Tutorial

The entire codebase will be documented with the mindset of creating a tutorial. We will assume the reader is a motivated beginner. The documentation for any given module, struct, or function should not just explain *what* it does, but *why* it exists, how it fits into the larger picture, and what specific challenges it solves.

### 13.2. Leveraging `rustdoc`

We will use `rustdoc` as the primary tool for generating and enforcing our documentation standards. All public APIs (`struct`s, `enum`s, `fn`s, `trait`s, and `mod`s) will be thoroughly documented using Markdown within `///` comments.

Key `rustdoc` features we will leverage:
*   **Code Examples:** Every public function will include at least one runnable doctest example. This serves as both documentation and a mini-unit test, demonstrating practical usage.
*   **Intra-doc Links:** We will use links to connect related parts of the API, making it easy for developers to navigate the codebase and understand relationships between components.
*   **Module-Level Explanations:** Each module (`mod.rs` or the file itself) will begin with a detailed explanation of its purpose, its responsibilities, and how it interacts with other modules.

### 13.3. The Documentation Standard

Our standard for documentation is that **a novice programmer should be able to debug the code using only the documentation as a guide.**

*   **For `struct`s and `enum`s:**
    *   A summary of the data structure's purpose.
    *   A detailed explanation of each field or variant, including its role, expected state, and any invariants.
    *   Example instantiation where applicable.

*   **For `fn`s:**
    *   A concise summary of what the function does.
    *   A `# Panics` section if the function can panic.
    *   An `# Errors` section detailing the conditions under which it will return an `Err` variant, and what the error means.
    *   A `# Safety` section for any `unsafe` code, explaining why it is safe.
    *   A detailed `# Examples` section with one or more runnable doctests.

### 13.4. Gating Factor for Milestones

No milestone will be considered complete until its associated code meets our documentation standard. This will be enforced by a `cargo doc` check.

**`cargo doc --no-deps --document-private-items --fail-on-warnings`**

This command will be run as part of the test suite for each milestone. It ensures that all items (including private ones, to encourage good internal documentation) are documented and that there are no broken links or other `rustdoc` warnings.

---

## 14. Future Directions

This section outlines potential features and enhancements that could be considered for future versions of `shutri`, beyond the core functionality described in this document.

### 14.1. Programmable Editing & Effects

While the initial version focuses on manual, precise editing, the text-based nature of the `.shutri` file opens up powerful possibilities for automation. Future versions could introduce features for "programmable editing," where the user can apply changes to multiple clips at once using scripts or commands.

Examples include:

*   **Automated Filler Word Removal:** A command to find and delete all clips that only contain "um," "uh," or other specified filler words.
*   **Silence Adjustment:** A function to automatically shorten or lengthen silences between clips to meet a specific duration.
*   **Applying Audio Effects:** The `.shutri` format could be extended to support applying SoX effects to specific clips.

### 14.2. Advanced Vim Integration

The Vim plugin could be enhanced with more sophisticated features, such as:

*   **Visual Highlighting:** Highlighting the currently playing clip or chunk in the Vim buffer.
*   **Multi-Clip Operations:** Allowing users to visually select multiple lines (clips) and perform actions on them, such as playing them in sequence or deleting them all at once.
*   **Speaker Identification:** If the transcription service provides speaker diarization, this information could be displayed in the `.shutri` file, allowing for speaker-specific edits.

### 14.3. Cross-Platform Support

While the initial version is focused on Debian-based Linux, future work could expand support to other operating systems. This would involve:
*   **macOS:** Creating a dedicated installation script using Homebrew.
*   **Windows:** Developing an installation method using a package manager like Chocolatey or Scoop.
*   **Testing:** Establishing a testing pipeline for each supported platform.
