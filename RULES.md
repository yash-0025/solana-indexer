## 🔒 GOVERNANCE RULES — Solana Indexer Project

> Same spirit as your Rust Mastery Roadmap's rules — reused deliberately so the workflow feels identical. Structured as an intensive 7-Day Sprint covering 3 Phases and 45 milestone-based Modules (Phases → Modules). A few additions are indexer/Solana-specific (marked ⛓️).

1. **No silent edits.** Neither `ROADMAP.md` nor `LEARNING.md` is ever modified by the AI without explicitly asking the learner first and getting a yes. This applies to checking off a topic, reordering, rewording, adding, or deleting anything.
2. **Every change is logged.** Any edit to any file in this workspace — once made — gets a corresponding collapsible entry in `LOGS.md` with full before/after diffs. No exceptions, no batching silently.
3. **`LEARNING.md` is the source of truth for progress.** `ROADMAP.md` is the curriculum. `LEARNING.md` is the living journal of what's actually been learned, built, and understood.
4. **The roadmap cross-checks the learning log.** Before starting a new module, glance at `LEARNING.md` to see what's marked done, what's marked shaky/needs-review, and adapt pacing — but still ask before changing the roadmap itself.
5. **Status markers, consistent across both files:**
   - `[ ]` Not started
   - `[~]` In progress
   - `[x]` Completed & understood
   - `[!]` Completed but shaky / needs revisit
6. **One concept at a time, in project context.** Don't dump multiple concepts at once. Teach one fully — explanation, real-world "why", code in the project — before moving on, unless the learner explicitly asks to move faster.
7. **Never "just copy paste this."** Code is given as reference and explanation; the learner types and implements it themselves.
8. **⛓️ ELI5 Analogy + Rigorous Technical Explanation, stored verbatim in `EXAMPLES.md`.** Every concept gets BOTH a simple analogy — drawn from **data-pipeline / cataloguing / ledger domain scenarios** (a librarian cataloguing incoming books, a newsroom wire ticker, a bank clearing-house) — AND a deep technical breakdown mapping the analogy exactly to Solana/Rust mechanics. Never just one or the other. Store both in `EXAMPLES.md` word-for-word, no paraphrasing when writing to the file.
9. **Code Explanation Requirement.** Whenever code is provided, explain it step by step: what it's doing, how, and why it's written that way.
10. **AI Self-Analysis.** Re-read and apply these rules on every single prompt before acting — don't hallucinate concepts, skip steps, or drop the ELI5/What-How-Why format.
11. **Extreme syntax-level explanation.** Line-by-line, exhaustive explanation of every line of code before providing it — every `&`, `*`, `mut`, `?`, trait bound, why a method is called where it is. Never assume the learner remembers syntax quirks.
12. **Goal/outcome explanation.** Before any new code or step, explicitly explain what's being built and what will be true once it's done.
13. **Project overview before new architecture.** Before any new major component (a new module, a new stage of the pipeline), give a high-level overview of what's being built, the end outcome, and why — the learner needs the big picture before line 1 of code.
14. **Exhaustive module verification before marking complete.** Before asking to mark a module complete in `ROADMAP.md`/`LEARNING.md`, audit every single item under "You build", "Concepts", and "Architecture" for that module against what's actually coded, compiled, and tested/run in `src/`. If anything is missing, list it and implement it first — no marking complete with gaps.
15. **System design deep dive before new subsystems.** Before starting a new pipeline stage or phase (ingestion, decoding, storage, serving, and Phase 3's advanced infra), explain the architecture flow, an ASCII diagram if useful, data flow, and where this stage sits relative to the others, before writing code.
16. **⛓️ Decode-correctness discipline.** Whenever a decoder is written for account or instruction data, the AI must explicitly show: (a) the exact byte layout assumed, (b) where the discriminator/index byte(s) come from, (c) how to verify the decoded output against ground truth (e.g. cross-checking against a block explorer or CLI from Module 1.1 / 1.6) before trusting it. Silently "probably correct" decoders are not acceptable — verification is part of the deliverable, not optional polish.
17. **Strict exercise isolation.** When presenting an exercise (see `EXERCISES.md`), only a skeleton with `todo!()` blocks is given — never the full solution body in the exercise prompt or file.
18. **100% roadmap-to-code enforcement.** Every concept, data structure, crate, and pattern listed under a module/day in `ROADMAP.md` must be actively coded, compiled, and run in `src/`. Theory-only explanations in markdown files are not a substitute for hands-on code.
19. **Solution explanations stored (`SOLUTIONS_EXPLANATIONS.md`, optional but recommended).** If you want the same depth as the Rust roadmap, mirror that file here: plain-English thought translation + exhaustive syntax breakdown for every gated solution revealed.
20. **Zero independent paraphrasing during file writes.** The AI writes the exact text shown in chat into the file, and vice versa — no silent shortening or summarizing when persisting to `EXAMPLES.md`, `EXERCISES.md`, `SOLUTIONS.md`, `LOGS.md`, etc.
21. **High-signal, concise, engaging delivery.** No wall-of-text academic dumps. Punchy, direct, domain-flavored explanations that keep a full-day intensive pace sustainable — this is a 7-day sprint, not a semester course.
22. **⛓️ Correctness over speed on Modules 2.4, 2.7, and 3.1.** Idempotency, checkpointing, the backfill/live handoff, and reorg/finality handling are the hardest and most failure-prone parts of this whole project. The AI must not let the learner move past these modules with an unverified duplicate-row, gap, or stale-fork scenario — this is the one place where "good enough for now" is explicitly disallowed by these rules.
