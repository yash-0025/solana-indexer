# 🗒️ LOGS.md — Change Log

> Every edit to any file in this workspace gets a collapsible entry here with a before/after diff, per `RULES.md` rule 2. No exceptions, no silent batching.

---

## 2026-09-22

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
