# A Guide to the VIBE Development Methodology

## 1. Introduction: What is VIBE?

Welcome to the VIBE (Value-Integrated, Iterative, Backlog-Enabled) Development Methodology. This document is a guide for project managers and developers who want to build software in close collaboration with a Large Language Model (LLM) assistant, such as the **Gemini CLI**.

VIBE is a hybrid approach that combines the rigor of milestone-driven development with the flexibility of Agile and Scrum. It's designed to maximize the unique capabilities of an LLM, turning it from a simple coding assistant into the primary driver of the development process.

Throughout this guide, we will use the [`shutri`](../) project as a real-world case study. You can explore its artifacts to see VIBE in action.

---

## 2. The Core Philosophy: Your LLM as the Developer

The VIBE methodology is built on a fundamental shift in perspective: **the LLM is the developer**. The human's role is to act as the **Project Manager** or **Product Owner**—setting the vision, defining the goals, and verifying the results.

*   **Agent-Driven Development:** The LLM writes the code, runs the tests, and manages the project artifacts.
*   **Self-Documentation:** The LLM is responsible for creating and maintaining all project documentation, ensuring it is always synchronized with the code.
*   **Integrated Version Control:** All `git` activities—from writing detailed commit messages to staging and committing files—are performed by the LLM.

This approach creates a powerful, self-documenting, and highly-auditable development process.

---

## 3. How to Implement VIBE: A Step-by-Step Guide

### Step 1: Lay the Foundation with a Technical Specification

Before writing a single line of code, your first task is to collaborate with your LLM assistant to create a detailed **Technical Specification**. This document is the blueprint for your project.

**Your Goal:** To produce a comprehensive `TECH_SPECS.md` file.

**How to do it:**
1.  **Start a conversation** with your LLM. Describe your project's vision, its core features, the target audience, and any known constraints.
2.  **Ask the LLM to propose a technical solution.** This should include system architecture, key dependencies, data structures, and the core user workflow.
3.  **Iterate and refine.** Work with the LLM to flesh out the details. Challenge its assumptions. Ask for alternative approaches. The goal is to explore the problem space thoroughly *before* committing to a specific implementation.

> **Case Study: `shutri`**
> The technical specification for the `shutri` project was built through this exact iterative process. The result is a detailed document that covers everything from the system architecture to the command-line options.
>
> **See the artifact:** [`TECH_SPECS.md`](./TECH_SPECS.md)

### Step 2: Define the "Ways of Working"

Once the technical vision is clear, the next step is to define the development process itself. This is where you create the `VIBE_METHODOLOGY.md` document (the very guide you are reading now).

**Your Goal:** To establish a clear, documented process for the project.

**How to do it:**
1.  **Ask the LLM** to create a project plan based on the `TECH_SPECS.md`.
2.  **Instruct it to break the plan into phases and milestones.** Each milestone should have a clear, testable goal. This forms the "Development Plan" section.
3.  **Define the quality and documentation standards.** Ask the LLM to establish rules for defect tracking, change management, and code documentation. This ensures consistency and quality from the start.

> #### **Case Study: The `shutri` Development Plan**
>
> The following is the complete, multi-phase development plan created for the `shutri` project. It serves as a concrete example of the output of this step.
>
> ##### **Phase 1: Prototype**
> This phase focuses on building the core functionality of `shutri` and validating the text-based audio editing concept.
>
> *   **Milestone 1: Project Setup and Core Data Structures**
>     *   **Goal:** Initialize the Rust project and define the core data structures.
>     *   **Testable Outcome:** The project compiles successfully. Unit tests for the data structures pass.
> *   **Milestone 2: Audio Import and Splitting**
>     *   **Goal:** Import an MP3 file and split it into `SPLITS` based on silence.
>     *   **Testable Outcome:** Running `shutri --import <file>` creates a project with audio `SPLITS` in the `~/.shutri` directory.
> *   **Milestone 3: Transcription File Generation**
>     *   **Goal:** Generate a `.shutri` project file with mock data.
>     *   **Testable Outcome:** Running `shutri --transcribe <project> --mock` generates a correctly formatted `.shutri` file.
> *   **Milestone 4: Vim Integration & Playback**
>     *   **Goal:** Create the core interactive editing loop within Vim.
>     *   **Testable Outcome:** Running `shutri --edit <project>` opens Vim; `<Leader>p` and `<Leader>c` play the correct audio.
> *   **Milestone 5: Audio Export**
>     *   **Goal:** Combine edited `CLIPS` into a final MP3 file.
>     *   **Testable Outcome:** Running `shutri --export <project>` generates a final MP3 file matching the edits.
> *   **Milestone 6: Real Transcription Service & Authentication**
>     *   **Goal:** Implement the real transcription service and authentication.
>     *   **Testable Outcome:** Running `shutri --transcribe <project>` populates the `.shutri` file with a real transcription.
> *   **Milestone 7: Polish and Finalize**
>     *   **Goal:** Finalize the CLI, implement robust error handling, and improve the user experience.
>     *   **Testable Outcome:** The application is fully functional, robust, and user-friendly.
>
> ##### **Phase 2: Dev**
> This phase focuses on expanding features, stability, and reach.
>
> *   **Milestone 8: Alpha Release & Community Feedback**
>     *   **Goal:** Package the application for an alpha release and gather initial feedback.
>     *   **Testable Outcome:** At least 10 users have successfully installed and used `shutri`.
> *   **Milestone 9: Feature Complete & API Stability**
>     *   **Goal:** Implement "Future Directions" features and stabilize the internal API.
>     *   **Testable Outcome:** New features are covered by integration tests.
> *   **Milestone 10: Cross-Platform Support**
>     *   **Goal:** Add support for macOS and Windows.
>     *   **Testable Outcome:** The application can be successfully installed and run on all three target platforms.
>
> ##### **Phase 3: User**
> This phase focuses on preparing the application for a wider, non-technical audience.
>
> *   **Milestone 11: Beta Release & User Acceptance Testing (UAT)**
>     *   **Goal:** A wider public beta to gather feedback from non-technical users.
>     *   **Testable Outcome:** The application is successfully used by at least 50 beta testers.
> *   **Milestone 12: General Availability (GA) & Long-Term Support (LTS)**
>     *   **Goal:** The official 1.0 release with a commitment to long-term support.
>     *   **Testable Outcome:** The `v1.0.0` release is published with a public issue tracker.

