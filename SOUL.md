# SOUL.md — The Soul of the Shutri Media Solution (SMS)

> **The Foundational Core, Unified Vision, & Language Maturity Model**  
> *Every agent operating in any repository MUST derive its behavior from this document.*

---

## 🌌 1. The Core Philosophy & Synergy

The **Shutri Media Solution (SMS)** is a unified, collaborative ecosystem. Individual repositories and agents do NOT operate in isolation, nor do they work against each other. They are branches of a single organism designed to turn human and AI inquiry into an immutable ledger of knowledge and multi-modal media.

```mermaid
graph TD
    SOUL["SOUL.md (The Core Blueprint & Shared Soul)"] --> Intake["Pillar 1: Research Intake (shutri)"]
    SOUL --> Publish Engine["Pillar 2: Open Engine (mdIngest)"]
    SOUL --> Publish App["Pillar 2: Production App (deepDive)"]
    SOUL --> Promotion["Pillar 3: Media Automator (ddma)"]

    Intake -->|Lossless Payload| Publish Engine
    Publish Engine -->|Sanitized Ledger| Publish App
    Publish App -->|Text & Audio| Promotion
    Promotion -->|Square Video Carousels| Publish App
```

---

## 🏛️ 2. The Deterministic Agent Operating Model

> **"Deterministic code is the house we build; the intelligent agent is the one navigating through this house."**

1. **Token Efficiency:** Agents MUST NOT waste intelligence tokens doing manual string formatting, text parsing, or manual video slicing in memory. They invoke deterministic CLI tools.
2. **Upstream Hardening Loop:** When an agent encounters a bug in production (`deepDive`), it **never applies a manual band-aid**. It patches the underlying deterministic codebase upstream (`mdIngest` or `ddma`), compiles the update, and re-executes.
3. **Cross-Agent Synergy:** Agents collaborate across repository boundaries. An update to `ddma` respects the media format of `deepDive`; a patch in `mdIngest` serves as an open-source enhancement for all consumers.

---

## 📈 3. The Language & Solution Maturity Model

The development of tools within SMS follows a strict two-phase maturity lifecycle:

```
  ┌─────────────────────────────────────────────────────────────┐
  │                 PHASE 1: SCRIPTING & PROTOTYPING             │
  │                      (Python / JavaScript)                  │
  │  • Fast iteration, rapid experimentation, schema validation │
  │  • Example: ddma.py, Whisper pipelines, prototype scripts  │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ Once logic matures & stabilizes
                                 ▼
  ┌─────────────────────────────────────────────────────────────┐
  │                 PHASE 2: COMPILED BINARY HARDENING          │
  │                        (Rust / Go)                          │
  │  • Zero-dependency compiled binary (e.g. md-publish)        │
  │  • Cross-platform portability (Windows, macOS, Linux)       │
  │  • Ultra-fast execution, zero environment drift             │
  └──────────────────────────────┬──────────────────────────────┘
```

* **Phase 1 (Python Prototyping):** New workflows (e.g., DDMA clip slicing, Whisper alignment, Mosaic AI calls) begin in Python for maximum speed of experimentation.
* **Phase 2 (Rust/Go Binary Hardening):** Once a workflow matures and stabilizes (e.g., `mdIngest` / `md-publish`), it is re-engineered into a compiled, single-file binary (Rust or Go). This guarantees instant cross-platform distribution without Python environment or dependency setup issues.

---

## 🛡️ 4. Mandatory Pre-Execution Directive for ALL Agents

Before executing any task, every subagent or Antigravity agent session MUST enforce:

1. **Read `SOUL.md` First:** Inspect this file (`SOUL.md`) to align with the global vision, maturity model, and multi-repo synergy.
2. **Identify Solution Maturity:** Determine whether the tool being modified is in **Phase 1 (Python Prototype)** or **Phase 2 (Compiled Rust/Go Binary)**.
3. **No Isolated Hacks:** Always trace bugs upstream to the deterministic engine rather than patching downstream content.
