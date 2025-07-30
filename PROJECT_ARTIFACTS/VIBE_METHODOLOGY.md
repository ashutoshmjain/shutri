# vibeD Development Methodology

## 1. Introduction: What is vibeD?

Welcome to the vibeD Development Methodology. This document is a guide for project managers and developers who want to build software in close collaboration with a Large Language Model (LLM) assistant, such as the **Gemini CLI**.

vibeD is a hybrid approach that combines the rigor of milestone-driven development with the flexibility of Agile and Scrum. It's designed to maximize the unique capabilities of an LLM, turning it from a simple coding assistant into the primary driver of the development process.

Throughout this guide, we will use the [`shutri`](https://github.com/ashutoshmjain/shutri) project as a real-world case study. You can explore its artifacts to see vibeD in action.

### A Grounded Methodology, Not a Flight of Fancy

It is important to understand that the `vibeD` methodology was not created in a vacuum. It was developed and refined alongside the creation of a real-world application: the `shutri` audio editor. This project serves as the concrete, living example for every principle described in this guide.

The `shutri` project itself was "vibe-coded" from the ground up using this very process. This ensures that the methodology is practical, grounded, and proven—not just a theoretical fantasy. The relationship is symbiotic: `shutri` is the product of `vibeD`, and `vibeD` is the documented process that built `shutri`. By exploring the project's artifacts, you are seeing the direct output of the methodology in practice.

---

## 2. What's in a Name? 

In Hindi, the word **"shutri" literally means **"vibes"** :-) 


---

## 3. The Core Philosophy: Your LLM as the Developer

The vibeD methodology is built on a fundamental shift in perspective: **the LLM is the developer**. The human's role is to act as the **Project Manager** or **Product Owner**—setting the vision, defining the goals, and verifying the results.

*   **Agent-Driven Development:** The LLM writes the code, runs the tests, and manages the project artifacts.
*   **Self-Documentation:** The LLM is responsible for creating and maintaining all project documentation, ensuring it is always synchronized with the code.
*   **Integrated Version Control:** All `git` activities—from writing detailed commit messages to staging and committing files—are performed by the LLM.

This approach creates a powerful, self-documenting, and highly-auditable development process.

---

## 4. How to Implement vibeD: A Step-by-Step Guide

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
> **See the artifact:** [`TECH_SPECS.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/TECH_SPECS.md)

### Step 2: Define the "Ways of Working"

Once the technical vision is clear, the next step is to define the development process itself. This is where you create the `vibeD_METHODOLOGY.md` document (the very guide you are reading now).

**Your Goal:** To establish a clear, documented process for the project.

**How to do it:**
1.  **Ask the LLM** to propose a high-level, milestone-based plan based on the `TECH_SPECS.md`.
2.  **Instruct it to break the plan into phases and milestones.** Each milestone should have a clear, testable goal. This forms the "Development Plan" section.
3.  **Define the quality and documentation standards.** Ask the LLM to establish rules for defect tracking, change management, and code documentation. This ensures consistency and quality from the start.

**A Note on Planning Philosophy:** The vibeD methodology deliberately avoids detailed, upfront project planning with Work Breakdown Structures (WBS) or tools like Microsoft Project. The focus is on high-level, flexible sprint planning. Each milestone represents a sprint with a clear goal, but the specific tasks to achieve that goal are determined dynamically by the LLM during the development process. This approach maintains agility and leverages the LLM's ability to adapt and solve problems without being constrained by a rigid, pre-defined task list.

> #### **Case Study: The `shutri` Development Plan**
>
> The following is the complete, multi-phase development plan created for the `shutri` project. It serves as a concrete example of the output of this step.
>
> ##### **Phase 1: Prototype**
> This phase focuses on building the core functionality of `shutri` and validating the text-based audio editing concept.
>
> *   **Milestone 1: Project Setup and Core Data Structures**
> *   **Milestone 2: Audio Import and Splitting**
> *   **Milestone 3: Transcription File Generation**
> *   **Milestone 4: Vim Integration & Playback**
> *   **Milestone 5: Audio Export**
> *   **Milestone 6: Real Transcription Service & Authentication**
> *   **Milestone 7: Polish and Finalize**
>
> ##### **Phase 2: Dev**
> This phase focuses on expanding features, stability, and reach.
>
> *   **Milestone 8: Alpha Release & Community Feedback**
> *   **Milestone 9: Feature Complete & API Stability**
> *   **Milestone 10: Cross-Platform Support**
>
> ##### **Phase 3: User**
> This phase focuses on preparing the application for a wider, non-technical audience.
>
> *   **Milestone 11: Beta Release & User Acceptance Testing (UAT)**
> *   **Milestone 12: General Availability (GA) & Long-Term Support (LTS)**

### Step 3: Plan and Execute Sprints (Milestones)

This is the core loop of the vibeD methodology. You will treat each milestone as a self-contained "sprint."

#### Part A: Create the Test Plan

For each new milestone, your first action is to ask the LLM to create a dedicated test plan. This document translates the milestone's goal into a concrete, verifiable checklist. While the LLM can and should write automated unit and integration tests, the milestone test plan is different—it's a list of actions for a human to perform to validate the outcome.

**Human-in-the-Loop Testing is Mandatory:** The vibeD methodology requires that a human verifies the outcome of every milestone. The LLM's role is to make this process as easy as possible by generating a clear and comprehensive test plan.

> **Case Study: `shutri`**
> For each milestone, the Gemini CLI created a detailed test plan outlining the steps for manual verification.
>
> **See the artifacts:**
> *   [`TESTPLAN_MILESTONE2.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/TESTPLAN_MILESTONE2.md)
> *   [`TESTPLAN_MILESTONE3.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/TESTPLAN_MILESTONE3.md)
> *   [`TESTPLAN_MILESTONE4.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/TESTPLAN_MILESTONE4.md)

