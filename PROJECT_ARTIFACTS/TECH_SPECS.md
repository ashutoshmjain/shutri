# `shutri` - Technical Specification

## 1. Overview

### 1.1. Core Concept

`shutri` is a Rust application for text-based audio editing, integrating the audio processing power of SoX (Sound eXchange) with the efficiency of the Vim editor. The core concept is to transform the traditionally cumbersome process of waveform manipulation into a precise, text-driven workflow.

The initial version of `shutri` is specifically targeted for **Debian-based Linux distributions** (e.g., Debian, Ubuntu). All development and testing will be focused on this platform.

The primary workflow consists of four main stages:

1.  **Import:** A user-provided MP3 audio file is imported. The audio is first broken down into machine-generated `SPLITS` based on silence detection.
2.  **Transcribe:** Each `SPLIT` is transcribed into a `CLIP` (a text snippet with a precise timestamp).
3.  **Edit:** The `CLIPS` are presented to the user in a structured text file inside Vim, where they are visually grouped into `CHUNKS` for easy navigation. The user edits the text and timestamps.
4.  **Export:** The edited `CLIPS` are used to extract the corresponding audio segments from the original file, which are then concatenated into a new, final MP3 file.

### 1.2. Key Terminology

To understand the `shutri` workflow, it's essential to be clear on the following terms, which represent the three-stage data transformation from raw audio to an editable text file:

1.  **`SPLIT`**: This is the most granular, machine-generated audio segment.
    *   **Purpose**: To break the original audio file into smaller pieces at every point of natural silence. This is a raw processing step.
    *   **Creation**: A `SPLIT` is created by SoX, which cuts the audio wherever there's a silence of 0.6 seconds or more. Very short splits are then merged until all are at least 6 seconds long.
    *   **End Result**: A manifest of many small, sequential audio files (`SPLITS`) with precise start and end times, ready for transcription.

2.  **`CLIP`**: This is the fundamental unit of editable content, pairing text with audio.
    *   **Purpose**: To represent a piece of transcribed text and its exact location in the original audio.
    *   **Creation**: Each `SPLIT` is sent to the Gemini API for transcription. The resulting text, combined with the `SPLIT`'s start/end times, becomes a `CLIP`.
    *   **End Result**: A list of `[start_time] text [end_time]` entries. These are the lines you directly edit in Vim.

3.  **`CHUNK`**: This is a logical, visual grouping of `CLIPS` for the user.
    *   **Purpose**: To structure the long list of `CLIPS` in the editor, making the project file easier to navigate. It acts like a chapter or section heading.
    *   **Creation**: `CLIPS` are grouped together into `CHUNKS` that are roughly 60 seconds long.
    *   **End Result**: A comment line in the `.shutri` file (e.g., `// --- CHUNK 1 (00:00.000 - 01:00.000) ---`) that visually separates groups of `CLIPS`.

**Workflow Summary:**

**Original MP3** -> is broken into many -> **`SPLITS`** (raw audio at silences) -> each is transcribed into a -> **`CLIP`** (text + timestamp) -> which are then grouped into -> **`CHUNKS`** (for display in Vim).

### 1.3. The Problem with Waveform-Based Editing

Traditional Digital Audio Workstations (DAWs) and audio editors rely on a visual representation of the audio waveform. This approach has several drawbacks, especially for long-form content like podcasts, interviews, or lectures:

*   **Difficult Navigation:** Navigating long recordings using a waveform is often imprecise and slow.
*   **Inflexible Markers:** Markers or regions are often cumbersome to manage and lack the flexibility of text-based search and manipulation.
*   **High Cognitive Load:** The waveform is a graphical abstraction that requires visual interpretation but does not directly represent the spoken content, which is the primary focus of the editing task.

### 1.4. The Opportunity: Text-Based Editing

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
*   `~/.shutri/imports/`: Stores the original audio files and their corresponding `SPLITS`.
*   `~/.shutri/exports/`: The output directory for final, rendered audio files.
*   `~/.shutri/cache/`: Stores transcription results to avoid redundant API calls.

#### 4.1.1. Project Files (`.shutri`)

A `.shutri` file is a text file that represents the state of an editing project. To make navigation easier, the file is visually structured into `CHUNKS`.

