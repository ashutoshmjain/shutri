# Research Ingestion & Peer Review Workflow

> **End-to-End Guide for Internal & External Research Intake in the Shutri Media Solution (SMS)**

---

## 🏛️ Workflow Overview

The research ingestion pipeline bridges human researchers, **deepMind AI (`dm`)**, and the **deepDive** knowledge ledger through a clear 4-step lifecycle:

```mermaid
graph TD
    H1["1. Submission (Form / GitHub Issue)"] -->|Author Check| Check{"Is Author Internal?"}
    
    Check -->|Yes: ashutoshmjain| IntTrack["Internal Track (template)"]
    Check -->|No: External Author| ExtTrack["External Track (mempool)"]

    IntTrack -->|Assign Clean Key| CleanKey["Key: XXX.md"]
    ExtTrack -->|Assign Parked Key| ParkKey["Key: _XXX.md"]

    CleanKey --> H2["2. Human runs Master Packaging Prompt in Gemini"]
    ParkKey --> H2

    H2 -->|Drop extract.py into Downloads| DL["Downloads/ Folder"]

    DL -->|Run md-publish CLI| Ingest["3. Ingestion Engine (md-publish)"]
    
    Ingested -->|Internal| Template["deepDive/src/XXX.md (# Recent / Template)"]
    Ingested -->|External| Mempool["deepDive/src/_XXX.md (# Mempool / Parked)"]

    Mempool -->|4. Team Review via NotebookLM| Review{"Reviewer Decision"}
    Review -->|Post /approve on Issue| Approve["Agent runs: md-publish --unpark XXX XXX"]
    Approve --> Template
```

---

## 🔢 1. Numbering & Staging Modalities

| Modality | Staging Target | Filename Format | Tree Location in `SUMMARY.md` | Author Type |
|---|---|---|---|---|
| **`template`** | Active Mining / Production | `src/XXX.md` (e.g. `245.md`) | `# Recent ..` / `# block template` | Internal Core Team (`ashutoshmjain`) |
| **`mempool`** | Unconfirmed Staging | `src/_XXX.md` (e.g. `_245.md`) | `# The Mempool (Unconfirmed)` | External Author / Community |

---

## 📋 2. Step-by-Step Lifecycle Guide

### Step 1: Research Submission
- Submitter uses the form on **`shutri.com`** or opens a GitHub Issue.
- The **Intake Agent** inspects the submitter username:
  - If **Internal (`ashutoshmjain`)**: Assigns Episode Key `XXX` and routes to **`template`**.
  - If **External**: Assigns Parked Key `_XXX` and routes to **`mempool`**.

### Step 2: Human-in-the-Loop Packaging (Gemini Deep Research)
- The submitter/editor copies the **Master Packaging Prompt** from `shutri/SOUL.md` or `mdIngest`.
- Paste the prompt into Gemini Deep Research.
- Save the resulting self-extracting Python script (`extract.py`) into your local `Downloads/` folder.

### Step 3: Agent Ingestion (`md-publish`)
- The agent (or editor) executes `python Downloads/extract.py` to yield `final_research.md`.
- **For Internal Track (`template`):**
  ```bash
  md-publish --text XXX
  ```
  *(Creates `src/XXX.md` and indexes under `# Recent ..`)*

- **For External Track (`mempool`):**
  ```bash
  md-publish --text XXX
  md-publish --park XXX
  ```
  *(Creates `src/_XXX.md` and indexes under `# The Mempool`)*

### Step 4: Review & Promotion (`/approve`)
- A team member evaluates `src/_XXX.md` against NotebookLM scope and criteria.
- **To Approve & Promote:**
  - Post `/approve` as a comment on the GitHub Issue (or add the `approved` label).
  - The Intake Agent detects `/approve` and executes:
    ```bash
    md-publish --unpark XXX XXX
    ```
  - **Result:** Promotes `src/_XXX.md` $\rightarrow$ `src/XXX.md`, moves it from `# Mempool` into `# Recent ..`, and closes the GitHub Issue.
