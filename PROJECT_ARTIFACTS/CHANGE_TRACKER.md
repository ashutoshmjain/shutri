# Change Tracker

This document tracks significant changes in requirements that necessitate re-work of previously completed milestones.

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
