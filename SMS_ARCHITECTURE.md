# Shutri Media Solution (SMS): Master Architecture & Governance

> **System Blueprint & Agent Navigation Model**

---

## 🏛️ 1. High-Level Solution Architecture

The **Shutri Media Solution (SMS)** is an end-to-end, comprehensive research, publishing, and promotion platform structured into **Three Operational Pillars**:

```mermaid
graph TD
    subgraph Pillar 1: Research Intake (Staging & Review)
        A["GitHub Issues & Portal"] -->|Mempool Triage & dm AI Review| B["shutri Repository"]
    end

    subgraph Pillar 2: Research Publishing (Knowledge Ledger)
        C["mdIngest (Upstream Engine: coolchain / md-publish)"] -->|Sanitizes & Indexes| D["deepDive (Production Application & PWA Ledger)"]
    end

    subgraph Pillar 3: Research Promotion (Multi-Modal Media Engine)
        E["ddma (Media Automation Engine)"] -->|Whisper + 740x740 Square Video + Mosaic AI| F["Multi-Platform Syndication (Spotify, YouTube, Nostr <20MB, IG, TikTok)"]
    end

    B -->|Lossless Python Payload| C
    D -->|Episode Audio & Text| E
    E -->|Square Video Carousel & Podcast Links| D
```

---

## 📐 2. The Three Operational Pillars

### Pillar 1: Research Intake (`shutri`)
* **Role:** Intake staging, peer review, and community consensus.
* **Substrate:** Uses **GitHub Issue Management** as an open, transparent substrate where submitters lodge research drafts, AI (`deepMind`/`dm`) generates synthesis reports, human reviewers collaborate, and verified papers enter the **Mempool**.

### Pillar 2: Research Publishing (`mdIngest` + `deepDive`)
* **Upstream Open-Source Engine (`mdIngest`):** A lean, platform-agnostic Rust CLI (`coolchain` / `md-publish`). Holds **zero episode content**; holds only deterministic sanitization, KaTeX hardening, and indexer logic.
* **Production Application (`deepDive`):** The live, consumer-facing Progressive Web App ([deepdive.shutri.com](https://deepdive.shutri.com)) holding the 200+ episode ledger, PWA service workers, and `SUMMARY.md` tree.

### Pillar 3: Research Promotion (`ddma`)
* **Role:** Multi-modal media automation and infographic generation.
* **Mechanism:** Ingests raw audio $\rightarrow$ runs Whisper word-level transcription $\rightarrow$ cuts timelines with `acrossfade` stings $\rightarrow$ renders **740x740 Square Videos (<20 MB for Nostr relay compliance)** and Mosaic AI motion graphics $\rightarrow$ exports master audio (`.mp3`) and video (`.mp4`).

---

## 🤖 3. The Deterministic Agent Operating Philosophy

```
  ┌─────────────────────────────────────────────────────────────┐
  │                 GOOGLE ANTIGRAVITY (AGY)                    │
  │            Intelligent Agent (The Navigator)               │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ Invokes Deterministic CLI Commands
                                 ▼
  ┌─────────────────────────────────────────────────────────────┐
  │                   DETERMINISTIC CODEBASE                    │
  │                  "The House We Are Building"                │
  │                                                             │
  │  • md-publish (Rust CLI)     • ddma.py (Fast Demuxer & Fades)│
  │  • mdbook / mdbook-katex     • FFmpeg & Whisper Pipelines     │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ When Bugs / Edge Cases Are Found
                                 ▼
  ┌─────────────────────────────────────────────────────────────┐
  │                   UPSTREAM HARDENING LOOP                   │
  │  Agent patches deterministic code ➔ Recompiles ➔ Re-runs CLI│
  └─────────────────────────────────────────────────────────────┘
```

1. **Deterministic Execution Over Token Waste:** AI LLMs must NOT perform manual text manipulation or video stream slicing in memory. They invoke deterministic CLI tools (`md-publish`, `ddma.py`, `cargo build`, `mdbook build`).
2. **Upstream Hardening Loop:** When an agent encounters a bug in production (`deepDive`), it **never applies a manual band-aid**. It patches the underlying deterministic codebase (`mdIngest/crate/src/sanitizer.rs` or `ddma/ddma.py`), compiles the update, and re-executes.
3. **Compounding System Strength:** Every episode publication makes the underlying codebase more resilient for future runs.

---

## 📂 4. Repository Agent Mapping Matrix

| Pillar | Repository | Agent Role | Config File | Deterministic Codebase |
|---|---|---|---|---|
| **1. Intake** | **`shutri`** | Research Intake Agent | [`shutri/AGENTS.md`](file:///c:/Users/ashut/OneDrive/Desktop/github/shutri/AGENTS.md) | GitHub Issue Parsers, Staging Scripts |
| **2. Engine** | **`mdIngest`** | Ingestion Engine Agent | [`mdIngest/AGENTS.md`](file:///c:/Users/ashut/OneDrive/Desktop/github/mdIngest/AGENTS.md) | Rust `md-publish` binary (`crate/src/`) |
| **2. App** | **`deepDive`** | Content Orchestrator Agent | [`deepDive/AGENTS.md`](file:///c:/Users/ashut/OneDrive/Desktop/github/deepDive/AGENTS.md) | `mdBook`, `mdbook-katex`, `SUMMARY.md` Indexer |
| **3. Promotion** | **`ddma`** | Media Automator Agent | [`ddma/.agents/AGENTS.md`](file:///c:/Users/ashut/OneDrive/Desktop/github/ddma/.agents/AGENTS.md) | `ddma.py`, FFmpeg fast demuxer, Whisper crossfader |
