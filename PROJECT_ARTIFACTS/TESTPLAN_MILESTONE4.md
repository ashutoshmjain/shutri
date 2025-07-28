# Test Plan: Milestone 4

This document outlines the steps to manually test the successful completion of Milestone 4: Vim Integration & Playback.

### Objective
The goal of this test is to verify that the `shutri --edit <project_name>` command successfully opens a project in Vim and that the core playback keybindings (`<Leader>p`, `<Leader>c`) function correctly, playing the appropriate audio by calling the Rust binary from Vim.

### Prerequisites

1.  A compiled `shutri` binary located at `./target/debug/shutri`.
2.  An existing `shutri` project that has been created and transcribed. If not, create one:
    ```bash
    cargo run -- import audio/short_clip.mp3
    cargo run -- transcribe short_clip --mock
    ```
3.  Vim or Neovim is installed on the system.
4.  A basic `shutri.vim` plugin file has been created and placed in the appropriate Vim configuration directory (e.g., `~/.config/nvim/plugin/shutri.vim` for Neovim or `~/.vim/plugin/shutri.vim` for Vim).

### Step 1: Run the Edit Command

From the root directory of the `shutri` project, execute the `--edit` (or `-v`) command, passing the name of your project.

```bash
cargo run -- --edit short_clip
```

### Step 2: Verify Vim Opens Correctly

1.  **Observe the terminal.** The command should launch Vim or Neovim.
2.  **Check the buffer content.** The editor should contain the content of the `~/.shutri/projects/short_clip/short_clip.shutri` file.
3.  **(Visual Check) Verify Syntax Highlighting.** The comment lines (e.g., `// --- CHUNK...`) and timestamps (`[MM:SS.ms]`) should be a different color from the main clip text, indicating that the Vim plugin's syntax rules are active.

### Step 3: Test Clip Playback (`<Leader>p`)

1.  **Position the cursor.** Move the cursor to any line containing a clip, for example:
    `[00:00.000] This is a mock transcription for split #1. [00:02.832]`
2.  **Press the keybinding.** In Vim's normal mode, press `<Leader>p`. (The Leader key is typically `\` by default).
3.  **Listen for audio.**
4.  **Expected Outcome:** You should hear the audio corresponding *only* to that specific clip. The playback should last for approximately the duration of the clip (in this case, about 2.8 seconds).

### Step 4: Test Original Chunk Playback (`<Leader>c`)

1.  **Position the cursor.** Keep the cursor anywhere within the same chunk as the clip you just tested.
2.  **Press the keybinding.** In Vim's normal mode, press `<Leader>c`.
3.  **Listen for audio.**
4.  **Expected Outcome:** You should hear the audio for the *entire* original chunk, as defined by the timestamps in the `// --- CHUNK ...` comment line. This playback will be longer than the single clip playback.

### Step 5: Test Stop Playback (`<Leader>s`)

1.  **Start playback.** Press `<Leader>c` to begin playing a chunk.
2.  **Press the stop keybinding.** While the audio is playing, press `<Leader>s`.
3.  **Listen for silence.**
4.  **Expected Outcome:** All audio playback should stop immediately.

### Conclusion

If the `edit` command opens Vim, the syntax is highlighted, and all playback keybindings (`<Leader>p`, `<Leader>c`, `<Leader>s`) control the audio as described, then the test is successful. This confirms that the interactive editing loop between Vim and the Rust binary is functioning correctly.