### Step 3: Execute the Plan, Milestone by Milestone

This is the core loop of the VIBE methodology. You will treat each milestone as a self-contained "sprint."

**Your Goal:** To complete the "Testable Outcome" for the current milestone.

**How to do it:**
1.  **Create the Test Plan:** For each new milestone, your first action is to ask the LLM to create a dedicated test plan. This document translates the milestone's goal into a concrete, verifiable checklist.
2.  **Instruct the LLM to Begin Development:** With the test plan as its guide, instruct the LLM to start working on the milestone's tasks.
3.  **Verify the Outcome:** Once the LLM reports that the work is complete, your job is to verify it against the test plan. Run the commands, check the outputs, and ensure the goal has been met.
4.  **Commit the Work:** If the outcome is successful, instruct the LLM to commit the changes with a descriptive message. The milestone is now complete.

> **Case Study: `shutri`**
> For Milestone 2, the goal was to import and split an audio file. The Gemini CLI first created a detailed test plan. It then implemented the feature and, once verified, the changes were committed.
>
> **See the artifacts:**
> *   [`TESTPLAN_MILESTONE2.md`](./TESTPLAN_MILESTONE2.md)
> *   [`TESTPLAN_MILESTONE3.md`](./TESTPLAN_MILESTONE3.md)

### Step 4: Manage Change and Quality with Trackers

No plan is perfect. As the project evolves, you will encounter bugs or realize that an enhancement is needed. VIBE manages this through two key documents.

*   **`DEFECT_TRACKER.md`**: Used to log any bug that breaks the functionality of a *previously completed* milestone.
*   **`CHANGE_TRACKER.md`**: Used to log any proposed enhancement or change that would alter the scope of a *previously completed* milestone.

**How to do it:**
1.  **Identify a Defect or Change:** When you notice an issue, describe it to the LLM.
2.  **Ask the LLM to Log It:** Instruct the LLM to add an entry to the appropriate tracker. This creates a formal record.
3.  **Prioritize and Address:** You can then instruct the LLM to address the item immediately or leave it in the tracker to be prioritized later.

> **Case Study: `shutri`**
> These trackers are used to maintain the quality and integrity of the project's approved milestones.
>
> **See the artifacts:**
> *   [`DEFECT_TRACKER.md`](./DEFECT_TRACKER.md)
> *   [`CHANGE_TRACKER.md`](./CHANGE_TRACKER.md)

---

## 4. Measuring Success: Defect Density

A core principle of VIBE is that quality should improve over time. The primary metric for measuring this is **Defect Density**.

### What is Defect Density?

In the context of VIBE, Defect Density is not measured against lines of code. It is a measure of process stability, calculated as:

**Defect Density = (Number of Primary Defects) / (Number of Completed Milestones)**

A "Primary Defect" is any bug logged in the `DEFECT_TRACKER.md` that represents a regression in a previously completed and verified milestone.

### The Goal: A 75% Reduction Per Phase

The goal of the VIBE methodology is to **reduce Defect Density by 75% in each successive development phase.**

*   **Phase 1 (Prototype):** This phase establishes the baseline Defect Density. The focus is on speed and validating the core concept, so a higher number of defects is expected.
*   **Phase 2 (Dev):** The goal is to achieve a Defect Density that is 75% lower than the baseline. This demonstrates that the core architecture is stabilizing and the development process is maturing.
*   **Phase 3 (User):** The goal is another 75% reduction from the Dev phase's density. This proves that the application is hardening and becoming robust enough for a general release.

Because the LLM assistant maintains the `DEFECT_TRACKER.md`, calculating this metric is a straightforward, data-driven process. It is the Project Manager's responsibility to periodically review this metric to ensure the project's quality is improving as expected.

---

## 5. Conclusion

The VIBE methodology provides a structured, transparent, and highly efficient framework for building software with an LLM assistant. By casting the human as the project manager and the LLM as the developer, it leverages the strengths of both to create high-quality, well-documented, and rapidly-developed projects.
