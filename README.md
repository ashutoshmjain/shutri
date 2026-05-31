# #SMS: Shutri Media Solution

**#SMS** is a professional-grade media workflow engine designed to automate the bridge between AI-native content generation (LLMs) and multi-platform publishing. It provides a modular pipeline to ingest raw research assets, "massage" them into high-fidelity formats, and publish them to production repositories.

## 🚀 Key Features

*   **Ontology-First Ingestion:** Assets (Text, Images, Video) are organized and indexed by a Master Key (MK).
*   **High-Fidelity Massaging:** 
    *   **mdBook:** Automatic KaTeX hardening, footnote re-indexing, and cover art injection.
    *   **Socials:** Specialized buffers for LinkedIn (AI-ready) and Nostr (NIP-23).
*   **Multimedia Orchestration:** Dynamically generates a global Javascript-powered cinematic video carousel for web publishing.
*   **n8n Cockpit:** A visual dashboard to trigger the entire pipeline via a single Webhook or terminal command.
*   **Professional Rigor:** Built using the **vibeD** methodology with full technical specs and defect tracking.

## 🛠️ Tech Stack

*   **Engine:** Rust (CLI)
*   **Automation:** n8n (Self-hosted on WSL/Debian)
*   **Distribution:** mdBook, LinkedIn, Nostr (NIP-23)

## 📖 Project Artifacts

The project is governed by its artifacts in the `PROJECT_ARTIFACTS/` directory:
*   `TECH_SPECS.md`: Architectural design and roadmap.
*   `DEFECT_TRACKER.md`: Milestone approvals and quality logs.
*   `KNOWLEDGE_BASE.md`: Patterns, decisions, and environment configuration.

## 🚦 Getting Started

1.  **Build the Engine:** `cd shutri && cargo build`
2.  **Start n8n:** Run the `start_n8n.sh` script to enable local privileges.
3.  **Import Workflow:** Load `SMS_FULL_PIPELINE.json` into your n8n UI.
4.  **Trigger:** 
    ```bash
    curl -X POST http://localhost:5678/webhook-test/sms-trigger \
    -H "Content-Type: application/json" \
    -d '{"mk": 242, "source": "/path/to/asset.rs"}'
    ```

---
Built with natural and artificial intelligence. **#SMS**
