# Test Plan: Milestone 3

This document outlines the steps to manually test the successful completion of Milestone 3: Mocked Transcription File Generation.

### Objective
The goal of this test is to verify that the `shutri transcribe --mock <project_name>` command correctly generates a `.shutri` project file with valid, mock data corresponding to a real audio project created in Milestone 2.

### Prerequisites

1.  A compiled `shutri` binary located at `./target/debug/shutri`.
2.  An existing `shutri` project that has been created using the `import` command. This test plan assumes a project named `short_clip` has been created from `audio/short_clip.mp3`.

### Step 1: Ensure a Project Exists

If you have not already done so, create a project to test against.

```bash
./target/debug/shutri import audio/short_clip.mp3
```

This will create the necessary project structure at `~/.shutri/projects/short_clip/`.

### Step 2: Run the Mock Transcription Command

From the root directory of the `shutri` project, execute the `transcribe` subcommand with the `--mock` flag, passing the name of your project.

```bash
./target/debug/shutri transcribe --mock short_clip
```

### Step 3: Observe the Command-Line Output

A successful run should produce the following output, confirming that the file was generated:

```
Generating mock transcription for project: short_clip
Successfully generated mock transcription at "/home/amj/.shutri/projects/short_clip/short_clip.shutri"
```

### Step 4: Verify Overwrite Safeguard

This step ensures that the application does not accidentally overwrite user edits.

1.  **Attempt to run the command again without the `--force` flag.**

    ```bash
    ./target/debug/shutri transcribe --mock short_clip
    ```

2.  **Observe the command-line output.** The command should fail with an error message indicating that the file already exists.

    ```
    Error: Project file already exists at "/home/amj/.shutri/projects/short_clip/short_clip.shutri". Use --force to overwrite.
    ```

3.  **Run the command again with the `--force` flag.**

    ```bash
    ./target/debug/shutri transcribe --mock short_clip --force
    ```

4.  **Observe the command-line output.** The command should now succeed and generate the file.

    ```
    Generating mock transcription for project: short_clip
    Successfully generated mock transcription at "/home/amj/.shutri/projects/short_clip/short_clip.shutri"
    ```

### Step 5: Verify the Project File Contents

After the command completes, inspect the contents of the newly created `.shutri` file.

```bash
cat ~/.shutri/projects/short_clip/short_clip.shutri
```

The expected output should be a well-formatted text file containing the following elements:

1.  A header comment with the project name.
2.  A complete list of keybindings as specified in the technical documentation.
3.  At least one `CHUNK` marker, formatted as `// --- CHUNK N (MM:SS.ms - MM:SS.ms) ---`.
4.  Under each chunk, one or more `CLIP` lines.
5.  Each `CLIP` line must follow the format `[MM:SS.ms] Mock text... [MM:SS.ms]`.
6.  Timestamps should be sequential and correctly formatted.

**Example File Content:**
```vim
" Project: short_clip.mp3
"
" Keybindings:
"   <Leader>p : Play current clip (preview your edit)
"   <Leader>c : Play original chunk (hear the 'before')
"   <Leader>C : Play edited chunk (hear the 'after')
"   <Leader>s : Stop all playback
"   <Leader>[ : Nudge start time of current clip earlier
"   <Leader>] : Nudge start time of current clip later
"   <Leader>{ : Nudge end time of current clip earlier
"   <Leader>} : Nudge end time of current clip later
" =============================================================================

// --- CHUNK 1 (00:00.000 - 00:02.832) ---
[00:00.000] This is a mock transcription for split #1. [00:02.832]

```
*(Note: The exact number of chunks, clips, and timestamps will vary based on the audio file used for the import.)*

### Conclusion

If all commands run as described, producing the correct output and error messages, the test is successful. This confirms that the core functionality of Milestone 3 and its overwrite safeguard are working as specified.
