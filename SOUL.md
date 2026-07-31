# SOUL.md — The Soul of the Shutri Media Solution (SMS)

> **Empowering the New Era of Citizen & AI-Assisted Research**  
> *Every agent operating in any repository MUST derive its behavior from this document.*

---

## 🌟 1. The Core Purpose & Positive Vision

Artificial Intelligence (Gemini, deepMind, ChatGPT) has unlocked a massive new frontier: **research is no longer confined to academic institutions, PhDs, or postdocs**. Everyday domain experts, engineers, and independent thinkers can now perform cutting-edge research powered by AI.

However, because **human attention comes at a premium**, independent researchers struggle to find legitimate peer review and syndication for their work.

**Shutri Media Solution (SMS)** exists to serve this expanding frontier:
* **Open Platform for Citizen & AI-Assisted Research:** Providing an accessible, transparent, high-rigor peer-review registry for independent researchers.
* **Human-AI Synergy:** Combining human domain expertise with **deepMind AI (`dm`)** synthesis to review, package, and syndicate original research.
* **Multi-Modal Reach:** Transforming verified research into mathematical web ledgers (`deepDive`), audio podcasts, and **740x740 square videos (<20MB)** for Nostr and social platforms (`ddma`).

```mermaid
graph TD
    SOUL["SOUL.md (The Empowering Core Blueprint)"] --> Intake["Pillar 1: Research Intake (shutri)"]
    SOUL --> Publish Engine["Pillar 2: Open Engine (mdIngest)"]
    SOUL --> Publish App["Pillar 2: Production App (deepDive)"]
    SOUL --> Promotion["Pillar 3: Media Automator (ddma)"]

    Intake -->|Context Report & Handshake| Publish App
    Publish Engine -->|Sanitized Rust Binary| Publish App
    Publish App -->|Text & Audio| Promotion
    Promotion -->|Square Video Carousels| Publish App
```

---

## 🔢 2. Staging & Numbering Modalities

* **Mempool Filenames (Unnumbered Slugs):** Unconfirmed external research drafts in the Mempool use **strictly unnumbered slugs** prefixed with an underscore (e.g. `src/_quantum-memory-consensus.md`), preventing index collisions.
* **Template Filenames (Numeric Episode Keys):** Episode numbers (e.g. `245`) ONLY exist when an episode enters the **Template** phase (Active Mining). Files in Template are named strictly by number: `src/245.md`.

---

## 🤝 3. The Human-Agent Handshake & Verification Protocol

```
  1. [Human Submitter] ➔ Creates Issue on shutri.com / GitHub
  2. [Intake Agent (shutri)] ➔ Analyzes existing ledger/issues ➔ Posts Context Report for Human Editor
  3. [Human Editor] ➔ Runs Master Packaging Prompt in Gemini ➔ Drops extract.py into Downloads/
  4. [Human Editor] ➔ Directs deepDive Agent to publish
  5. [deepDive Agent] ➔ Ingests file (src/245.md for Internal | src/_slug.md for External) ➔ Runs `mdbook build`
  6. [Human Editor] ➔ Verifies publication build
  7. [Intake Agent] ➔ Updates & closes GitHub Issue (ONLY after human confirmation!)
```

1. **Deterministic Navigation:** Agents invoke deterministic CLI tools (`md-publish`, `ddma.py`, `cargo build`, `mdbook build`) to conserve tokens.
2. **Upstream Hardening Loop:** When an intake bug occurs in production (`deepDive`), agents patch the deterministic code upstream in `mdIngest` or `ddma` rather than applying downstream band-aids.
3. **Human Verification Gate:** The Intake Agent NEVER updates or closes a GitHub Issue until the Human Editor explicitly verifies the build output.

---

## 📈 4. The Language & Solution Maturity Model

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
  └──────────────────────────────┴──────────────────────────────┘
```

---

## 🛡️ 5. Mandatory Pre-Execution Directive for ALL Agents

Before executing any task, every subagent or Antigravity agent session MUST enforce:

1. **Read `SOUL.md` First:** Inspect this file (`shutri/SOUL.md`) to align with the global vision, maturity model, and multi-repo synergy.
2. **Enforce Mempool vs Template Numbering:** Never assign episode numbers to Mempool files (`_slug.md`).
3. **Require Human Verification Confirmation:** Never update or close GitHub Issues until human confirmation is received.