Lines starting with `//` are treated as comments. They are used to delineate the `CHUNKS` and to provide informational notes to the user. The `CHUNK`-level timestamps in these comments are for reference only and should not be edited.

Each editable line corresponds to an audio `CLIP` and follows this format:
`[start_time] text [end_time]`

*   **`start_time` / `end_time`:** Time-stamps in `MM:SS.ms` format (e.g., `00:01.234`). **Important:** All timestamps are absolute, relative to the beginning of the original audio file.
*   **`text`:** The transcribed text for the `CLIP`.

**Example Vim Interface (`.shutri` file):**

```vim
" Project: podcast_episode_1.mp3
"
" Keybindings:
"  <Leader>p : Play current clip (preview your edit)
"  <Leader>c : Play original chunk (hear the 'before')
"  <Leader>C : Play edited chunk (hear the 'after')
"  <Leader>s : Stop all playback
"  <Leader>[ : Nudge start time of current clip earlier
"  <Leader>] : Nudge start time of current clip later
"  <Leader>{ : Nudge end time of current clip earlier
"  <Leader>} : Nudge end time of current clip later
" =============================================================================

// --- CHUNK 1 (00:00.000 - 00:30.000) ---
[00:01.123] This is a valid clip. [00:05.450]

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

  2.  **Iterative Merging of Short Splits:** The manifest is repeatedly scanned to merge any `SPLIT` shorter than **6 seconds** until none remain. The merging logic handles all cases, including the first and last splits in the project:
      *   **Middle Split:** A short split with two neighbors is merged with its shorter neighbor. In case of a tie, it is merged with the *previous* split.
      *   **First Split:** A short split at the beginning of the project is merged with the *next* split.
      *   **Last Split:** A short split at the end of the project is merged with the *previous* split.
  The manifest is updated after each merge, and the process repeats until all splits are at least 6 seconds long.

  **Phase 2: Transcription (Creating Clips)**

  3.  **Transcribe and Associate Timestamps:** Each `SPLIT` from the final manifest is sent to the Gemini API. The returned text becomes a `CLIP` and is paired with the accurate `startTime` and `endTime` from the manifest.

  **Phase 3: Structuring for Presentation (Creating Chunks)**

  4.  **Chunking Algorithm:** The `CLIPS` are grouped into logical `CHUNKS` for the user interface based on two primary rules:
      *   **Large Split Override:** If a `CLIP`'s source `SPLIT` is **longer than 60 seconds**, it becomes its own `CHUNK`.
      *   **Greedy Grouping:** Otherwise, `CLIPS` are added to the current `CHUNK` until its total duration approaches **60 seconds**. The next `CLIP` that would exceed the limit starts a new `CHUNK`.

  This strategy ensures accurate timestamp management and provides clear rules for handling all audio structuring scenarios.


4.  A preliminary `.shutri` project file is created, containing only comment lines that define the `CHUNK` boundaries (e.g., `// --- CHUNK 1 (00:00.000 - 00:28.530) ---`). This file contains no editable `CLIPS` yet.

**Pseudocode (Rust):**

```rust
mod audio {
    fn import_audio(file_path: &Path) -> Result<Project, Error> {
        // 1. Validate file is in MP3 format.
        // 2. Create a new project directory in `~/.shutri/projects/`
        // 3. Copy the original file to `~/.shutri/imports/`
        // 4. Use SoX to create audio SPLITS based on silence.
        //    - `sox <input.mp3> <output_split.mp3> silence 1 0.1 1% 1 0.6 1% : newfile : restart`
        //    - This command tells SoX to create a new file every time it detects
        //      at least 0.6 seconds of silence at 1% volume threshold.
        // 5. Follow the three-phase strategy (Splits -> Clips -> Chunks) described above.
        // 6. Return a new `Project` struct
    }
}
```

#### 4.2.2. Transcribe (`shutri transcribe <project_name>`)

As of Milestone 3, this command generates a **mock** transcription file. This is a placeholder to allow for the development and testing of the editing workflow (`shutri -v`) before the real transcription service is implemented in Milestone 6.

