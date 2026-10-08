# 🗒️ LOGS.md — Change Log

> Every edit to any file in this workspace gets a collapsible entry here with a before/after diff, per `RULES.md` rule 2. No exceptions, no silent batching.

---

## 2026-10-08

<details>
<summary>2026-10-08 — LEARNING.md, ROADMAP.md — Mark Module 1.4 Complete (CLI Interface: The Indexer Terminal)</summary>

**Before:**
- `LEARNING.md` line 28: `- [ ] 1.4 — CLI Interface: The Indexer Terminal`.
- `ROADMAP.md` Module 1.4: All 4 items marked `[ ]`.

**After:**
- `LEARNING.md` line 28: Marked `- [x] 1.4 — CLI Interface: The Indexer Terminal`.
- `ROADMAP.md` Module 1.4: Checked off all 4 items `[x]`.

**Why:** Learner confirmed completion of Module 1.4 after exhaustive audit verified all roadmap items, concepts, command pattern dispatcher, and 13/13 passing tests per Rules 1, 2, 14, and 23.
</details>


<details>
<summary>2026-10-08 — social.md, SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md — Record Solution 1.4 & Generate Module 1.4 Social Content</summary>

**Before:**
- `social.md`: Ended at Module 1.3b content.
- `SOLUTIONS.md`: Ended at Solution 1.3b.
- `SOLUTIONS_EXPLANATIONS.md`: Ended at Solution 1.3b.
- `EXERCISES.md`: Exercise 1.4 open under `## Open / In-Progress`.

**After:**
- `social.md`: Added Module 1.4 social journey content (English standalone + 4-tweet thread on Clap derive & Algebraic Enums; Hinglish standalone + 4-tweet thread on Operator Debugging Realities & `--since <SLOT>` backfill recovery; all strictly <= 280 chars).
- `SOLUTIONS.md`: Appended Solution 1.4 reference implementation, "why this & why not that", and comparison with learner's passing attempt.
- `SOLUTIONS_EXPLANATIONS.md`: Appended Solution 1.4 plain English thought translation and exhaustive syntax breakdown.
- `EXERCISES.md`: Moved Exercise 1.4 to `## Solved` with learner's verified attempt; set `## Open / In-Progress` to empty.

**Why:** Learner verified all 13 workspace tests pass for Exercise 1.4. Recorded solutions, explanations, and social journey content per Rules 2, 19, 20, 25, and Step 3.5-D.
</details>


<details>
<summary>2026-10-08 — Cargo.toml, EXAMPLES.md, Rust-Decisions.md, EXERCISES.md, hinglish-docs.md — Initiate Concept 1.4 (CLI Interface & Subcommands with Clap Derive)</summary>

**Before:**
- `Cargo.toml`: Dependencies had `solana-client`, `solana-sdk`, `serde`, `toml` (lacked `clap`).
- `EXAMPLES.md`: Ended at Concept 1.3b.
- `Rust-Decisions.md`: Ended at Concept 1.3b.
- `EXERCISES.md`: `## Open / In-Progress` was empty.
- `hinglish-docs.md`: Ended at Section 8 + Cheatsheet as Section 9.