#### Part B: Execute and Verify

1.  **Instruct the LLM to Begin Development:** With the test plan as its guide, instruct the LLM to start working on the milestone's tasks.
2.  **Verify the Outcome:** Once the LLM reports that the work is complete, your job is to follow the test plan and verify the outcome.
3.  **Commit the Work:** If the outcome is successful, instruct the LLM to commit the changes with a descriptive message. The milestone is now complete.

### Step 4: Maintain Project Artifacts

A successful project relies on well-maintained documentation. In vibeD, this is an active, ongoing process driven by the LLM.

#### Knowledge Management

The **`KNOWLEDGE_BASE.md`** is the project's central, living repository of information. It's used to store important context, design decisions, and learnings that don't fit neatly into the technical specs or methodology.

**How to do it:** As you work with the LLM and make decisions, periodically instruct it to summarize the key takeaways and add them to the knowledge base. This prevents valuable context from being lost in the conversation history.

> **Case Study: `shutri`**
> The project's knowledge base is used to track architectural decisions and other key project information.
>
> **See the artifact:** [`KNOWLEDGE_BASE.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/KNOWLEDGE_BASE.md)

#### Change and Defect Tracking

No plan is perfect. vibeD manages change through two key documents:

*   **`DEFECT_TRACKER.md`**: Used to log any bug that breaks the functionality of a *previously completed* milestone.
*   **`CHANGE_TRACKER.md`**: Used to log any proposed enhancement or change that would alter the scope of a *previously completed* milestone.

> **Case Study: `shutri`**
> These trackers are used to maintain the quality and integrity of the project's approved milestones.
>
> **See the artifacts:**
> *   [`DEFECT_TRACKER.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/DEFECT_TRACKER.md)
> *   [`CHANGE_TRACKER.md`](https://github.com/ashutoshmjain/shutri/blob/main/PROJECT_ARTIFACTS/CHANGE_TRACKER.md)

#### Guiding Your AI Assistant with `GEMINI.md`

To ensure the LLM assistant remains perfectly aligned with the project's standards and conventions across different machines and development sessions, vibeD utilizes a special directive file.

*   **[`GEMINI.md`](https://github.com/ashutoshmjain/shutri/blob/main/GEMINI.md)**: This file, located in the project's root directory, contains a set of persistent instructions for the Gemini agent. The agent is required to read and adhere to these instructions every time it is invoked within the project.

**How to do it:**
1.  Create a `GEMINI.md` file in the root of your git repository.
2.  Add specific, clear instructions that you want the LLM to follow. This can include rules for code generation, documentation standards, or user-specific conventions (e.g., "My GitHub username is 'ashutoshmjain', not 'amj'").
3.  Commit this file to your remote repository. This ensures that the same set of directives is loaded automatically, providing a consistent development experience no matter which device you are working from.

This practice turns implicit conventions into explicit, automated instructions, further enhancing the consistency and reliability of the LLM-driven development process.

---

## 5. Measuring Success: Defect Density

A core principle of vibeD is that quality should improve over time. The primary metric for measuring this is **Defect Density**.

### What is Defect Density?

In the context of vibeD, Defect Density is not measured against lines of code. It is a measure of process stability, calculated as:

**Defect Density = (Number of Primary Defects) / (Number of Completed Milestones)**

A "Primary Defect" is any bug logged in the `DEFECT_TRACKER.md` that represents a regression in a previously completed and verified milestone.

### The Goal: A 75% Reduction Per Phase

The goal of the vibeD methodology is to **reduce Defect Density by 75% in each successive development phase.**

*   **Phase 1 (Prototype):** This phase establishes the baseline Defect Density. The focus is on speed and validating the core concept, so a higher number of defects is expected.
*   **Phase 2 (Dev):** The goal is to achieve a Defect Density that is 75% lower than the baseline. This demonstrates that the core architecture is stabilizing and the development process is maturing.
*   **Phase 3 (User):** The goal is another 75% reduction from the Dev phase's density. This proves that the application is hardening and becoming robust enough for a general release.

Because the LLM assistant maintains the `DEFECT_TRACKER.md`, calculating this metric is a straightforward, data-driven process. It is the Project Manager's responsibility to periodically review this metric to ensure the project's quality is improving as expected.

---

## 6. Conclusion

The vibeD methodology provides a structured, transparent, and highly efficient framework for building software with an LLM assistant. By casting the human as the project manager and the LLM as the developer, it leverages the strengths of both to create high-quality, well-documented, and rapidly-developed projects.