1.  `shutri` generates mock text for each audio `SPLIT`.
2.  The existing `.shutri` project file is updated with the mock `CLIPS`, inserted under their corresponding `CHUNK` markers.
3.  If a `.shutri` file already exists, the command will prompt the user for confirmation before overwriting it. This interactive safeguard prevents accidental data loss.
4.  The `--mock` flag, which enables this functionality, is only available in debug builds of the application.

**Note:** The final version of this command (Milestone 6) will send each audio `SPLIT` to the Gemini API for real transcription and cache the results.

**Pseudocode (Rust - for Milestone 3):**

```rust
mod transcription {
    fn transcribe_project_mock(project: &mut Project) -> Result<(), Error> {
        // 1. Check if .shutri file exists.
        // 2. If it exists, prompt the user for overwrite confirmation.
        // 3. If the user does not confirm, abort the operation.
        // 4. For each audio SPLIT in the project:
        // 5.   - Generate a mock transcription string (e.g., "This is a mock transcription...").
        // 6.   - Create a CLIP by pairing the mock text with the SPLIT's timestamp.
        // 7. Update the project's data structure with the list of CLIPS.
        // 8. Write the `Project` data (now including CLIPS) to the `.shutri` file,
        //    placing the CLIPS under their appropriate CHUNK markers.
    }
}
```

#### 4.2.3. Edit (`shutri -v <file>`)

1.  `shutri` invokes Vim, opening the `.shutri` project file.
2.  The user edits the file, adjusting `CLIP` time-stamps, deleting lines (which deletes the `CLIP`), or adding personal comments.
3.  The Vim plugin provides keybindings for an enhanced editing workflow. The core audio processing logic for playback resides in the Rust binary, while the Vimscript functions act as wrappers that gather context from the editor and call the binary with the appropriate arguments.

    *   **Playback Controls (Before & After):**
        *   `<Leader>c`: **Play Original Chunk (Before)**. Plays the original, unmodified audio for the entire `CHUNK`, providing context for your edits.
        *   `<Leader>C`: **Play Edited Chunk (After)**. Plays the sequence of edited `CLIPS` within the current `CHUNK` to preview the final result.
        *   `<Leader>p`: **Play Clip**. Plays the audio segment for the current line to preview a single edit.
        *   `<Leader>s`: Stop all playback.
    *   **Timestamp Nudging:**
        *   `<Leader>[`, `<Leader>]`: Nudge the start time of the current `CLIP` earlier or later.
        *   `<Leader>{`, `<Leader>}`: Nudge the end time of the current `CLIP` earlier or later.

**Vimscript and Rust Interaction:**

The Vimscript functions are lightweight. Their job is to gather context directly from the editor's live buffer (e.g., the current line, the lines belonging to the current chunk) and make a system call to the `shutri` binary. This is a crucial design point: the user does **not** need to save the file to preview their edits. The Rust application contains all the core logic for parsing this context received from the command line and using SoX for playback.

For example, the playback functions would be implemented in Vimscript as follows:
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

" Identifies the current chunk's original timestamps and calls the binary.
function! ShutriPlayOriginalChunk()
    " This function would need logic to find the current chunk's comment line
    " (e.g., `// --- CHUNK 1 (00:00.000 - 00:30.000) ---`) and pass the
    " timestamps to `shutri --play-original-chunk`.
endfunction

function! ShutriStopPlayback()
    call system('shutri --stop-playback')
endfunction
```

The corresponding logic in Rust would parse the arguments (`--play-clip`, `--play-edited-chunk`, `--play-original-chunk`), extract the necessary timestamps or clip data, and then use SoX to play the specific audio segment(s).

**Vimscript (for the plugin):**

```vim
" Play the current clip (previews the single edit)
nnoremap <Leader>p :call ShutriPlayClip()<CR>

" Play the original chunk (provides 'before' context)
nnoremap <Leader>c :call ShutriPlayOriginalChunk()<CR>

" Play the edited chunk (previews the 'after' result)
nnoremap <Leader>C :call ShutriPlayEditedChunk()<CR>

