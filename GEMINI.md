# Gemini Agent Directives for the `shutri` Project

This document contains essential instructions for the Gemini agent. You MUST adhere to these directives in every session within this project to ensure consistency, quality, and alignment with the project's goals.

## On Session Start

1.  **Full Project Context Ingestion:** Before taking any action, you MUST read the entire contents of the following files located in the `PROJECT_ARTIFACTS/` directory to build a complete understanding of the project's current state:
    *   `TECH_SPECS.md`: The primary technical design and architectural blueprint.
    *   `KNOWLEDGE_BASE.md`: The repository of established patterns, decisions, and technical solutions.
    *   `DEFECT_TRACKER.md`: The log of all known issues and their status.
    *   `CHANGE_TRACKER.md`: The log of all significant changes and their rationale.
    *   All `TESTPLAN_MILESTONE*.md` files: The testing requirements for each milestone.

2.  **Consistency Analysis:** After reading the artifacts, perform a brief internal consistency check. If you identify any contradictions between the documents (e.g., a defect marked `Closed` whose fix is not reflected in the `TECH_SPECS`), you must raise this to the user for clarification before proceeding with any conflicting tasks.

## Core Operational Mandates

1.  **Adherence to Specifications:** All code generation, modifications, and refactoring MUST strictly adhere to the architecture, data structures, and workflows defined in `TECH_SPECS.md`.
2.  **Leverage Existing Knowledge:** You MUST consult the `KNOWLEDGE_BASE.md` to ensure your solutions align with previously established patterns and decisions. Do not re-solve problems that are already documented.
3.  **Defect-Aware Development:** You MUST be aware of all `Open` issues in `DEFECT_TRACKER.md`. Do not write new code that replicates known bugs. If your task is to fix a defect, your solution must align with the resolution strategy, if one is documented.
4.  **Test-Driven Implementation:** All new features or bug fixes must be accompanied by corresponding tests that validate their correctness, following the guidelines in the relevant `TESTPLAN_MILESTONE*.md` file.
5.  **Maintain Documentation:** When you make a change that qualifies as a "Primary Defect" fix or a significant architectural modification, you MUST update the `DEFECT_TRACKER.md` or `CHANGE_TRACKER.md` accordingly, in addition to any necessary changes to the `TECH_SPECS.md` or `KNOWLEDGE_BASE.md`.

## User-Specific Conventions

*   **GitHub Username:** The project owner's GitHub username is `ashutoshmjain`. All generated links to GitHub profiles or content, especially within documentation like `VIBE_METHODOLOGY.md`, MUST use this username for consistency, not `amj`.
