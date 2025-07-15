# Shutri Project: Defect Tracker & Milestone Approvals

This document tracks primary defects and formal approvals for each milestone of the `shutri` project, in accordance with the Quality and Defect Management strategy outlined in `tech-specs.md`.

## Instructions

1.  **Milestone Approval:** Before starting a new milestone, the "Testable Outcome" for the previous milestone must be met and signed off below.
2.  **Defect Logging:** Any issue that qualifies as a "primary defect" must be logged in the table corresponding to the milestone in which it was *discovered*.

---

## Milestone 1: Project Setup and Core Data Structures

### Approval

*   **Testable Outcome:** The project compiles successfully. Unit tests for the data structures pass. The milestone's code passes the documentation standard check (`cargo doc --fail-on-warnings`).
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M1-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 2: Audio Import and Chunking

### Approval

*   **Testable Outcome:** Running `shutri --import <path/to/audio.mp3>` correctly creates a project with audio chunks in the `~/.shutri` directory. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M2-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 3: Mocked Transcription File Generation

### Approval

*   **Testable Outcome:** Running `shutri --transcribe --mock <project_name>` generates a correctly formatted `.shutri` file that is ready for interactive use. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M3-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 4: Vim Integration & Playback

### Approval

*   **Testable Outcome:** Running `shutri --edit <project_name>` opens Vim. Boundary-crossing clips are highlighted. `<Leader>p` and `<Leader>c` play the correct audio from the real audio file. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M4-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 5: Audio Export

### Approval

*   **Testable Outcome:** Running `shutri --export <project_name>` on a (mock or real) edited project generates a final MP3 file. The audio content matches the edits made. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M5-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 6: Real Transcription Service & Authentication

### Approval

*   **Testable Outcome:** Running `shutri --transcribe <project_name> --no-cache` populates the `.shutri` file with a real transcription, using either authentication method. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M6-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |

---

## Milestone 7: Polish and Finalize

### Approval

*   **Testable Outcome:** The application is fully functional, robust, and user-friendly. The end-to-end test suite passes. The milestone's code passes the documentation standard check.
*   **Status:** PENDING
*   **Approved By:** `(Your Name/Handle)`
*   **Date:** `(YYYY-MM-DD)`

### Defect Log

| Defect ID | Date Found   | Description                               | Status (Open/Closed) | Resolution / Commit Hash                               |
| :-------- | :----------- | :---------------------------------------- | :------------------- | :----------------------------------------------------- |
| M7-001    | `(YYYY-MM-DD)` | `(Detailed description of the defect)`      | Open                 | `(Link to the fixing commit or brief explanation)`       |