" Stop all playback
nnoremap <Leader>s :call ShutriStopPlayback()<CR>
```

#### 4.2.4. Export (`shutri -e <file>`)

The export process is designed to be as efficient and non-destructive as possible, minimizing audio re-encoding to preserve quality. It achieves this by intelligently identifying which parts of the audio have actually been changed.

1.  **State Comparison:** `shutri` first compares the current state of the `.shutri` project file against an original, unmodified manifest of clips that was created during the initial import.

2.  **Conditional Logic:**
    *   **If No Changes Are Detected:** If the user has not altered any clip's timestamps or deleted any clips, the process is simple and fast. The original imported MP3 file is copied directly to the `~/.shutri/exports/` directory. This is the ideal "least destructive" path, as it involves no re-encoding and preserves the original audio quality perfectly.
    *   **If Changes Are Detected:** If any part of the project has been edited, the "Optimized Export" process begins.

3.  **Optimized Export Process:**
    *   **A. Identify Changed Regions:** The application identifies all contiguous blocks of edited, deleted, or reordered clips.
    *   **B. Extract in Large Chunks:** Instead of processing every clip individually, `shutri` extracts the audio in large, logical chunks:
        *   **Unchanged Blocks:** Any long, continuous sequence of *unmodified* clips is extracted from the original file as a single, large segment. These segments are not re-encoded.
        *   **Changed Blocks:** The edited clips are extracted and processed.
    *   **C. Join and Crossfade:** The final audio file is constructed by joining these large chunks together. A **5-10 millisecond crossfade** is applied *only at the seams* between the chunks to ensure a smooth, professional transition without clicks or pops. This dramatically reduces the number of re-encoding operations compared to processing every single clip.

4.  **Final Output:** The final, combined audio is saved to the `~/.shutri/exports/` directory. To support multiple exports from the same project, the output file will be named using the convention: `<project_name>_export_YYYYMMDD-HHMMSS.mp3`.

**Pseudocode (Rust):**

```rust
mod audio {
    fn export_project(project: &Project) -> Result<(), Error> {
        // 1. Load the original clip manifest.
        // 2. Load the current clip state from the .shutri file.
        // 3. Compare the two. If they are identical, copy the original import
        //    file to the exports directory and return.
        // 4. If different, identify all contiguous blocks of changed and
        //    unchanged clips.
        // 5. For each block, extract it from the original audio into a temporary file.
        //    - For unchanged blocks, this is a simple, fast `sox trim` command.
        //    - For changed blocks, this may involve multiple trims and joins.
        // 6. Create a final list of temporary audio chunks to be joined.
        // 7. Iteratively join the chunks using `sox splice` with a small crossfade
        //    at each seam.
        // 8. Generate a timestamp string (e.g., "20250719-103000").
        // 9. Construct the final output path: `~/.shutri/exports/{project_name}_export_{timestamp}.mp3`.
        // 10. Move the final combined audio to the output path.
        // 11. Clean up all temporary chunk files.
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

*   **Status Updates:** Display simple, human-readable messages for each major step (e.g., "Importing audio...", "Splitting into 3 parts...", "Transcribing part 1 of 3...").
*   **Engaging Feedback:** During the transcription phase, which can be time-consuming, the CLI should display a series of engaging, humorous, or informative messages to keep the user entertained and aware that the process is still running. This is similar to the experience provided by modern interactive CLIs.

### 5.2. Command-Line Options

*   `shutri -i, --import <file.mp3>`: Import an audio file.
*   `shutri -t, --transcribe <project>`: Transcribe an imported project. As of Milestone 3, this generates a **mock** transcription. In later milestones, it will generate a real transcription using the Gemini API. If a transcription file already exists, it will prompt for confirmation before overwriting.
*   `shutri -e, --export <project>`: Export a project to a final audio file.
*   `shutri -v, --edit <project>`: Open a project in Vim for editing.
*   `shutri auth login`: Initiates an interactive OAuth 2.0 flow to sign in with a Google account.
*   `--mock`: Used with the `transcribe` command to generate mock data. This flag is only available in debug builds.
*   `--no-cache`: Used with a transcription command. Forces re-transcription of all audio `SPLITS`, ignoring any cached results. This is useful if the initial transcription is unsatisfactory.
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

The `install.sh` script will create the following file at `~/.config/shutri/config.toml`. The comments are designed to guide the user on how to configure the application.

```toml
# This is the configuration file for shutri.
#
# --- Authentication ---
# You must choose ONE of the two authentication methods below.

# --- Method 1: API Key (for developers) ---
# To use this method, uncomment the line below and replace "YOUR_API_KEY_HERE" with your key.
# api_key = "YOUR_API_KEY_HERE"

# --- Method 2: Sign in with Google (recommended for most users) ---
# To use this method, simply run the command `shutri auth login`.
# The application will automatically manage and fill in the token details below.
# Do not edit this section manually.
# [oauth_token]
# access_token = "..."
# refresh_token = "..."
# expires_in = "..."

# --- General Settings ---
# These are the default settings. You can modify them if needed.

# The command to invoke your preferred editor (e.g., "vim", "nvim").
editor = "vim"

# Audio processing settings.
# You can tune these values for different types of audio.
[audio]
# Minimum silence duration in seconds to consider for splitting audio.
# A podcast with fast speakers might need a smaller value (e.g., 0.4).
# A slow lecture might benefit from a larger value (e.g., 1.0).
min_silence_duration_secs = 0.6 # Default: 0.6

# Target length of audio chunks in seconds after merging short splits.
# You might prefer shorter chunks (e.g., 30.0) for more granular review,
# or longer chunks (e.g., 90.0) for a higher-level overview.
target_chunk_length_secs = 60.0 # Default: 60.0
```

---

## 9. Testing Strategy

The testing strategy will include:

*   **Unit Tests:** For individual functions in each module (e.g., parsing time-stamps, validating project files).
*   **Integration Tests:** For workflows that involve multiple modules (e.g., the full import-transcribe-export process). These tests will use mock objects for the Gemini API to avoid making real network requests.
*   **End-to-End Tests:** A suite of shell scripts that will test the `shutri` CLI from the user's perspective, using small, sample MP3 files. All end-to-end tests will be conducted on a reference Debian-based Linux environment to ensure stability and correctness on the target platform.

---

## 10. Development Methodology

For details on the project's development methodology, quality assurance, and documentation standards, please see [VIBE_METHODOLOGY.md](./VIBE_METHODOLOGY.md).

---

## 11. Installation

To ensure a smooth setup, `shutri` will be distributed with an installation script (`install.sh`). **This script is designed specifically for Debian-based Linux distributions (e.g., Debian, Ubuntu).**

### 11.1. Installation Script (`install.sh`)

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

### 11.2. Uninstallation

An `uninstall.sh` script will also be provided to remove the `shutri` binary, Vim plugin, and configuration files cleanly.

---

## 12. Future Directions

This section outlines potential features and enhancements that could be considered for future versions of `shutri`, beyond the core functionality described in this document.

### 12.1. Programmable Editing & Effects

While the initial version focuses on manual, precise editing, the text-based nature of the `.shutri` file opens up powerful possibilities for automation. Future versions could introduce features for "programmable editing," where the user can apply changes to multiple clips at once using scripts or commands.

Examples include:

*   **Automated Filler Word Removal:** A command to find and delete all clips that only contain "um," "uh," or other specified filler words.
*   **Silence Adjustment:** A function to automatically shorten or lengthen silences between clips to meet a specific duration.
*   **Applying Audio Effects:** The `.shutri` format could be extended to support applying SoX effects to specific clips.

### 12.2. Advanced Vim Integration

The Vim plugin could be enhanced with more sophisticated features, such as:

*   **Visual Highlighting:** Highlighting the currently playing clip or chunk in the Vim buffer.
*   **Multi-Clip Operations:** Allowing users to visually select multiple lines (clips) and perform actions on them, such as playing them in sequence or deleting them all at once.
*   **Speaker Identification:** If the transcription service provides speaker diarization, this information could be displayed in the `.shutri` file, allowing for speaker-specific edits.

### 12.3. Cross-Platform Support

While the initial version is focused on Debian-based Linux, future work could expand support to other operating systems. This would involve:
*   **macOS:** Creating a dedicated installation script using Homebrew.
*   **Windows:** Developing an installation method using a package manager like Chocolatey or Scoop.
*   **Testing:** Establishing a testing pipeline for each supported platform.