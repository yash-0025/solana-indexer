# Social Journey Content on X (formerly Twitter) — Rule 25

## Context & Purpose
Building a production-grade Solana indexer in Rust is an intensive, high-signal engineering journey. Sharing this journey publicly on X (formerly Twitter) reinforces learning, builds public proof-of-work, and connects with the Rust and Solana developer ecosystems.

## Core Directives

1. **Trigger & Location**:
   - Whenever an exercise/concept is completed and verified, generate fresh social content inside `social.md` at the project root.

2. **Dual Formats Per Milestone**:
   - **Standalone Post (1 Tweet)**: High-impact single post highlighting the specific milestone, engineering concept, or key takeaway.
   - **Thread (3–5 Tweets)**: Narrative progression:
     - *Tweet 1 (Hook)*: The counter-intuitive problem or milestone.
     - *Tweet 2 (The Architecture / Problem)*: Why naive approaches fail on Solana.
     - *Tweet 3 (What We Built & Rust Decisions)*: Specific types, methods, or trade-offs chosen.
     - *Tweet 4 (Key Takeaway / Next Step)*: The broader engineering lesson and what comes next.

3. **Completely Distinct Content & Topics (NO Translations or Paraphrasing)**:
   - Provide content in BOTH **English** and **Hinglish**.
   - **Crucial Requirement**: The English and Hinglish versions must NOT cover the same points or be paraphrased translations of each other. They must explore completely different technical sub-topics, challenges, or architectural angles of the milestone:
     - **English Focus**: Deep systems programming, Rust type invariants, zero-cost abstractions, memory layout (`repr(transparent)`, stack vs heap), compile-time guarantees, or production performance considerations.
     - **Hinglish Focus**: Solana runtime architecture quirks, consensus leader skips & fork hazards, developer debugging gotchas, real-world analogies (bullet train, overworked RPC), or honest builder journey reflections.

4. **Strict Character Limit Enforcement**:
   - Every single tweet/post must strictly stay under **280 characters** (target: **240–270 characters**).
   - Verify character counts before writing to `social.md`.
