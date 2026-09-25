# 🗒️ LOGS.md — Change Log

> Every edit to any file in this workspace gets a collapsible entry here with a before/after diff, per `RULES.md` rule 2. No exceptions, no silent batching.

---

## 2026-09-25

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