**After:**
- `Cargo.toml`: Added `clap = { version = "4.5", features = ["derive"] }`.
- `EXAMPLES.md`: Appended Concept 1.4 ELI5 (The Station Master's Dispatch Telegraph Console) and technical explanation.
- `Rust-Decisions.md`: Appended Concept 1.4 decisions (`clap` derive API vs builder pattern/`args()`, Enums as subcommands, `pub(crate)` visibility, `Option<u64>` for `--since`).
- `EXERCISES.md`: Added Exercise 1.4 skeleton (`Cli`, `Commands` enum, and `execute_command` dispatcher) under `## Open / In-Progress`.
- `hinglish-docs.md`: Inserted Section 9 for Module 1.4 with full 7-component structure grounded in real Solana dev workflows, updated TOC, and renumbered Cheatsheet to Section 10 with CLI parser comparisons.

**Why:** Resumed workflow via `/next` to start Phase 1 Day 1 Module 1.4 (CLI Interface: The Indexer Terminal) per Rules 2, 6, 8, 11, 12, 13, 15, 17, 20, 24, and Step 3 / 3.5.
</details>


<details>
<summary>2026-10-08 — SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md, LEARNING.md, ROADMAP.md, Conversation.md — Complete & Mark Module 1.3 (Configuration System)</summary>

**Before:**
- `SOLUTIONS.md`: Ended at Solution 1.3.
- `SOLUTIONS_EXPLANATIONS.md`: Ended at Solution 1.3.
- `EXERCISES.md`: Exercise 1.3b open under `## Open / In-Progress`.
- `LEARNING.md` line 27: `- [ ] 1.3 — Configuration System`.
- `ROADMAP.md` Module 1.3: All 4 deliverable items marked `[ ]`.
- `Conversation.md`: Awaiting learner review of Exercise 1.3b.

**After:**
- `SOLUTIONS.md`: Appended Solution 1.3b (3-tier configuration loader with TOML parsing and env var overrides).
- `SOLUTIONS_EXPLANATIONS.md`: Appended Solution 1.3b plain English thought translation and exhaustive syntax breakdown.
- `EXERCISES.md`: Moved Exercise 1.3b to `## Solved` with learner's verified passing attempt.
- `LEARNING.md` line 27: Marked `- [x] 1.3 — Configuration System`.
- `ROADMAP.md` Module 1.3: Checked off all 4 deliverable items `[x]`.
- `Conversation.md`: Added 2026-10-08 entry recording full Module 1.3 audit and completion.

**Why:** Learner confirmed all unit tests passed for Exercise 1.3b and instructed marking Module 1.3 complete per Rules 1, 2, 14, 19, 20, 23, and Step 5.
</details>

## 2026-10-07

<details>
<summary>2026-10-07 — hinglish-docs.md, .agents/rules/hinglish-docs.md, RULES.md, .agents/workflows/next.md — Ground Hinglish Intuition in Real-World Solana & Rust Dev Reality</summary>

**Before:**
- `hinglish-docs.md` Section 8 used a generic maritime port / customs house ELI5 analogy translated to Hinglish ("The Harbor Customs Clearing Rules").
- `.agents/rules/hinglish-docs.md`, `RULES.md` (Rule 24), and `.agents/workflows/next.md` (Step 3 Item 11) permitted generic domain analogies in Hinglish docs.

**After:**
- `hinglish-docs.md`: Rewrote Section 8 title and `### 📖 Intuition & Engineering Concept` to **Local Dev Se Production Cluster Tak Ka Safar (3-Tier Config)**, grounding intuition directly in:
  1. Local development with `solana-test-validator` (`127.0.0.1:8899`) and local `config.toml`.
  2. Teammate zero-setup fallback via Rust `Default` trait (`IndexerConfig::default()`).
  3. Production cloud/Docker container deployment with secret Helius/QuickNode RPC keys injected via `INDEXER_RPC_URL` environment variables without code recompilation.
- `.agents/rules/hinglish-docs.md`: Updated component 3 to strictly mandate grounding in real-life developer workflows, Solana cluster realities, and Rust mechanics rather than detached metaphors.
- `RULES.md`: Updated Rule 24 with explicit requirement that Hinglish intuition must never repeat detached metaphors from ELI5, but directly unite real-life dev workflows with Solana/Rust realities.
- `.agents/workflows/next.md`: Aligned Step 3 Item 11 with the same requirement.

**Why:** User explicitly requested that Hinglish intuition & engineering concepts connect directly to real-life developer workflows and Solana/Rust concepts, avoiding redundant translations of abstract ELI5 stories.
</details>

<details>
<summary>2026-10-07 — social.md, PROMPTS.md, Conversation.md — Add Module 1.3b Social Content & Update Workflow Gate Logging</summary>

**Before:**
- `social.md`: Lacked Module 1.3b entry.
- `PROMPTS.md`: Did not have prompts for mandatory approval gates and layered config testing.
- `Conversation.md`: Ended at Exercise 1.2b entry.

**After:**
- `social.md`: Added Module 1.3b entry:
  - English standalone post & 4-tweet thread focusing on **Why Partial TOML Deserialization Needs `Option<T>` & Serde Schema Mapping**.
  - Hinglish standalone post & 4-tweet thread focusing on **Docker Deployments & The 12-Factor "No Recompile" Mindset**.
  - All tweets strictly verified under 280 characters.
- `PROMPTS.md`: Added reusable prompt snippets for the mandatory file-edit approval gate and 3-tier config testing.
- `Conversation.md`: Added discussion entry for 2026-10-07 recording Exercise 1.3 completion, gate enforcement, and Module 1.3b alignment.

**Why:** Enforce strict governance rules, user approval gates, social media journey tracking (Rule 25), and prompt maintenance.
</details>

<details>
<summary>2026-10-07 — SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md, EXAMPLES.md, Rust-Decisions.md, hinglish-docs.md — Record Solution 1.3 & Initiate Concept 1.3b (3-Tier Precedence TOML & Env Loading)</summary>

**Before:**
- `SOLUTIONS.md`: Ended at Solution 1.2d.
- `SOLUTIONS_EXPLANATIONS.md`: Ended at Solution 1.2d.
- `EXERCISES.md`: Exercise 1.3 open under `## Open / In-Progress`.
- `EXAMPLES.md`: Ended at Concept 1.3.
- `Rust-Decisions.md`: Ended at Module 1.3 (`Option<T>` for Env Overrides).
- `hinglish-docs.md`: Had Sections 1 to 7 with Cheatsheet as Section 8.

**After:**
- `SOLUTIONS.md`: Appended Solution 1.3 (`IndexerConfig` & `Default` Devnet fallback reference implementation and comparison).
- `SOLUTIONS_EXPLANATIONS.md`: Appended Solution 1.3 plain English thought translation and syntax breakdown.
- `EXERCISES.md`: Moved Exercise 1.3 to `## Solved` with learner's passing attempt; added Exercise 1.3b skeleton (`ConfigFile` deserialization and `load_from_str_and_env` 3-tier precedence loading) to `## Open / In-Progress`.
- `EXAMPLES.md`: Appended Concept 1.3b ELI5 (The Harbor Customs Clearing Rules) and technical explanation.
- `Rust-Decisions.md`: Appended Concept 1.3b decisions (`ConfigFile` with `Option<T>` fields, `std::fs::read_to_string`, `std::env::var().ok()`, mutable borrowing for layered merging).
- `hinglish-docs.md`: Added Section 8 for Module 1.3b with complete 7-component structure, updated TOC, and renumbered Cheatsheet to Section 9.

**Why:** The learner completed and verified Exercise 1.3 with passing tests. Advanced curriculum to complete Module 1.3 deliverable (TOML file parsing and env var overrides) per Rules 2, 6, 8, 11, 12, 13, 14, 15, 17, 19, 20, 24, and Step 3 / 3.5.
</details>

<details>
<summary>2026-10-07 — social.md — Add Module 1.3 social journey content (English + Hinglish with distinct topics)</summary>

**Before:**
- `social.md` only had entries up to Module 1.2.

**After:**
- Added Module 1.3 (`IndexerConfig`) social content containing:
  - English standalone post & 4-tweet thread focusing on **The Cost of Borrowed Config: Why Lifetimes Viral-Spread in Systems**.
  - Hinglish standalone post & 4-tweet thread focusing on **Devnet Se Mainnet Ka Safar & The "Hardcoded URL" Ki Tabahi**.
  - All tweets strictly verified under 280 characters.

**Why:** User requested generating social journey content for X (formerly Twitter) with each active exercise/module per Rule 25.
</details>

<details>
<summary>2026-10-07 — Cargo.toml, EXAMPLES.md, Rust-Decisions.md, hinglish-docs.md, EXERCISES.md — Initiate Module 1.3 (Configuration System & Ownership)</summary>

**Before:**
- `Cargo.toml` lacked `serde` and `toml`.
- `EXAMPLES.md` ended at Concept 1.2d.
- `Rust-Decisions.md` ended at Module 1.2d.
- `hinglish-docs.md` had Sections 1 to 7 (ending at Systems Cheatsheet).
- `EXERCISES.md` had no open exercise.

**After:**
- Added `serde = { version = "1.0", features = ["derive"] }` and `toml = "0.8"` to `Cargo.toml`.
- Added Concept 1.3 ELI5 (The Telegraph Transmission Dispatch Slip) and technical explanation to `EXAMPLES.md`.
- Added Module 1.3 Rust decisions (`String` vs `&str`, `Default` trait, `Option` overrides) to `Rust-Decisions.md`.
- Added Section 7 for Module 1.3 (with full 7-component structure) to `hinglish-docs.md`, updated TOC, and renumbered Cheatsheet to Section 8.
- Added Exercise 1.3 skeleton (`IndexerConfig` & `Default` trait implementation) to `EXERCISES.md` under `## Open / In-Progress`.

**Why:** Resumed curriculum per `/next` advancing to Module 1.3 per Rules 2, 6, 8, 11, 12, 13, 15, 17, 20, 24, and Step 3 / 3.5.
</details>

<details>
<summary>2026-10-07 — LEARNING.md, ROADMAP.md, RULES.md, .agents/rules/social.md, social.md — Complete Module 1.2 & Mandate Distinct Topics in Rule 25</summary>

**Before:**
- `LEARNING.md` line 26: `- [~] 1.2 — Domain Types: The Language of On-Chain Data`.
- `ROADMAP.md` Module 1.2 had unchecked `[ ]` boxes.
- `RULES.md` Rule 25 & `.agents/rules/social.md` allowed English and Hinglish content to cover the same points with only tonal differences.
- `social.md` Module 1.2 had both English and Hinglish discussing the exact same list of 4 domain structs.

**After:**
- Marked Module 1.2 as complete `[x]` in `LEARNING.md` line 26 and checked off all 4 requirement items in `ROADMAP.md`.
- Updated Rule 25 in `RULES.md` and `.agents/rules/social.md` strictly mandating that English and Hinglish content must explore **completely different technical topics and angles** (e.g. English dives into Rust low-level type systems and zero-cost abstractions; Hinglish explores Solana runtime quirks, consensus skip slots, and builder lessons).
- Rewrote `social.md` Module 1.2:
  - English focuses on **Primitive Obsession & Rust Zero-Cost Newtypes (`Slot(pub u64)`)**.
  - Hinglish focuses on **Solana Stateless Runtime Shock (`executable: true` vs Data Accounts) & Consensus Skip-Slot Detection**.
  - All tweets verified strictly under 280 characters.

**Why:** Learner confirmed passing `cargo test`, explicitly authorized marking Module 1.2 complete, and requested strict distinction in topics and takeaways between English and Hinglish social content per Rules 1, 2, 23, and 25.
</details>

<details>
<summary>2026-10-07 — SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md — Record Solution 1.2d and mark Exercise 1.2d solved</summary>

**Before:**
- `SOLUTIONS.md` and `SOLUTIONS_EXPLANATIONS.md` ended at Solution 1.2c.
- `EXERCISES.md` had Exercise 1.2d in `## Open / In-Progress` with `Status: open`.

**After:**
- Appended Solution 1.2d reference implementation, rationale, and comparison to `SOLUTIONS.md`.
- Appended Solution 1.2d thought translation and syntax breakdown to `SOLUTIONS_EXPLANATIONS.md`.
- Moved Exercise 1.2d to `## Solved` with `Status: solved` and recorded learner's working implementation in `EXERCISES.md`.

**Why:** Learner implemented `SlotInfo` and `Slot` tuple struct in `src/models/slot.rs`, exported them in `src/models/mod.rs`, verified passing `cargo test`, and requested advancement per Rules 2, 9, 19, 20, and Step 3.5-D.
</details>

<details>
<summary>2026-10-07 — social.md, RULES.md, .agents/workflows/next.md, .agents/rules/social.md — Establish Rule 25 & create social.md for X journey content</summary>

**Before:**
- `social.md` did not exist.
- `RULES.md` ended at Rule 24.
- `.agents/workflows/next.md` lacked social media tracking.
- `.agents/rules/social.md` did not exist.

**After:**
- Created `social.md` containing standalone posts and 4-tweet threads for Module 1.1 and Module 1.2 in both English and Hinglish with independent creative angles, strictly within 280 characters per tweet.
- Added Rule 25 to `RULES.md` mandating shareable journey content generation in `social.md` for X (Twitter) in both English and Hinglish with verified character limits and distinct angles.
- Updated `.agents/workflows/next.md` (STEP 3 item 12, STEP 4 working files, STEP 7 audit) to enforce Rule 25.
- Created `.agents/rules/social.md` with full directives and format guidelines.

**Why:** User requested public journey sharing on X (Twitter) with separate standalone posts and threads in both English and Hinglish (not identical translations) respecting the ~280 character limit.
</details>

<details>
<summary>2026-10-07 — hinglish-docs.md, .agents/rules/hinglish-docs.md — Standardize full 7-component Hinglish curriculum documentation</summary>

**Before:**
- `hinglish-docs.md` Section 6 lacked explicit Overview, Goal of this Step, Plain Thought Translation, and Skeleton TODO Guide.
- `.agents/rules/hinglish-docs.md` did not enumerate all required Hinglish subsections.

**After:**
- Enriched `hinglish-docs.md` Section 6 (`SlotInfo`) with Overview (Big Picture), Goal of this Step, Plain Thought Translation, and Skeleton TODO Guide. Excluded ASCII diagrams (retained purely in English).
- Updated `.agents/rules/hinglish-docs.md` to formally require the 7-component structure for every module's Hinglish documentation alongside parallel English teaching.

**Why:** User requested comprehensive self-sufficient Hinglish coverage so dual-referencing English sections isn't necessary for conceptual understanding and implementation tasks, while maintaining parallel English explanations and keeping ASCII diagrams in English only.
</details>

<details>
<summary>2026-10-07 — SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md, EXAMPLES.md, Rust-Decisions.md, hinglish-docs.md — Record Solution 1.2c and Initiate Exercise 1.2d (SlotInfo & Slot Tuple Struct)</summary>

**Before:**
- `SOLUTIONS.md` and `SOLUTIONS_EXPLANATIONS.md` ended at Solution 1.2b.
- `EXERCISES.md` had Exercise 1.2c in `## Open / In-Progress`.
- `EXAMPLES.md` ended at Concept 1.2c.
- `Rust-Decisions.md` ended at Module 1.2c.
- `hinglish-docs.md` had Sections 1 to 6 (ending at Systems Cheatsheet).

**After:**
- Recorded Solution 1.2c reference implementation and comparison in `SOLUTIONS.md`.
- Added Solution 1.2c thought translation and syntax breakdown in `SOLUTIONS_EXPLANATIONS.md`.
- Moved Exercise 1.2c to `## Solved` with learner's working attempt in `EXERCISES.md`.
- Added Exercise 1.2d skeleton (`Slot` tuple struct, `SlotInfo` domain struct) to `EXERCISES.md` under `## Open / In-Progress`.
- Added Concept 1.2d ELI5 (The Master Ledger Page Header) and technical explanation to `EXAMPLES.md`.
- Added Module 1.2d Rust decisions (tuple struct vs type alias, `Option<u64>`, `.0` indexing) to `Rust-Decisions.md`.
- Added Section 6 for Module 1.2d, shifted Cheatsheet to Section 7, and updated Table of Contents and Cheatsheet in `hinglish-docs.md`.

**Why:** Learner confirmed unit tests passed for Exercise 1.2c and requested `/next`. Recorded solution and initiated the final deliverable of Module 1.2 per Rules 2, 6, 8, 11, 19, 20, 24, and Step 3.5.
</details>

<details>
<summary>2026-10-07 — EXERCISES.md, EXAMPLES.md, Rust-Decisions.md, hinglish-docs.md — Initiate Exercise 1.2c (DecodedInstruction & Algebraic Enums)</summary>

**Before:**
- `EXERCISES.md` had no open exercise.
- `EXAMPLES.md` ended at Concept 1.2b.
- `Rust-Decisions.md` ended at Module 1.2b.
- `hinglish-docs.md` had Sections 1 to 5.

**After:**
- Added Exercise 1.2c skeleton (`DecodedInstruction` and `InstructionPayload` algebraic enum) to `EXERCISES.md` under `## Open / In-Progress`.
- Added Concept 1.2c ELI5 (The Itemized Dispatch Voucher) and technical breakdown to `EXAMPLES.md`.
- Added Module 1.2c Rust decisions (`InstructionPayload` algebraic enum, `Vec<Pubkey>`, `match` exhaustiveness) to `Rust-Decisions.md`.
- Added Section 5 for Module 1.2c and updated Systems Cheatsheet (Section 6) in `hinglish-docs.md`.

**Why:** Resumed Module 1.2 via `/next` after learner authorized progress and requested advancement to next exercise per Rule 6, 8, 11, 20, 24, and Step 3.5.
</details>

<details>
<summary>2026-10-07 — SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md — Record Solution 1.2b and mark Exercise 1.2b solved</summary>

**Before:**
- Exercise 1.2b was in `## Open / In-Progress` with `Status: open`.
- `SOLUTIONS.md` and `SOLUTIONS_EXPLANATIONS.md` had entries up to Solution 1.2.

**After:**
- Moved Exercise 1.2b to `## Solved` with `Status: solved` and recorded learner's working attempt in `EXERCISES.md`.
- Added Solution 1.2b reference implementation, rationale, and comparison to `SOLUTIONS.md`.
- Added Solution 1.2b thought translation and syntax breakdown to `SOLUTIONS_EXPLANATIONS.md`.

**Why:** Exercise 1.2b was successfully completed, verified, and gated solution unlocked per Rule 19 and Step 3.5.
</details>

<details>
<summary>2026-10-07 — EXERCISES.md, EXAMPLES.md, Rust-Decisions.md, hinglish-docs.md — Revert premature Exercise 1.2c additions</summary>

**Before:**
Prematurely injected Exercise 1.2c across working files without learner confirmation or module audit.

**After:**
Reverted Exercise 1.2c additions from `EXERCISES.md`, `EXAMPLES.md`, `Rust-Decisions.md`, and `hinglish-docs.md`. Kept `## Open / In-Progress` clear awaiting explicit module review and learner consent.

**Why:** Uphold Rule 1, Rule 23, and workflow discipline: require explicit learner alignment, permission, and module audit before generating new exercises.
</details>

<details>
<summary>2026-10-07 — LEARNING.md, Conversation.md — Mark Module 1.2 in-progress [~] and document milestone</summary>

**Before:**
`LEARNING.md` line 26:
```markdown
- [ ] 1.2 — Domain Types: The Language of On-Chain Data
```

**After:**
`LEARNING.md` line 26:
```markdown
- [~] 1.2 — Domain Types: The Language of On-Chain Data
```
Appended milestone and alignment session log to `Conversation.md`.

**Why:** Learner explicitly authorized status update to reflect verified completion of `AccountSnapshot` (Exercise 1.2) and `TransactionRecord` (Exercise 1.2b) per Rule 1, Rule 2, and Rule 5.
</details>

## 2026-10-05

<details>
<summary>2026-10-05 — hinglish-docs.md, RULES.md, next.md, Conversation.md — Calibrate Hinglish docs to Knowledgeable + Fun to Read blend</summary>

**Before:**
- Initial attempt stripped intuitive framing, creating dry, academic translations of technical terms into Hinglish without engaging flavor.
- Rule 24 and workflows did not clearly articulate the blend between engineering rigor and lively intuition.

**After:**
- Refined Rule 24 in `RULES.md` and `.agents/rules/hinglish-docs.md` to mandate a seamless blend of **deep technical knowledge + engaging, fun, conversational delivery**.
- Updated `.agents/workflows/next.md` (STEP 3 item 11).
- Overhauled `hinglish-docs.md` uniting real technical architecture (Solana 400ms slots, PoH consensus, RPC rate limits, stateless programs vs data accounts, owner program permissions, memory layouts, lifetimes, precision division, cryptographic signatures, sentinel hazard) directly with lively, memorable intuition (bullet train, overworked RPC node, newsroom wire ticker, standardized catalog card, clearinghouse slip).
- Logged session calibration in `Conversation.md`.

**Why:** User requested a balance where technical depth and lively, fun-to-read intuition are seamlessly woven together, avoiding dry academic dumps while preserving deep learning value.
</details>

## 2026-10-04

<details>
<summary>2026-10-04 — RULES.md, next.md, hinglish-docs.md, Conversation.md — Establish Rule 24 and create hinglish-docs.md</summary>

**Before:**
- Explanations were required in technical English with domain ELI5 analogies, but lacked a requirement for fun, intuitive Hinglish explanations.
- No central document existed to retain Hinglish conceptual walkthroughs and Rust intuition.

**After:**
- Added Rule 24 to `RULES.md` mandating dual English + Hinglish explanations for all concepts, architectures, exercises, and decisions, along with verbatim persistence into `hinglish-docs.md`.
- Updated Rule 20 in `RULES.md` to track `hinglish-docs.md`.
- Created `.agents/rules/hinglish-docs.md` to formalize the dual-explanation directive in workspace rules.
- Updated `.agents/workflows/next.md` (STEP 3, STEP 4, STEP 7, STEP 8) ensuring future turns uphold Rule 24.
- Created `hinglish-docs.md` retroactively capturing full Hinglish breakdowns for the Big Picture, Module 1.1 (Cluster Handshake & Wire Ticker), Module 1.2 (`AccountSnapshot` & Catalog Card), Module 1.2b (`TransactionRecord` & Clearinghouse Slip), and a consolidated Rust trade-off cheatsheet.
- Added session record to `Conversation.md`.

**Why:** User requested explanations in Hinglish alongside English to make technical concepts intuitive, relatable, and fun without cognitive fatigue, with all explanations stored permanently in `hinglish-docs.md`.
</details>

## 2026-09-26

<details>
<summary>2026-09-26 — RULES.md, next.md, Conversation.md — Establish Rule 23: Two-step module transition boundary</summary>

**Before:**
No formal separation rule preventing teaching the next module in the same response as marking the previous module complete.

**After:**
- Added Rule 23 to `RULES.md`: When a module is audited and approved for completion, update tracking files, report milestone, and stop. Explicitly ask if learner is ready for the next module. Never advance to teaching the next module in the same turn.
- Updated `.agents/workflows/next.md` (STEP 5 and STEP 8) enforcing this two-step gate and adding it as an explicit anti-pattern.
- Documented in `Conversation.md`.

**Why:** User requested a strict rule that module completion and next module initiation are never bundled in the same turn.
</details>

<details>
<summary>2026-09-26 — EXERCISES.md, EXAMPLES.md, Rust-Decisions.md — Add Exercise 1.2b, ELI5, and Rust Decisions</summary>

**Before:**
Module 1.2 was the latest entry across all files.

**After:**
- Added Exercise 1.2b (`TransactionRecord` struct, constructor, `Display` implementation, and unit test) to `EXERCISES.md`.
- Added Module 1.2b ELI5 (Clearinghouse Transfer Slip) and technical explanation to `EXAMPLES.md`.
- Added Module 1.2b Rust decisions (`Signature` vs `String`, `Option<i64>`, and `Display` vs `Debug`) to `Rust-Decisions.md`.

**Why:** Advanced to second domain type (`TransactionRecord`) within Module 1.2 per roadmap specifications.
</details>

<details>
<summary>2026-09-26 — SOLUTIONS.md & SOLUTIONS_EXPLANATIONS.md — Add Solution 1.2 and deep explanation</summary>

**Before:**
`SOLUTIONS.md` had Solution 1.1 only. `SOLUTIONS_EXPLANATIONS.md` had Solution 1.1 only.

**After:**
Added Solution 1.2 reference implementation, "why this & why not that" rationale, comparison against learner's attempt, thought translation, and syntax breakdown.

**Why:** Gated solution unlocked after learner successfully implemented and verified Exercise 1.2 per Rule 19 and Step 3.5.
</details>

<details>
<summary>2026-09-26 — EXERCISES.md — Mark Exercise 1.2 solved</summary>

**Before:**
Exercise 1.2 was under `## Open / In-Progress` with `Status: attempted`.

**After:**
Moved Exercise 1.2 to `## Solved` with `Status: solved` and finalized attempt code.

**Why:** Learner confirmed tests pass and solution is working.
</details>

<details>
<summary>2026-09-26 — Rust-Decisions.md — Add Decisions 5 and 6 (Semicolons & Method Calls)</summary>

**Before:**
Decisions 1 to 4 under Module 1.2.

**After:**
Added Decision 5 (Expression vs Statement / Omitting Semicolons) and Decision 6 (Method Call Parentheses `()` vs Field Access) under Module 1.2.

**Why:** Documented Rust return syntax and method invocation mechanics encountered during Exercise 1.2 per Rule 11.
</details>

<details>
<summary>2026-09-26 — EXERCISES.md — Record learner attempt for Exercise 1.2</summary>

**Before:**
Exercise 1.2 was marked `Status: open` with empty `My attempt:`.

**After:**
Marked Exercise 1.2 `Status: attempted` and recorded learner's `AccountSnapshot` struct, constructor, `sol_balance` method, and unit tests.

**Why:** Learner submitted attempt for Exercise 1.2 per Step 3.5.
</details>

<details>
<summary>2026-09-26 — Rust-Decisions.md, RULES.md, next.md, Conversation.md — Create Rust-Decisions.md and integrate into governance rules</summary>

**Before:**
`Rust-Decisions.md` did not exist. `RULES.md` and `next.md` did not track it as a required file.

**After:**
- Created `Rust-Decisions.md` containing all past and present "Why this & why not that" Rust syntax, architecture, and typing decisions verbatim (Modules 1.1 and 1.2).
- Updated `RULES.md` (Rules 11 and 20) to enforce writing every generated Rust decision into `Rust-Decisions.md` without independent paraphrasing.
- Updated `.agents/workflows/next.md` (Step 4) to include `Rust-Decisions.md` in file-edit discipline.
- Added session record to `Conversation.md`.

**Why:** User requested a dedicated file to track all Rust architectural and syntax decisions verbatim across the project.
</details>

## 2026-09-25

<details>
<summary>2026-09-25 — EXERCISES.md — Add Exercise 1.2 skeleton for AccountSnapshot</summary>

**Before:**
`## Open / In-Progress` was empty.

**After:**
Added Exercise 1.2 skeleton for defining `AccountSnapshot`, constructor, and lamports-to-SOL conversion method with unit test.

**Why:** Hands-on exercise for Module 1.2 domain type modeling per Rule 17 and Step 3.5.
</details>

<details>
<summary>2026-09-25 — EXAMPLES.md — Add ELI5 and technical explanation for Module 1.2</summary>

**Before:**
Module 1.1 was the latest concept.

**After:**
Added Module 1.2 ELI5 (Standardized Cataloguing Card) and technical explanation mapping to Solana's stateless program / account storage model.

**Why:** Preserved domain-consistent ELI5 library cataloguing analogy and technical explanation per Rule 8.
</details>

<details>
<summary>2026-09-25 — LEARNING.md & ROADMAP.md — Mark Module 1.1 complete and Day 1 in progress</summary>

**Before:**
`LEARNING.md`: Day 1 `[ ]`, Module 1.1 `[ ]`.
`ROADMAP.md`: Module 1.1 items marked `[ ]`.

**After:**
`LEARNING.md`: Day 1 marked `[~]` (in progress), Module 1.1 marked `[x]` (completed).
`ROADMAP.md`: Module 1.1 "You build", "Concepts", "Architecture", and "Deliverable" marked `[x]`.

**Why:** Learner confirmed explicit approval to mark Module 1.1 complete after successfully compiling, running, and verifying cluster connectivity handshake.
</details>

<details>
<summary>2026-09-25 — rust-toolchain.toml — Pin toolchain to stable</summary>

**Before:**
*(file did not exist)*

**After:**
Created `rust-toolchain.toml` with `channel = "stable"` per Module 1.1 deliverable.

**Why:** Required skeleton configuration file specifying the active Rust toolchain channel.
</details>

<details>
<summary>2026-09-25 — SOLUTIONS.md & SOLUTIONS_EXPLANATIONS.md — Add Solution 1.1 and deep explanation</summary>

**Before:**
`SOLUTIONS.md` empty. `SOLUTIONS_EXPLANATIONS.md` empty.

**After:**
Added Solution 1.1 reference implementation, "why this & why not that" rationale, comparison against learner's attempt, thought translation, and syntax breakdown.

**Why:** Gated solution unlocked after learner successfully implemented and verified Exercise 1.1 per Rule 19 and Step 3.5.
</details>

<details>
<summary>2026-09-25 — EXERCISES.md — Mark Exercise 1.1 solved</summary>

**Before:**
Exercise 1.1 was under `## Open / In-Progress` with `Status: attempted`.

**After:**
Moved Exercise 1.1 to `## Solved` with `Status: solved` and finalized attempt code.

**Why:** Learner confirmed solution is working.
</details>

<details>
<summary>2026-09-25 — EXERCISES.md — Record learner attempt for Exercise 1.1</summary>

**Before:**
```markdown
### Exercise 1.1 (Day 1) — Initializing RpcClient & Cluster Connectivity Handshake
**Status:** open
...
**My attempt:** *(paste here when ready, even if broken/partial)*
```

**After:**
```markdown
### Exercise 1.1 (Day 1) — Initializing RpcClient & Cluster Connectivity Handshake
**Status:** attempted
...
**My attempt:**
(Recorded learner's RpcClient handshake implementation)
```

**Why:** Learner submitted attempt for Exercise 1.1; marked status as attempted per Step 3.5.
</details>

## 2026-09-24

<details>
<summary>2026-09-24 — RULES.md & next.md & Conversation.md — Calibrate Rust teaching to 'Why this & not that' and prevent fatigue</summary>

**Before (RULES.md Rule 11):**
```markdown
11. **Extreme syntax-level explanation.** Line-by-line, exhaustive explanation of every line of code before providing it — every `&`, `*`, `mut`, `?`, trait bound, why a method is called where it is. Never assume the learner remembers syntax quirks.
```

**After (RULES.md Rule 11):**
```markdown
11. **Intuitive Rust Teaching ("Why this & not that") without cognitive fatigue.** The learner is relatively new to Rust and learning it along the way. Teach Rust mechanisms in context, focusing on *why we are using this specific approach, type, or pattern and why not an alternative* (e.g., `String` vs `&str`, `match` vs `if let`, ownership vs borrowing). Crucially, explain with high-signal, punchy intuition rather than exhaustive, pedantic line-by-line dumps of every trivial token. Prevent cognitive fatigue — make explanations crisp, memorable, and directly relevant to what's being built.
```

**Changes in .agents/workflows/next.md:**
Updated Step 3 point 10 to reflect the updated Rule 11 approach.

**Changes in Conversation.md:**
Logged discussion on learning style calibration, prioritizing intuitive conceptual trade-offs over academic line-by-line dumps.

**Why:** Learner explicitly requested teaching Rust along the way focusing on why specific constructs are chosen over alternatives, while avoiding overly detailed token-by-token dumps that cause cognitive fatigue.
</details>

<details>
<summary>2026-09-24 — EXERCISES.md — Add Exercise 1.1 skeleton for RPC client and cluster connectivity</summary>

**Before:**
```markdown
## Open / In-Progress

*(Empty — your first exercise lands here once Day 1 starts.)*
```

**After:**
```markdown
## Open / In-Progress

### Exercise 1.1 (Day 1) — Initializing RpcClient & Cluster Connectivity Handshake
**Status:** open
**Goal:** Verify toolchain, dependencies, and cluster connectivity by querying a live Solana RPC endpoint version.

**Skeleton:**
```rust
use solana_client::rpc_client::RpcClient;

fn main() {
    println!("==================================================");
    println!("          SOLANA INDEXER — PHASE 1 CLI            ");
    println!("==================================================");

    let rpc_url = "https://api.devnet.solana.com";
    println!("[*] Connecting to RPC endpoint: {}", rpc_url);

    // TODO(1): Instantiate a synchronous `RpcClient` using `RpcClient::new(rpc_url.to_string())`
    // TODO(2): Call `.get_version()` on the client to fetch the remote node version
    // TODO(3): Match on the Result:
    //          - On Ok(v), print "[+] Connected! Node version: {}" with the `solana_core` field
    //          - On Err(e), print "[-] Connection failed: {}" and call `std::process::exit(1)`
    todo!()
}
```

**Constraints:** Keep it synchronous (blocking `RpcClient`); do not use `async` or `tokio` yet.
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*
```

**Why:** Provided skeleton exercise for Module 1.1 cluster connectivity handshake per Rule 17 and Step 3.5.
</details>

<details>
<summary>2026-09-24 — EXAMPLES.md — Add ELI5 and technical explanation for Module 1.1</summary>

**Before:**
```markdown
---

*(Empty — concepts land here as Day 1 begins.)*
```

**After:**
```markdown
### 1.1 — Connecting to the Cluster (The Wire Ticker Handshake)

**ELI5 (domain analogy):**
> Imagine setting up a dedicated terminal in a busy financial newsroom to monitor incoming telegraph dispatches from a central stock exchange. Before you write any parsing rules, print fancy headlines, or file reports into drawers, the very first thing you must do is plug in the telegraph cable, turn on the power switch, ping the exchange's transmission tower, and wait for an acknowledgment signal. If the tower doesn't reply "healthy and operational," attempting to catalogue or read incoming paper tape is useless. Our indexer's entry point and RPC client handshake is that initial power-on and telegraph ping.

**Technical explanation:**
> In Solana indexing, an indexer never executes transactions itself; it observes state transitions produced by validator nodes. To read state, the indexer must first establish a communication channel with an RPC node via HTTP JSON-RPC using `solana_client::rpc_client::RpcClient`. Before initiating expensive queries or running backfills, the binary performs an initial probe—invoking `RpcClient::get_version()` to query the software version running on the node or `RpcClient::get_health()` to ensure the node is healthy and caught up to cluster slot tolerance. In Rust, this begins with configuring dependencies (`solana-client` and `solana-sdk`) inside `Cargo.toml`, setting up a clean single-binary entry point in `src/main.rs`, rendering a startup banner to stdout, and establishing an initial synchronous RPC connection.
```

**Why:** Established domain-consistent ELI5 telegraph ticker analogy and technical explanation for cluster connection per Rule 8.
</details>

<details>
<summary>2026-09-24 — Conversation.md — Create conversation & discussion log</summary>

**Before:**
*(file did not exist)*

**After:**
Created `Conversation.md` as a dedicated space for tracking chats, open questions, brainstorming, architectural debates, and discussion history across the Solana Indexer curriculum.

**Why:** User requested a dedicated file to ask questions, chat, and keep running discussion history throughout the project.
</details>

## 2026-09-22

<details>
<summary>2026-09-22 — github.md — Add Section 15: Real-world emergency scenarios and high-impact fixes</summary>

**Before:**
Sections 1–14 ending at GitHub collaboration best practices.

**After:**
Added Section 15 with 8 subsections:
1. Leaked secrets & private keys pushed to GitHub (`git-filter-repo`, BFG, emergency revocation, and force push warnings).
2. The 100MB giant file rejection trap by GitHub and Git LFS configuration.
3. Recovering from committing directly to `main` instead of a feature branch.
4. Escaping and understanding the "Detached HEAD" state safely.
5. Windows vs Linux CRLF vs LF line endings hell & `.gitattributes` renormalization.
6. Keeping an out-of-date feature branch in sync with `main` via merge vs rebase (`--force-with-lease`).
7. Windows file case-sensitivity rename bugs (`git mv`).
8. Advanced stashing (untracked files `-u`, inspecting without popping, and `git stash branch`).

**Why:** Documented mission-critical edge cases and emergency rescue workflows frequently encountered in collaborative and Solana/Rust development.
</details>

<details>
<summary>2026-09-22 — github.md — Expand guide with merge conflicts, rebase vs merge, reflog, bisect, tags, worktrees & collaboration</summary>

**Before:**
Sections 1–6 (Trees & HEAD, rollback scenarios, interactive rebase, cherry-pick, basic cheatsheet, basic GitHub actions).

**After:**
Sections 1–14 (Added merge conflict resolution with marker breakdowns, merge vs rebase comparison, squashing 5 commits into 1, `git reflog` disaster recovery, `git bisect` binary search debugging, semantic tags & releases, `git worktree` concurrent branch workflow, `git blame` & pickaxe search, untracked junk cleanup with `git clean`, and open-source GitHub collaboration with SSH authentication & upstream remotes).

**Why:** Comprehensive real-world Git and GitHub operational manual requested by learner covering advanced survival tools, debugging, and team collaboration workflows.
</details>

<details>
<summary>2026-09-22 — github.md — Create beginner-friendly Git & GitHub reference and actions guide</summary>

**Before:**
*(file did not exist)*

**After:**
*(Created `github.md` with sections on Git 3 trees, HEAD mental model, rollback scenarios, interactive rebase commit modification, cherry-picking, daily command cheatsheet, and GitHub Actions CI YAML configuration)*

**Why:** Provided an accessible, practical Git reference answering rollback scenarios, commit modification, cherry-picking, and CI/CD setup for the project.
</details>

<details>
<summary>2026-09-22 — .gitignore — Add comprehensive Rust, Solana, database, and IDE ignore patterns</summary>

**Before:**
```gitignore
/target
/HISTORY.md
```

**After:**
```gitignore
# Rust build artifacts
/target/
**/*.rs.bk
*.pdb

# Environment variables & secrets
.env
.env.*
!.env.example
*.pem
*.key

# Solana local wallet keypairs & credentials
id.json
*-keypair.json

# Local data & database directories
/data/
/postgres-data/
/redis-data/
*.db
*.sqlite
*.sqlite3
*.log

# Profiling, benchmarking & coverage
flamegraph.svg
perf.data*
criterion/

# IDE & Editor artifacts
.idea/
*.iml
.vscode/
*.swp
*.swo
*~

# Operating system files
.DS_Store
Thumbs.db
Desktop.ini

# Project specific
/HISTORY.md
```

**Why:** Added industry-standard ignore rules covering Rust target files, Solana keypairs/secrets, environment variables, local database directories, profiling outputs, and editor artifacts while keeping `/HISTORY.md`.
</details>

<details>
<summary>2026-09-22 — .agents/workflows/next.md — Harmonize workflow for Solana Indexer 7-day sprint</summary>

**Before:**
```markdown
# /next — Advance One Curriculum Step (Trading Platform, Rust)
1. RULES.md — 20 governance rules.
fn place_order(/* ... */) -> Result<Order, TradingError>
```

**After:**
```markdown
# /next — Advance One Curriculum Step (Solana Indexer, Rust — 7-Day Sprint)
1. RULES.md — 22 governance rules.
fn decode_account(/* ... */) -> Result<AccountSnapshot, IndexerError>
```

**Why:** Aligned the `/next` agent workflow with the Solana indexer domain, 22 governance rules, and 7-day sprint structure.
</details>

<details>
<summary>2026-09-22 — RULES.md — Harmonize sprint preamble and Rule 18</summary>

**Before:**
```markdown
> Same spirit as your Rust Mastery Roadmap's rules — reused deliberately so the workflow feels identical. This project is milestone-based (Phases → Modules), not day-boxed, matching how the Trading Platform Roadmap is structured. A few additions are indexer/Solana-specific (marked ⛓️).
18. 100% roadmap-to-code enforcement. Every concept, data structure, crate, and pattern listed under a day in ROADMAP.md must be actively coded, compiled, and run in src/.
```

**After:**
```markdown
> Same spirit as your Rust Mastery Roadmap's rules — reused deliberately so the workflow feels identical. Structured as an intensive 7-Day Sprint covering 3 Phases and 45 milestone-based Modules (Phases → Modules). A few additions are indexer/Solana-specific (marked ⛓️).
18. 100% roadmap-to-code enforcement. Every concept, data structure, crate, and pattern listed under a module/day in ROADMAP.md must be actively coded, compiled, and run in src/.
```

**Why:** United the 7-day sprint timeline with the 45 milestone-based modules without altering any rules.
</details>

<details>
<summary>2026-09-22 — ROADMAP.md — Add 7-Day Intensive Sprint Schedule table</summary>

**Before:**
```markdown
- **Deliverable** — what must be working, running, and verifiable after this module
- **Status** — `[ ]` / `[~]` / `[x]` / `[!]`

---

## PHASE 1 — Synchronous Foundations Through a Real CLI Indexer
```

**After:**
```markdown
- **Deliverable** — what must be working, running, and verifiable after this module
- **Status** — `[ ]` / `[~]` / `[x]` / `[!]`

### ⚡ 7-DAY INTENSIVE SPRINT SCHEDULE

| Day | Phase | Target Modules | Core Deliverable & Focus |
|:---|:---|:---|:---|
| **Day 1** | Phase 1 | **Modules 1.1 – 1.7** | **RPC & Decoding Foundations:** Setup, domain types, config, CLI, error handling, RPC client, SPL Token & System decoders |
| **Day 2** | Phase 1 | **Modules 1.8 – 1.15** | **Phase 1 Capstone:** In-memory engine, live polling loop, JSON persistence, backfill & checkpoint cursor, tests, documentation, working CLI indexer |
| **Day 3** | Phase 2 | **Modules 2.1 – 2.3** | **Async Ingestion & APIs:** Async Tokio conversion, WebSocket PubSub live stream, Axum REST + WebSocket event push API |
| **Day 4** | Phase 2 | **Modules 2.4 – 2.8** | **Production Storage & Pipeline:** PostgreSQL schema + migrations (`sqlx`), API key auth & rate limiting, Redis cache, multi-stage async concurrency, tracing with slot lag metrics |
| **Day 5** | Phase 2 | **Modules 2.9 – 2.15** | **Phase 2 Capstone:** Docker & compose stack, layered config, background reconciliation jobs, 3-tier cache, 5-crate workspace, criterion benchmarks, production API |
| **Day 6** | Phase 3 | **Modules 3.1 – 3.8** | **Advanced Core & Performance:** Reorg/fork/finality handling, plugin decoders (`dyn ProgramDecoder`), custom Geyser plugin (`cdylib`), Yellowstone gRPC, lock-free queues (`crossbeam`), zero-copy parsing, arena allocators, Miri-clean `unsafe` |
| **Day 7** | Phase 3 | **Modules 3.9 – 3.15** | **Phase 3 Capstone Platform:** Procedural macro IDL codegen (`syn`/`quote`), CQRS event bus, columnar ClickHouse analytics, WS fan-out at scale, sharding, flamegraphs, multi-program platform |

---

## PHASE 1 — Synchronous Foundations Through a Real CLI Indexer
```

**Why:** Embedded the 7-day sprint mapping to show which modules are targeted each day without altering any module contents.
</details>

<details>
<summary>2026-09-22 — LEARNING.md — Add 7-Day Sprint Progress tracking table</summary>

**Before:**
```markdown
> Module-level tracking here; see `ROADMAP.md` for each module's full "You build" / "Concepts" / "Architecture" detail to audit against before marking complete (Rule 14).

---

## Phase 1 — Synchronous Foundations Through a Real CLI Indexer
```

**After:**
```markdown
> Module-level tracking here; see `ROADMAP.md` for each module's full "You build" / "Concepts" / "Architecture" detail to audit against before marking complete (Rule 14).

---

## ⚡ 7-Day Sprint Progress

| Day | Target Modules | Status | Major Milestone |
|:---|:---|:---:|:---|
| **Day 1** | Modules 1.1 – 1.7 | `[ ]` | RPC & Decoding Foundations |
| **Day 2** | Modules 1.8 – 1.15 | `[ ]` | Phase 1 Capstone: CLI Indexer |
| **Day 3** | Modules 2.1 – 2.3 | `[ ]` | Async WebSocket Ingestion & Axum API |
| **Day 4** | Modules 2.4 – 2.8 | `[ ]` | PostgreSQL Storage & Multi-Stage Pipeline |
| **Day 5** | Modules 2.9 – 2.15 | `[ ]` | Phase 2 Capstone: Production Backend |
| **Day 6** | Modules 3.1 – 3.8 | `[ ]` | Advanced Core, Reorgs, Geyser & Yellowstone |
| **Day 7** | Modules 3.9 – 3.15 | `[ ]` | Phase 3 Capstone: High-Throughput Platform |

---

## Phase 1 — Synchronous Foundations Through a Real CLI Indexer
```

**Why:** Provided high-level daily sprint milestone tracking alongside the granular module-by-module checkboxes.
</details>

<details>
<summary>2026-09-22 — HISTORY.md — Initialize granular historical evolution log</summary>

**Before:**
*(empty file)*

**After:**
Added full contextual record documenting background, objectives, 7-day sprint alignment, verbatim file diffs, and architectural verification.

**Why:** Preserves a permanent, un-truncated audit trail of every governance and structural change in this workspace.
</details>
