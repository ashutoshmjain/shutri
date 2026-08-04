# The Shutri Manifesto

## Version 0.1

### A Reference Architecture for Open AI-Assisted Research Review

---

# 1. Introduction

Artificial Intelligence has substantially reduced the cost of generating knowledge. Research papers, software, educational material, simulations, and scientific hypotheses can now be produced at a rate that was previously impossible.

The challenge is no longer generating information.

The challenge is reviewing it.

Existing review systems evolved during a period when research itself was expensive to produce. As the cost of generation approaches zero, the limiting factor becomes the availability of experts willing to invest their attention in understanding, verifying, organizing, and communicating knowledge.

The purpose of Shutri is to describe one practical approach to this problem.

This document presents a reference architecture for conducting open, AI-assisted research review using freely available tools. The architecture has been implemented and is currently deployed through the **deepDive** project, where it has been exercised over hundreds of review episodes. The implementation is not intended to be definitive. It is intended to demonstrate one complete and working solution.

Reviewers are encouraged to adapt every component according to the requirements of their own discipline.

---

# 2. Design Goals

The architecture is guided by a small number of practical constraints.

## Zero operational cost for reviewers

Reviewers contribute their attention. The architecture should minimize recurring infrastructure costs so that participation is not limited by financial resources.

## Public by default

Research, discussions, revisions, and publications should be publicly accessible whenever confidentiality does not require otherwise.

## Modular

Each capability should be independently replaceable without affecting the rest of the workflow.

## AI-assisted, human-reviewed

Artificial Intelligence accelerates review. Human reviewers remain responsible for every published conclusion.

## Durable

Reviewed knowledge should remain accessible through open formats that are independent of any particular software vendor.

---

# 3. Reference Architecture

The reference architecture consists of three products and one reference deployment.

The products are intended to be reusable across disciplines. The reference deployment demonstrates how they operate together as a complete review system.

## 3.1 Publication Utilities

The first product provides the publication pipeline.

Its purpose is to transform reviewed material into durable, version-controlled publications that can be hosted at minimal cost.

The reference implementation combines:

- mdIngest for content ingestion and publication automation.
- mdBook for static publication.
- GitHub Pages for hosting.
- Progressive Web App support for offline reading.

These technologies are not architectural requirements. They satisfy the current design goals and may be replaced by equivalent alternatives.

---

## 3.2 Social Media Automation

Publication is necessary but not sufficient.

Reviewed knowledge should be communicated in forms appropriate for different audiences.

The second product provides automation for generating and managing derivative media including podcasts, short-form video, and supporting visual material.

The current reference implementation is DDMA.

The objective is not promotion.

The objective is accessibility.

---

## 3.3 Knowledge Ledger

Knowledge should accumulate rather than disappear into isolated publications.

The architecture therefore organizes reviewed material into a persistent ledger.

Research first enters a **Mempool**, where new topics await review.

Accepted reviews are developed within a **Template**.

Completed groups of reviews are committed to a permanent **Chain**.

The terminology is intentionally borrowed from Bitcoin because it provides a familiar organizational model. Unlike Bitcoin, the ledger provides neither consensus nor Proof of Work. It is simply a structured method for organizing reviewed knowledge.

Each completed chain is subsequently made available through a conversational interface, allowing readers to explore reviewed material without reading every publication sequentially.

---

## 3.4 Reference Deployment

The architecture described in this document is implemented by the **deepDive** project.

deepDive demonstrates the complete workflow:

```text
Research Ingestion
        ↓
      Review
        ↓
    Publication
        ↓
      Podcast
        ↓
 Short-form Media
        ↓
 Knowledge Ledger
```

The purpose of deepDive is not to define the architecture.

Its purpose is to validate it.

---

# 4. Proof of Attention

The central contribution made by a reviewer is attention.

Attention consists of reading carefully, verifying evidence, improving explanations, responding to criticism, refining publications, and communicating reviewed knowledge to wider audiences.

These activities produce observable public artifacts.

- A reviewed publication.
- A revised explanation.
- A podcast.
- A diagram.
- A discussion.

Collectively, these artifacts constitute **Proof of Attention**.

Proof of Attention is not a protocol.

It is not a cryptocurrency.

It is not a reputation system.

It is simply the visible record of careful intellectual work.

---

# 5. The Reviewer

The architecture assumes reviewers are independent practitioners with expertise in one or more fields.

Reviewers **SHOULD**:

- Review before publishing.
- Distinguish evidence from opinion.
- Acknowledge uncertainty where appropriate.
- Improve the clarity of reviewed material.
- Communicate knowledge beyond specialist audiences.
- Contribute improvements back to the architecture whenever practical.

Reviewers **MAY** replace any implementation component provided the replacement satisfies the design goals described in this document.

---

# 6. Extensibility

This document intentionally specifies capabilities rather than technologies.

Future reviewers may replace GitHub, mdBook, NotebookLM, DDMA, or any other implementation with alternatives that better satisfy the needs of their discipline.

The architecture should evolve.

The design goals should remain comparatively stable.

Success should therefore not be measured by uniformity of implementation.

It should be measured by the number of independent reviewers capable of adapting the architecture to produce thoughtful, publicly accessible, and durable review.

---

# 7. Closing Remarks

This document does not propose a new publishing platform.

It documents one reference architecture for open AI-assisted research review.

The accompanying software demonstrates that the architecture is practical.

Its future development depends upon reviewers who adapt it, improve it, and apply it within their own disciplines.

The objective is not to standardize software.

The objective is to make high-quality open review easier to practice.
