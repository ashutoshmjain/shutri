# Change Tracker

This document tracks significant changes in requirements that necessitate re-work of previously completed milestones.

---

## 2025-07-27: Refinement of Transcription Command and Overwrite Handling

*   **Impacted Milestone(s):**
    *   **Milestone 3: Mocked Transcription File Generation**

*   **Reason for Change:**
    *   The previous implementation of the `transcribe` command used a `--force` flag to handle overwriting existing files. This was deemed too destructive and not user-friendly. A more interactive approach was needed. Additionally, the `--mock` flag's purpose was clarified to be for debugging and testing only, not for production use.

*   **Summary of Change:**
    *   **Previous Strategy:** The `transcribe` command included a `--force` flag to overwrite existing transcription files. The `--mock` flag was a standard argument.
    *   **New Strategy:**
        *   The `--force` flag has been completely removed.
        *   The application now interactively prompts the user for confirmation (`[y/N]`) if they attempt to transcribe a project that already has a transcription file. This prevents accidental data loss.
        *   The `--mock` flag has been restricted to debug builds only (`#[cfg(debug_assertions)]`). This ensures that mock data generation is only used for development and testing, not in a production release.
    *   **Outcome:** The user experience is safer and more intuitive. The CLI is cleaner and less prone to user error. The `TESTPLAN_MILESTONE3.md` and `TECH_SPECS.md` documents were updated to reflect these changes. The implementation in `cli.rs`, `main.rs`, and `transcription.rs` was updated accordingly.

---

## 2025-07-16: Revision of Audio Processing and Data Strategy

*   **Impacted Milestone(s):**
    *   **Milestone 1: Project Setup and Core Data Structures**

*   **Reason for Change:**
    *   A detailed review of the initial "Chunking Strategy" revealed several potential issues and edge cases. The original strategy was not robust enough to handle timestamp accuracy, the merging of very short audio segments, or the grouping of segments into logical chunks for the user interface. This would have led to significant problems in later milestones.

*   **Summary of Change:**
    *   **Previous Strategy:** A two-phase approach that involved aggressively splitting audio by silence and then merging the resulting segments into 30-second "chunks". This model was too simple and lacked a clear definition for handling the data.
    *   **New Strategy:** A more robust, three-phase strategy was defined:
        1.  **Phase 1 (Splits):** Audio is split by silence (0.6s) into `SPLITS`. A manifest is created to track precise timestamps. Short splits (<6s) are iteratively merged.
        2.  **Phase 2 (Clips):** The final `SPLITS` are transcribed into `CLIPS`, inheriting their precise timestamps.
        3.  **Phase 3 (Chunks):** `CLIPS` are grouped into logical `CHUNKS` (~60s) for presentation, with special handling for very long splits.
    *   **Outcome:** This fundamental change to the data model and processing logic required re-implementing the core data structures defined in Milestone 1 from scratch.
