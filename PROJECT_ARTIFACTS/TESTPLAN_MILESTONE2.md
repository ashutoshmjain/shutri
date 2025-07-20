# Test Plan: Milestone 2

This document outlines the steps to manually test the successful completion of Milestone 2: Audio Import and Splitting.

### Prerequisites

1.  A compiled `shutri` binary located at `./target/debug/shutri`.
2.  An MP3 audio file to use for testing. For this guide, we will assume the file is located at `~/Music/test_podcast.mp3`.

### Step 1: Run the Import Command

From the root directory of the `shutri` project, execute the `import` subcommand, passing the path to your MP3 file.

```bash
./target/debug/shutri import ~/Music/test_podcast.mp3
```

### Step 2: Observe the Command-Line Output

A successful run should produce output similar to the following. The exact messages regarding merging will vary based on the content of your audio file.

```
Creating project: test_podcast
Copied source file to "/home/amj/.shutri/imports/test_podcast.mp3"
Initial split successful. Now merging short splits...
Merged "split-002.mp3" into "split-001.mp3"
Merged "split-004.mp3" into "split-003.mp3"
Project 'test_podcast' created and processed successfully.
```

### Step 3: Verify the Project Directory and Files

After the command completes, verify that the project structure has been created correctly in your home directory.

Run the following command to inspect the contents of the `~/.shutri` directory:

```bash
ls -R ~/.shutri
```

The expected output should show the following structure:

1.  An `imports` directory containing a copy of your original audio file.
2.  A `projects` directory containing a sub-directory named after your audio file (e.g., `test_podcast`).
3.  A `splits` directory within your project folder, containing the final, processed `split-*.mp3` files.

**Example Directory Listing:**

```
/home/amj/.shutri:
imports/  projects/

/home/amj/.shutri/imports:
test_podcast.mp3

/home/amj/.shutri/projects:
test_podcast/

/home/amj/.shutri/projects/test_podcast:
splits/

/home/amj/.shutri/projects/test_podcast/splits:
split-001.mp3
split-003.mp3
split-005.mp3
```

### Conclusion

If the command runs without errors and the directory structure and files are created as described above, the test is successful. This confirms that the core functionality of Milestone 2 is working as specified in the technical documentation.
