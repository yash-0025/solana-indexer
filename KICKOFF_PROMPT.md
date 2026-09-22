# 🚀 KICKOFF_PROMPT.md — Paste this to start a session

Copy everything below the line into a fresh chat (with all workspace files attached/available: `ROADMAP.md`, `RULES.md`, `EXERCISES.md`, `SOLUTIONS.md`, `EXAMPLES.md`, `DECISIONS.md`, `LEARNING.md`, `LOGS.md`, `NOTES.md`, `PROMPTS.md`, `QUESTIONS.md`, `README.md`).

---

I'm working through my **Solana Indexer Roadmap** (`ROADMAP.md` in this workspace) — a milestone-based, project-based build of a production-grade Solana indexer in Rust, structured as 3 Phases × 15 Modules (45 total), same shape as my Trading Platform Roadmap. I just finished a 30-day Rust Mastery Roadmap first (ownership → traits/generics → concurrency/async → Axum + sqlx REST APIs), so I'm comfortable with core Rust — I'm new to Solana's account/transaction model and to indexing architecture specifically. Phase 1 is a synchronous CLI indexer (foundations), Phase 2 makes it an async production backend (Postgres, Redis, Axum, Docker), and Phase 3 is advanced infra (reorg handling, a real Geyser plugin, lock-free pipelines, zero-copy parsing, procedural macro codegen, sharding).

**Before doing anything else, read these in full:**
1. `RULES.md` — the governance rules for how you (the AI) operate in this project. Follow them on every single message, not just this first one.
2. `ROADMAP.md` — the full 3-phase, 45-module curriculum. Don't edit it without asking me first and getting an explicit yes.
3. `LEARNING.md` — my current progress log. Check what's marked `[x]`, `[~]`, `[!]`, or `[ ]` before deciding where we are.
4. `EXAMPLES.md`, `EXERCISES.md`, `SOLUTIONS.md`, `DECISIONS.md`, `LOGS.md` if they have content — these hold prior context you should cross-check against.

**Then:**
- Tell me which module `LEARNING.md` says I'm on (or confirm we're starting fresh at Module 1.1 if it's empty).
- Give me the **big-picture project overview** (rule 13/15) for that module before we write any code — what we're building, the end outcome, and why, plus the architecture/data-flow picture if it's a new pipeline stage or new phase.
- Then walk me through the module **one concept at a time**, in project context, with the ELI5 (data-pipeline/cataloguing/ledger domain analogy) + rigorous technical explanation for each concept, stored to `EXAMPLES.md` verbatim as we go (rule 8).
- When it's time for me to write code, give me a **skeleton with `todo!()`** in `EXERCISES.md` per rule 17 — not the full solution. I'll attempt it and paste my attempt back before you show me `SOLUTIONS.md`.
- Log every file edit to `LOGS.md` as you make it (rule 2).
- At the end of the module, audit the roadmap's full "You build", "Concepts", and "Architecture" list for that module against what we actually built (rule 14) before asking if we can mark it complete in `LEARNING.md` and `ROADMAP.md` — and only touch those two files after I say yes.

One more thing: I want to actually be a master of this by the end of Phase 3 — a real, portfolio-defensible, multi-program indexing platform, not a toy. This isn't time-boxed, so if a module (especially 1.11, 2.4, 2.7, or 3.1 — the correctness-critical ones) needs more time than a single session, tell me directly rather than rushing to keep pace.

Let's start.