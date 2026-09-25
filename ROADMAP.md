# ⛓️ SOLANA INDEXER ROADMAP — Production-Grade Indexing Platform

> **Created:** 2026-09-22 · **Project:** solana-indexer · **Location:** wherever you scaffold it locally
> **Learner profile:** Full-stack engineer, just completed a 30-day Rust Mastery Roadmap (ownership, borrowing, lifetimes, generics, traits, smart pointers, async/Tokio, Axum, sqlx, testing) via a trading-platform project. New to Solana specifically.
> **Philosophy:** ONE project, continuously evolving — same as the trading platform. Every Rust and Solana concept exists because the indexer demands it. Never taught in isolation.
> **Rust edition target:** 2024 Edition · **Toolchain:** stable

---

## 🔒 GOVERNANCE RULES

Full rules live in `RULES.md` in this workspace — read that file before touching anything. Summary: no silent edits to `ROADMAP.md`/`LEARNING.md`, every change logged in `LOGS.md`, one concept at a time in project context, ELI5 + rigorous technical explanation for every concept (stored in `EXAMPLES.md`), exhaustive line-by-line code walkthroughs, system-design overview before any new subsystem, and full verification against the roadmap before a module is marked complete.

---

## 📐 HOW THIS ROADMAP IS STRUCTURED

This roadmap is organized into **3 Phases**, each containing **15 Modules** — 45 modules total. It's **milestone-based, not time-boxed**: each module is a meaningful feature addition to the indexer, and you move on when it's actually built, tested, and understood — not when a clock runs out.

Every module has:
- **You build** — the feature/subsystem being added
- **Concepts** — Rust and Solana concepts the project demands, taught just-in-time
- **Architecture** — system design, data flow, how real indexing infra does this
- **Deliverable** — what must be working, running, and verifiable after this module
- **Status** — `[ ]` / `[~]` / `[x]` / `[!]`

### ⚡ 7-DAY INTENSIVE SPRINT SCHEDULE

| Day | Phase | Target Modules | Core Deliverable & Focus |
|:---|:---|:---|:---|
| **Day 1** | Phase 1 | **Modules 1.1 – 1.7** | **RPC & Decoding Foundations:** Setup, domain types, config, CLI, error handling, RPC client, SPL Token & System decoders |
| **Day 2** | Phase 1 | **Modules 1.8 – 1.15** | **Phase 1 Capstone:** In-memory engine, live polling loop, JSON persistence, backfill & checkpoint cursor, tests, documentation, working CLI indexer |
| **Day 3** | Phase 2 | **Modules 2.1 – 2.3** | **Async Ingestion & APIs:** Async Tokio conversion, WebSocket PubSub live stream, Axum REST + WebSocket event push API |
| **Day 4** | Phase 2 | **Modules 2.4 – 2.8** | **Production Storage & Pipeline:** PostgreSQL schema + migrations (sqlx), API key auth & rate limiting, Redis cache, multi-stage async concurrency, tracing with slot lag metrics |
| **Day 5** | Phase 2 | **Modules 2.9 – 2.15** | **Phase 2 Capstone:** Docker & compose stack, layered config, background reconciliation jobs, 3-tier cache, 5-crate workspace, criterion benchmarks, production API |
| **Day 6** | Phase 3 | **Modules 3.1 – 3.8** | **Advanced Core & Performance:** Reorg/fork/finality handling, plugin decoders (dyn ProgramDecoder), custom Geyser plugin (cdylib), Yellowstone gRPC, lock-free queues (crossbeam), zero-copy parsing, arena allocators, Miri-clean unsafe |
| **Day 7** | Phase 3 | **Modules 3.9 – 3.15** | **Phase 3 Capstone Platform:** Procedural macro IDL codegen (syn/quote), CQRS event bus, columnar ClickHouse analytics, WS fan-out at scale, sharding, flamegraphs, multi-program platform |

---

## PHASE 1 — Synchronous Foundations Through a Real CLI Indexer

> **Goal:** Learn Solana's account/transaction/instruction model, decoding, and core indexing concepts through a fully synchronous, blocking CLI tool — no async yet, on purpose, same as the trading platform's Phase 1 stayed synchronous. You learn *what* an indexer does before you learn how to make it fast and concurrent.
>
> **End state:** A polished, tested, modular CLI indexer for one Solana program — polling-based live updates, historical backfill, JSON persistence, queryable via CLI. **Portfolio piece #1.**

---

### Module 1.1 — Project Setup & Cargo Fundamentals
- [x] **You build:** The project skeleton. `cargo new solana-indexer`, `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, a `main.rs` that prints an indexer banner and connects to a configured RPC endpoint to confirm connectivity.
- [x] **Concepts:** `cargo new`/`cargo check`/`cargo run`/`cargo build --release` · `Cargo.toml` vs `Cargo.lock` · Edition 2024 · `rustfmt` + `clippy` from minute one · `rust-toolchain.toml` · `solana-client`/`solana-sdk` as first dependencies · Program entry point
- [x] **Architecture:** Single-binary architecture, same reasoning as the trading platform — evolve into a workspace later (Module 2.13), not now.
- [x] **Deliverable:** Project compiles, runs, and successfully calls `get_version` or `get_health` against a local validator. `cargo fmt --check` and `cargo clippy -- -D warnings` pass.

---

### Module 1.2 — Domain Types: The Language of On-Chain Data
- [ ] **You build:** Core domain types modeling what an indexer actually indexes: `AccountSnapshot` (Pubkey, owner, lamports, data, slot), `TransactionRecord` (signature, slot, block_time, success), `DecodedInstruction` (program_id, accounts, decoded payload), `SlotInfo`.
- [ ] **Concepts:** Structs (named-field, tuple structs) · Enums as algebraic data types · `match` exhaustiveness · Deriving `Debug`, `Clone`, `PartialEq` · `impl` blocks, `Self::new()` constructor pattern · `&self` vs `&mut self` · `Pubkey`/`Signature` wrapper types from `solana-sdk` · Type aliases · `///` doc comments
- [ ] **Architecture:** Domain-Driven Design applied to indexing — why your types should model "what an indexer produces" independent of any one program, so Phase 3's multi-program support doesn't require rewriting these.
- [ ] **Deliverable:** All core types defined in `src/models/` with constructors, `Display` formatting, and basic unit tests.

---

### Module 1.3 — Configuration System
- [ ] **You build:** A configuration loader reading from `config.toml` with environment variable overrides. Settings: RPC URL, target program ID, commitment level, poll interval, output data directory.
- [ ] **Concepts:** Ownership deep dive — `String` vs `&str`, moves, clones · Borrowing: `&T` vs `&mut T` · The borrow checker as compile-time data-race prevention · `std::fs::read_to_string` · `toml` crate · `std::env::var` · `Option<T>` for optional config · `unwrap()` vs `unwrap_or` vs `unwrap_or_else`
- [ ] **Architecture:** Configuration hierarchy (file → env → defaults). Why an indexer's config must be swappable per-program without recompiling — this is what makes Phase 3's multi-program indexer possible later.
- [ ] **Deliverable:** Config loads from file with env var overrides. Unit tests for each fallback path.

---

### Module 1.4 — CLI Interface: The Indexer Terminal
- [ ] **You build:** A `clap`-based CLI with subcommands: `account <PUBKEY>`, `tx <SIGNATURE>`, `watch <PROGRAM_ID>` (polling loop, built in Module 1.9), `backfill <PROGRAM_ID> --since <SLOT>`, `stats`.
- [ ] **Concepts:** `clap` derive API · Module system: `mod`, `pub`, `pub(crate)`, file-based modules · `use` imports and re-exports · Visibility rules · Shadowing vs mutation
- [ ] **Architecture:** Command pattern — how CLIs map to indexer operations. How real indexer CLIs (e.g. a Geyser plugin's admin tool) are structured.
- [ ] **Deliverable:** Working CLI skeleton accepting all subcommands, printing placeholder responses.

---

### Module 1.5 — Error Handling: When RPC Calls Fail
- [ ] **You build:** A custom `IndexerError` enum covering: `RpcError`, `DecodeError`, `AccountNotFound`, `InvalidPubkey`, `RateLimited`, `ConfigError`, `StorageError`. Propagation through the entire call stack.
- [ ] **Concepts:** `Result<T, E>` as the alternative to exceptions · `?` operator and `From::from` desugaring · `panic!` vs `Result` · Custom error enums with `thiserror` · `anyhow` for application-level errors · `From`/`Into` for error conversion · Why `unwrap()`/`expect()` in an indexer's hot path is especially dangerous — one bad account silently killing the whole process
- [ ] **Architecture:** Error hierarchy design for infra that must never crash on one bad input. How a decode failure on *one* account should degrade to "skip and log", not "take down the indexer".
- [ ] **Deliverable:** `IndexerError` used across all modules. No `unwrap()` in non-test code. Descriptive errors for every failure path, including RPC timeouts.

---

### Module 1.6 — Solana RPC Client Fundamentals
- [ ] **You build:** A thin RPC wrapper (`src/rpc.rs`) around `solana-client::RpcClient` exposing `get_account`, `get_balance`, `get_signatures_for_address`, `get_transaction`, with retry-on-rate-limit built in. Wire it into the `account`/`tx` CLI subcommands.
- [ ] **Concepts:** The Solana account model — everything is an account, programs are stateless and own accounts, not the other way around · `Pubkey` (base58 ed25519) vs `Keypair` · Commitment levels: `processed`/`confirmed`/`finalized` — why an indexer reads at `confirmed`/`finalized` only, never `processed` · Transaction anatomy: `Message`, `Instruction`s, `AccountMeta` with `is_signer`/`is_writable` · Local validator (`solana-test-validator`) vs devnet/mainnet RPC rate limits
- [ ] **Architecture:** Why every real indexer wraps its RPC client in a retry/backoff layer from day one — rate limiting isn't an edge case, it's the default operating condition against any public RPC.
- [ ] **Deliverable:** `account`/`tx` subcommands print real decoded-at-the-byte-level-only (raw) data from local validator and devnet, retrying cleanly on 429s.

---

### Module 1.7 — Account & Instruction Decoding
- [ ] **You build:** A `decoders` module with a `trait AccountDecoder` and `trait InstructionDecoder`, implemented for: SPL Token `Mint`, SPL Token `TokenAccount`, System Program `Transfer`, SPL Token `Transfer`/`TransferChecked`. Wire into the CLI so output is fully human-readable.
- [ ] **Concepts:** Traits as behavior contracts, implementing a trait for multiple types · Borsh serialization — fixed-order fields, no self-describing schema · Anchor's 8-byte discriminator (`sha256("account:<Name>")[..8]`) vs manual instruction-index-first-byte convention (System/Token programs predate Anchor) · Associated Token Account PDA derivation (`find_program_address`) · Generic functions over `T: AccountDecoder`
- [ ] **Architecture:** Why decoding is modeled as a trait, not a big `match` statement — this is the exact seam Phase 3's plugin decoder architecture (Module 3.2) will later snap new programs into without touching this code.
- [ ] **Deliverable:** A real devnet USDC transfer decoded from scratch into `Transfer { from, to, amount }`, verified against a block explorer.

> ⚠️ **Budget extra time here.** Getting one decoder exactly right teaches the pattern for every decoder you'll ever write, including Phase 3's IDL-to-code-generation module.

---

### Module 1.8 — In-Memory Indexing Engine
- [ ] **You build:** An `Index` struct holding decoded state in `HashMap`/`BTreeMap`: accounts by Pubkey, transfers by mint, running aggregate stats (total volume, unique holders, largest transfer). Query methods using iterator chains.
- [ ] **Concepts:** Iterators — `Iterator` trait, laziness, consuming methods · Closures — `Fn`, `FnMut`, `FnOnce` · Chains: `.map()`, `.filter()`, `.fold()`, `.sum()`, `.collect()` · `HashMap` vs `BTreeMap` — unsorted vs sorted-by-key, when each fits (e.g. `BTreeMap<Slot, _>` for chronological iteration) · `Entry` API · Turbofish `::<>`
- [ ] **Architecture:** This is the first pass at "what does querying indexed data even look like" — a deliberately naive in-memory version before Module 2.4 replaces it with Postgres, so you understand exactly what the database needs to replace.
- [ ] **Deliverable:** `stats` subcommand prints live-computed aggregates from the in-memory index, populated by a one-shot backfill.

---

### Module 1.9 — Polling-Based "Live" Indexing Loop
- [ ] **You build:** A blocking `watch` loop: every N seconds, poll `getSignaturesForAddress` for new signatures since the last seen one, fetch + decode + insert into the `Index`. Simplest possible form of "live".
- [ ] **Concepts:** `loop` + `break`, blocking `std::thread::sleep` · Why polling has an inherent latency floor (poll interval) and a throughput ceiling (rate limits) · Cursor tracking (last-seen signature) as a plain `struct` field, kept deliberately simple here
- [ ] **Architecture:** This module exists specifically to make the *pain* of polling concrete — latency, wasted RPC calls when nothing changed, and the impossibility of true real-time — which is the exact motivation for Module 2.1's move to async WebSocket subscriptions. Don't optimize this; feel why it's insufficient.
- [ ] **Deliverable:** `watch` subcommand runs indefinitely, printing new decoded transfers as they're polled, with a measured average latency-to-detection logged.

---

### Module 1.10 — File Persistence: Saving Indexed State
- [ ] **You build:** JSON-based persistence for the entire `Index` — snapshot file loaded on startup, saved after every batch of writes, using atomic write-to-temp-then-rename to prevent corruption on crash mid-write.
- [ ] **Concepts:** `serde` + `serde_json` · `#[derive(Serialize, Deserialize)]` · Serde attributes: `#[serde(rename_all = "camelCase")]`, `#[serde(default)]`, `#[serde(skip)]` · Lifetimes in structs · File I/O with proper error propagation · `PathBuf` vs `Path` (owned vs borrowed)
- [ ] **Architecture:** Persistence strategies for indexers specifically — why JSON is fine for a single-program dev indexer but breaks down at scale (this sets up Module 2.4's Postgres migration as a felt need, not an arbitrary upgrade). Atomic writes as crash-recovery insurance.
- [ ] **Deliverable:** State survives process restarts. A killed-mid-write test proves no corruption. Round-trip serialization unit tests.

---

### Module 1.11 — Backfill & Checkpoint Cursor
- [ ] **You build:** A proper historical backfill that walks `getSignaturesForAddress` backward to a target slot/date, sharing a `Cursor` (last-indexed-slot) between the backfiller and the Module 1.9 watch loop so restarting never re-processes everything from scratch.
- [ ] **Concepts:** Smart pointers — `Rc<T>` for shared ownership of the `Cursor` between backfill and watch components · `RefCell<T>` for interior mutability (updating the cursor through a shared reference) · `Rc<RefCell<T>>` pattern · `Weak<T>` and when cyclic bookkeeping references would need it · When to reach for each smart pointer vs a plain `&mut`
- [ ] **Architecture:** Checkpointing as the foundation of "restart without redoing work or leaving a gap" — this exact cursor concept is what Module 2.7's async pipeline and Module 3.1's reorg handling both build directly on top of. Get the semantics right here while everything is still single-threaded and easy to reason about.
- [ ] **Deliverable:** Backfill-then-watch runs end to end; killing and restarting the process resumes from the checkpoint with zero gap and zero full-history duplication, verified by a test.

---

### Module 1.12 — Testing Suite
- [ ] **You build:** Comprehensive tests across all Phase 1 modules — unit tests, integration tests, doc tests. Fixtures for known-good decoded structs, mocked RPC responses for decoder tests that don't require network access.
- [ ] **Concepts:** `#[cfg(test)] mod tests` · `#[test]`, `assert!`, `assert_eq!`, `assert_ne!` · `#[should_panic]` · `Result`-returning tests · Integration tests in `tests/` · Doc tests in `///` comments · Test fixtures/builders for indexer domain objects
- [ ] **Architecture:** Testing strategy for indexers specifically — why decoder tests must never depend on live network state (byte-fixture based), while pipeline tests (backfill/checkpoint) need integration-level coverage against a local validator.
- [ ] **Deliverable:** 90%+ test coverage. Every public decoder has a doc test with a real byte fixture. Integration test for the full backfill → watch → persist flow.

---

### Module 1.13 — Multi-Module Architecture Refactoring
- [ ] **You build:** Refactor into clean module architecture: `src/models/`, `src/decoders/`, `src/rpc/`, `src/storage/`, `src/cli/`, `src/errors/`, `src/config/`.
- [ ] **Concepts:** `mod.rs` vs `foo.rs` + `foo/` (modern style) · `pub use` re-exports · `pub(crate)` vs `pub(super)` · Dependency direction — `decoders`/`storage` depend on `models`, never the reverse · Circular dependency prevention
- [ ] **Architecture:** Clean architecture applied to indexing infra. Why `models` having zero dependency on `rpc` or `storage` is what lets you swap Postgres in for JSON (Module 2.4) without touching a single decoder.
- [ ] **Deliverable:** Clean module tree, no circular dependencies, each module has one clear responsibility.

---

### Module 1.14 — Documentation & Code Quality
- [ ] **You build:** `rustdoc` for all public APIs, README with usage examples, clippy configuration, formatting rules.
- [ ] **Concepts:** `///` and `//!` doc comments · `cargo doc --open` · Intra-doc links · Doc-test code examples · `#![deny(missing_docs)]` · Clippy lint configuration · `rustfmt.toml` customization
- [ ] **Architecture:** Why documentation matters more for indexer decoders than almost anywhere else in the codebase — a decoder's doc comment is often the only record of *why* a byte offset is what it is, six months later.
- [ ] **Deliverable:** Complete API docs, README quickstart, all clippy warnings resolved.

---

### Module 1.15 — 🏁 Phase 1 Capstone: Portfolio-Ready CLI Indexer
- [ ] **You build:** Final polish pass — refactor, optimize, add missing tests, improve error messages, benchmark key operations (decode time, backfill throughput) with `std::time::Instant`, write a comprehensive README.
- [ ] **Concepts:** Review and solidify all Phase 1 concepts · Performance measurement basics · Code review checklist · Idiomatic Rust patterns review
- [ ] **Architecture:** Architecture review — what works, what's the natural next bottleneck (polling latency, JSON at scale, single-threaded decode), and how Phase 2 addresses each directly.
- [ ] **Deliverable:** A polished, tested, documented CLI indexer for one Solana program with live polling, backfill, checkpointing, and JSON persistence. **Portfolio piece #1.**

---

**🏁 Phase 1 Deliverables Summary:**
- Complete domain model for indexed on-chain data
- Solana account/transaction/instruction fundamentals
- Trait-based account & instruction decoders (Borsh + Anchor discriminators)
- In-memory indexing engine with aggregate queries
- Polling-based live updates + historical backfill
- Checkpoint cursor with crash-safe resume
- File-based persistence with serde
- Comprehensive test suite
- Clean multi-module architecture
- Full documentation
- **Portfolio-ready CLI indexer**

---

## PHASE 2 — Production Backend

> **Goal:** Transform the CLI indexer into a production-grade async service with real-time streaming, a database, an API, caching, observability, and containerization. Learn async Rust, Tokio, Axum, and production indexing infrastructure.
>
> **End state:** A fully functional indexing service with PostgreSQL, Redis, async WebSocket ingestion, a REST + WebSocket query API, Docker deployment, structured lag-aware observability, and benchmarks. **Portfolio piece #2.**

---

### Module 2.1 — Async Foundations: Why Indexers Need Async
- [ ] **You build:** Convert the core RPC/decode/store operations to async, and replace Module 1.9's blocking poll loop with an async task. Understand why an indexer watching multiple accounts/programs can't afford thread-per-poll.
- [ ] **Concepts:** Why async exists — I/O-bound concurrency vs CPU-bound parallelism · `async fn`, `.await`, `Future` trait · Futures as lazy state machines (not JS promises) · Why Rust needs an external runtime · Tokio: `#[tokio::main]`, `tokio::spawn`, tasks vs OS threads · `.await` as preemption points · `Send`/`Sync` bounds
- [ ] **Architecture:** Polling vs event-driven ingestion at scale. How production indexers watch dozens of programs concurrently without a thread per program.
- [ ] **Deliverable:** Core operations are async. The watch loop no longer blocks the whole process.

---

### Module 2.2 — Async Live Ingestion: WebSocket PubSub on Tokio
- [ ] **You build:** Replace polling entirely with `accountSubscribe`/`programSubscribe`/`logsSubscribe` over WebSocket. Automatic reconnect with exponential backoff. mpsc channel separating "receive from socket" from "decode and store".
- [ ] **Concepts:** Solana JSON-RPC PubSub methods and payload shapes · `tokio-tungstenite` (or a pubsub client crate) · Backoff + jitter reconnect patterns · mpsc fan-out so a slow decoder never blocks the socket reader · Why WebSocket PubSub alone still isn't fully production-grade (documented explicitly in `DECISIONS.md` — sets up Module 3.4's Geyser gRPC upgrade)
- [ ] **Architecture:** Event-driven ingestion replacing polling. Latency drops from "poll interval" to "sub-second", at the cost of a new failure mode (dropped/lagged subscriptions) that reconnect logic must handle.
- [ ] **Deliverable:** Indexer detects a new transfer within ~1s of it happening on-chain, and survives a manual validator restart via auto-reconnect.

---

### Module 2.3 — REST & WebSocket API: Serving Indexed Data
- [ ] **You build:** Axum-based API exposing indexed data: `GET /accounts/:pubkey`, `GET /transfers?mint=&owner=`, `GET /stats`, plus a `WS /stream` endpoint pushing newly-indexed events to connected clients in real time.
- [ ] **Concepts:** Axum design: `tokio` + `hyper` + `tower` · Routing, handlers, extractors (`Path`, `Query`, `Json`, `State`) · `Arc<AppState>` for shared state · Custom error type implementing `IntoResponse` · `tokio::sync::broadcast` for pushing live events to WS clients
- [ ] **Architecture:** Read API design for indexed data — how Helius/Solscan-style backends expose query endpoints. The mirror-image relationship between "subscribe to Solana" (Module 2.2) and "let clients subscribe to us" (this module).
- [ ] **Deliverable:** All indexed data queryable via REST; a connected WS client receives a push within ~1s of a new on-chain event.

---

### Module 2.4 — Database: PostgreSQL with Compile-Time Safety
- [ ] **You build:** Migrate from JSON file storage to PostgreSQL. Schema: `accounts` (latest snapshot per Pubkey), `transactions` (signature, slot, block_time, success), `transfers` (decoded event rows), `checkpoints` (last_indexed_slot). Migrations via `sqlx`.
- [ ] **Concepts:** `sqlx` — compile-time checked SQL, `query!`/`query_as!` macros · Connection pooling (`PgPool`) · Migrations (`sqlx migrate add`) · DB transactions · `FromRow` derive · Idempotent upserts (`INSERT ... ON CONFLICT DO UPDATE`) keyed on `(signature, instruction_index)` · Snapshot tables vs append-only event tables
- [ ] **Architecture:** Why you can't lose or duplicate an indexed event the way you (almost) can a cache entry. ACID guarantees for indexed data. Indexing strategy for the query patterns Module 2.3's API actually needs.
- [ ] **Deliverable:** Full Postgres integration, JSON storage removed, a simulated crash-and-restart proves zero duplicate rows.

---

### Module 2.5 — API Auth: Keys & Middleware
- [ ] **You build:** API-key based authentication for the query API (protecting admin/reindex-trigger endpoints; read endpoints can stay public or tiered). Per-key rate limiting.
- [ ] **Concepts:** `tower` middleware/layers · Custom extractors · `tower-http` layers: CORS, compression · Middleware ordering · Request guards
- [ ] **Architecture:** Why indexer APIs usually gate *write-adjacent* operations (manual reindex, backfill trigger) much more strictly than read queries, and rate-limit reads per key instead of blocking them outright.
- [ ] **Deliverable:** Protected admin endpoints reject unauthenticated requests with 401; public read endpoints are rate-limited per key.

---

### Module 2.6 — Redis: Caching Hot Queries & Rate Limiting
- [ ] **You build:** Redis integration caching hot read queries (e.g. "stats for the last 24h", "top holders") and backing the Module 2.5 rate limiter.
- [ ] **Concepts:** `redis` crate, async operations · Connection pooling (`deadpool-redis`/`bb8`) · TTL-based expiration · Cache-aside pattern · Cache invalidation on new writes
- [ ] **Architecture:** Why expensive aggregate queries (volume, top-N) get cached with a short TTL instead of recomputed on every request — the classic hot-path-vs-freshness tradeoff every indexing API makes.
- [ ] **Deliverable:** Cached stats endpoint measurably faster under repeated load; rate limiter backed by Redis instead of in-process memory.

---

### Module 2.7 — Concurrency: Multi-Stage Async Pipeline
- [ ] **You build:** Formalize ingestion → decode → persist into a proper multi-stage pipeline with `mpsc` channels between stages, a `Semaphore`-limited concurrent backfill (bounded concurrent `get_transaction` calls), and `Arc<RwLock<Cursor>>` replacing Module 1.11's single-threaded `Rc<RefCell<>>`.
- [ ] **Concepts:** `Arc<T>` — atomic reference counting · `Mutex<T>` vs `RwLock<T>` — exclusive vs read-heavy access · `tokio::sync::Mutex` vs `std::sync::Mutex` — when to use which · `tokio::sync::Semaphore` for bounded RPC concurrency · Channels: `mpsc`, `broadcast`, `watch` · `Send`/`Sync` marker traits
- [ ] **Architecture:** The backfill/live handoff problem, done properly this time: no gap, no explosion of duplicates, verified under concurrent load rather than single-threaded assumption.
- [ ] **Deliverable:** Pipeline backfills 24h of history concurrently (bounded), then hands off to live streaming with zero gap and zero duplicate signatures, verified by test.

---

### Module 2.8 — Tracing & Observability
- [ ] **You build:** Structured logging and tracing across the whole pipeline — request IDs, span contexts, and critically, a **lag metric**: how far behind the chain tip the indexer currently is, in slots and seconds.
- [ ] **Concepts:** `tracing` crate — spans, events, subscribers · `tracing-subscriber` configuration · `#[instrument]` · Structured fields vs string messages · JSON log output · Computing lag from `getSlot` vs `last_indexed_slot`
- [ ] **Architecture:** Why "lag" is the single most important number an indexer exposes — a technically-running indexer that's 40 minutes behind is often worse than one visibly down, because nothing tells consumers to distrust its data.
- [ ] **Deliverable:** Every pipeline stage traced with timing; `/health` reports current lag in slots and seconds; structured JSON logs throughout.

---

### Module 2.9 — Docker: Containerization & Deployment
- [ ] **You build:** Multi-stage `Dockerfile`, `docker-compose.yml` wiring Postgres + Redis + the ingestion pipeline + the API together.
- [ ] **Concepts:** Docker multi-stage builds for Rust (compile stage + slim runtime stage) · `docker-compose` multi-service orchestration · Env var configuration in containers · Health checks · Volume mounts for Postgres persistence
- [ ] **Architecture:** Why multi-stage builds shrink a Rust indexer image from ~2GB to tens of MB. Dev vs staging vs production compose profiles.
- [ ] **Deliverable:** `docker-compose up` brings up the entire stack; API reachable, DB persists across restarts.

---

### Module 2.10 — Configuration Management
- [ ] **You build:** Environment-aware config: dev/staging/production profiles, secrets kept out of version control, startup validation that fails fast on missing/invalid config (e.g. missing RPC URL or program ID).
- [ ] **Concepts:** `config` crate — layered configuration · `serde` deserialization for config · Builder pattern for config · `once_cell`/`LazyLock` for global config · Feature flags (`[features]`)
- [ ] **Architecture:** 12-factor-app principles applied to an indexer that needs to run identically against local validator, devnet, and mainnet with only config changing.
- [ ] **Deliverable:** Profile-based config; startup fails immediately and clearly on invalid config rather than failing mysteriously mid-run.

---

### Module 2.11 — Background Jobs & Scheduling
- [ ] **You build:** A background job runner for: periodic re-validation of recent slots (catching missed events), checkpoint cleanup, and a daily indexed-volume snapshot job.
- [ ] **Concepts:** `tokio::spawn` for background tasks · `tokio::time::interval` for periodic jobs · `tokio::select!` for cancellation · Graceful shutdown, `tokio::signal` for SIGTERM · `JoinSet` for managing multiple background tasks
- [ ] **Architecture:** Why indexers run periodic "did I miss anything" reconciliation jobs even with a solid live pipeline — sets up Module 3.1's deeper reorg-handling logic.
- [ ] **Deliverable:** Background job runner with configurable schedules, graceful shutdown on SIGTERM, job status visible via `/health`.

---

### Module 2.12 — Multi-Layer Caching Strategy
- [ ] **You build:** In-process LRU cache + Redis + Postgres, three-tier fallthrough for hot queries. Cache warming on startup for the most-queried data.
- [ ] **Concepts:** LRU cache implementation · `dashmap` for concurrent HashMap access · Cache-aside vs write-through · TTL strategy per tier · Basic metrics (hit/miss counts)
- [ ] **Architecture:** L1 (process) → L2 (Redis) → L3 (Postgres) fallthrough — how high-traffic indexing APIs keep p50 latency low without hitting the database on every request.
- [ ] **Deliverable:** Three-tier caching with automatic fallthrough and measured hit/miss rates under load.

---

### Module 2.13 — Workspace Architecture: Multi-Crate Project
- [ ] **You build:** Split the monolith into a Cargo workspace: `indexer-core` (domain types + decoders, zero framework deps), `indexer-ingest` (RPC/WS pipeline), `indexer-storage` (Postgres/Redis), `indexer-api` (Axum), `indexer-cli`.
- [ ] **Concepts:** Cargo workspaces — `[workspace]`, `[workspace.dependencies]` · Path dependencies · Crate boundaries and public APIs · Inter-crate testing
- [ ] **Architecture:** Why `indexer-core` having zero dependency on Tokio, Axum, or sqlx is what makes Module 3.2's plugin decoder system and Module 3.9's codegen both possible without core rewrites.
- [ ] **Deliverable:** Clean 5-crate workspace, each crate has one focused responsibility, shared types defined once in `indexer-core`.

---

### Module 2.14 — Benchmarking & Profiling
- [ ] **You build:** `criterion` benchmarks for hot paths: decode operations, checkpoint writes, aggregate query computation. Establish baselines.
- [ ] **Concepts:** `criterion` — statistical benchmarking, `cargo bench` · Benchmark groups/comparisons · `std::hint::black_box` · Profiling with `cargo flamegraph` · Identifying hot paths
- [ ] **Architecture:** Performance culture for indexing infra — why decode-per-second throughput is the number that determines whether your indexer can keep up with a busy program at all.
- [ ] **Deliverable:** Benchmark suite with baselines, flamegraph of the decode path, at least one optimization with measured before/after.

---

### Module 2.15 — 🏁 Phase 2 Capstone: Production-Ready Indexer API
- [ ] **You build:** Final production hardening — load testing the API, security review, API documentation (OpenAPI), deployment guide.
- [ ] **Concepts:** Review all Phase 2 concepts · API versioning · Rate limiting review · Input validation · Load testing with `hey`/`wrk`
- [ ] **Architecture:** Production readiness checklist for indexing infra specifically — what would a code review at Helius/Triton look for.
- [ ] **Deliverable:** A production-grade indexer with async streaming ingestion, Postgres, Redis caching, API keys, tracing with lag metrics, Docker deployment, and a benchmark suite. **Portfolio piece #2.**

---

**🏁 Phase 2 Deliverables Summary:**
- Async ingestion on Tokio, WebSocket PubSub replacing polling
- REST + WebSocket API with Axum
- PostgreSQL with compile-time checked SQL, idempotent upserts
- API key auth & per-key rate limiting
- Redis caching & multi-layer cache strategy
- Concurrent, semaphore-bounded backfill + live pipeline
- Structured tracing with a lag metric as the core health signal
- Docker deployment with compose
- Multi-crate workspace architecture
- Benchmark suite with profiling
- **Production-ready indexer API**

---

## PHASE 3 — Advanced Indexing Infrastructure & High-Throughput Concepts

> **Goal:** Build the infrastructure that makes an indexer feel like it belongs at Helius/Triton scale: rock-solid reorg handling, a plugin-based multi-program decoder architecture, real Geyser streaming (both consuming and writing a plugin), lock-free high-throughput pipelines, zero-copy parsing, and codegen from IDLs.
>
> **End state:** A complete, multi-program, high-throughput, fault-tolerant indexing platform. **Portfolio piece #3 — the final showpiece.**

---

### Module 3.1 — Reorg, Fork & Finality Handling: The Correctness Core
- [ ] **You build:** Explicit handling of slot skips, forks, and rollbacks. Track `processed`/`confirmed`/`finalized` state per indexed row; detect when a previously-indexed `confirmed` transaction gets orphaned by a fork and needs correcting.
- [ ] **Concepts:** Solana's fork choice and finality model in depth · Why `confirmed` isn't a permanent guarantee the way `finalized` is · Designing tables/logic that can *revise* a row instead of only ever appending · Reconciliation strategy: periodic re-check of recently-`confirmed`-but-not-yet-`finalized` data against canonical chain state
- [ ] **Architecture:** This is the indexer's equivalent of the trading platform's order book/matching engine module — the single hardest, most correctness-critical piece. Real indexing incidents are disproportionately "we indexed data that got reorged out and never corrected it."
- [ ] **Deliverable:** A test that forces a local-validator-style rollback scenario and proves the indexer detects and corrects the orphaned data rather than serving it forever.

> ⚠️ **This is the hardest module in the whole roadmap.** Budget real time. Get this wrong and everything downstream (API, analytics, capstone) inherits silently-wrong data.

---

### Module 3.2 — Plugin-Based Decoder Architecture: Multi-Program Indexing
- [ ] **You build:** Generalize Module 1.7's decoder traits into a runtime-registerable plugin system — `Vec<Box<dyn ProgramDecoder>>` — so a second and third program can be indexed by registering a new decoder, without touching the core pipeline.
- [ ] **Concepts:** Trait objects for pluggable decoders (`Box<dyn ProgramDecoder>`) · Dynamic dispatch vs the static dispatch used in Phase 1 — the deliberate tradeoff here (flexibility over raw speed) · Strategy pattern · Config-driven decoder registration per program ID
- [ ] **Architecture:** How real multi-program indexers (a Solscan-style "index everything" backend) stay maintainable — new integrations are additive, not a rewrite of the pipeline core.
- [ ] **Deliverable:** Indexer runs against two different programs simultaneously (e.g. SPL Token + one other), each routed to its own decoder purely via config.

---

### Module 3.3 — Writing a Geyser Plugin (Server-Side)
- [ ] **You build:** A minimal Geyser plugin (the `libloading`-based dynamic library a validator loads) implementing the plugin trait's `on_account_update`/`on_transaction` hooks, forwarding events out over a local IPC channel or gRPC to your indexer process.
- [ ] **Concepts:** The Geyser plugin interface, compiled as a `cdylib` a validator loads directly · Why plugin-side filtering (deciding what to forward before it ever leaves the validator process) is fundamentally lower-latency than any client-side subscription · FFI-adjacent concerns of a dynamically loaded Rust library
- [ ] **Architecture:** The difference between *consuming* a Geyser feed (Module 2.2, Module 3.4) and *producing* one — this module puts you on the other side of that relationship, which is where the real production ingestion infra actually lives.
- [ ] **Deliverable:** A working plugin loaded by your local validator, confirmed via logs that it's receiving and forwarding account updates in real time.

---

### Module 3.4 — Production Streaming: Yellowstone gRPC Ingestion
- [ ] **You build:** Replace/complement the Module 2.2 WebSocket ingestor with a Yellowstone gRPC client — the production-grade transport `DECISIONS.md`'s ADR-001 flagged as the real upgrade path.
- [ ] **Concepts:** Yellowstone gRPC client crate, subscription filters (by account, by program, by transaction) · Backpressure handling on a gRPC stream · Resubscription and stream-health monitoring · Comparing WS-drop-rate vs gRPC-drop-rate under sustained load
- [ ] **Architecture:** Why gRPC streaming from a Geyser-plugin-backed provider is what every serious commercial indexer actually runs on, and what WebSocket PubSub was always a stand-in for.
- [ ] **Deliverable:** Pipeline ingesting via Yellowstone gRPC with measurably lower lag and zero dropped events under a sustained load test, compared against the Module 2.2 baseline.

---

### Module 3.5 — Lock-Free, High-Throughput Concurrent Pipeline
- [ ] **You build:** Replace the Module 2.7 `mpsc`-based pipeline's hottest link with a lock-free SPSC/MPMC queue for decode-worker fan-out, and atomic counters for live throughput metrics.
- [ ] **Concepts:** `crossbeam` — scoped threads, lock-free channels/queues (`crossbeam::queue::SegQueue`) · `std::sync::atomic` — `AtomicU64`, memory `Ordering` (Relaxed/Acquire/Release/SeqCst) · Compare-and-swap · Why mutex contention becomes the bottleneck at high event throughput
- [ ] **Architecture:** Mechanical sympathy applied to indexing — why a busy program (thousands of events/sec) turns "a mutex around the index" from a non-issue into the entire bottleneck.
- [ ] **Deliverable:** Benchmark comparing the Module 2.7 mutex-based pipeline against the lock-free version under synthetic high-throughput load, with measured numbers.

---

### Module 3.6 — Zero-Copy Account & Instruction Parsing
- [ ] **You build:** A zero-copy variant of the Module 1.7 decoders that borrows directly from the incoming network/account buffer instead of allocating new `String`/`Vec` for every decoded field.
- [ ] **Concepts:** Lifetime-scoped parsed structs borrowing `&[u8]` · `zerocopy` crate · Endianness and alignment handling · The allocation cost of the "easy" decoders from Phase 1, made visible via benchmark
- [ ] **Architecture:** Why every allocation in the decode hot path costs real microseconds at scale, and how zero-copy parsing is the direct analog of the trading platform's zero-copy config parser module — same Rust concept, different domain payoff.
- [ ] **Deliverable:** Zero-copy decoder benchmarked against the Phase 1 allocating version, with measured allocation and latency reduction.

---

### Module 3.7 — Memory Optimizations: Arenas & Object Pools for the Decode Hot Path
- [ ] **You build:** An arena allocator (or object pool) for short-lived decoded-instruction structs in the hot path, avoiding per-event heap allocation churn.
- [ ] **Concepts:** Arena allocation pattern · Object pool pattern · `#[repr(C)]`/alignment for predictable layout · Stack vs heap allocation strategy in a tight loop
- [ ] **Architecture:** Where allocation-avoidance actually pays off vs where it's premature — profiling first (tying back to Module 2.14), optimizing the proven bottleneck only.
- [ ] **Deliverable:** Arena-backed decode path benchmarked against the standard-allocator version under sustained throughput.

---

### Module 3.8 — Unsafe Rust: When Safety Isn't Enough
- [ ] **You build:** A small, heavily-commented `unsafe` block inside the arena allocator from Module 3.7, with every invariant documented and validated under Miri.
- [ ] **Concepts:** `unsafe` blocks/`unsafe fn` — what invariants you're personally promising · Raw pointers `*const T`/`*mut T` · Dereferencing, `unsafe impl Send/Sync` · Miri for undefined-behavior detection
- [ ] **Architecture:** Where `unsafe` is actually justified in production indexing infra (the same small-surface-area, safe-public-API pattern `bytes`/`tokio` use), and where it's just "shut up the borrow checker" — the latter is never acceptable.
- [ ] **Deliverable:** Documented `unsafe` block with written safety proof, Miri-clean, measured performance delta vs the safe version.

---

### Module 3.9 — Procedural Macros: IDL → Decoder Code Generation
- [ ] **You build:** A procedural macro (or build-script codegen) that takes an Anchor IDL JSON file and generates the Rust decoder struct + discriminator-matching code automatically — the thing that made Module 1.7 and Module 3.2 tedious by hand.
- [ ] **Concepts:** Declarative macros (`macro_rules!`) as a stepping stone · Procedural derive macros, `syn` and `quote` crates · `proc-macro2` · Reading an IDL's account/instruction schema programmatically · `cargo expand` for debugging generated code
- [ ] **Architecture:** This is the module that makes Module 3.2's "just register a new program" claim actually cheap — onboarding a new Anchor program stops being "hand-write a decoder" and becomes "point codegen at its IDL."
- [ ] **Deliverable:** Feed the macro/codegen tool a real Anchor program's IDL; it generates a working decoder that matches Module 1.7's hand-written USDC decoder in correctness, verified against the same fixture.

---

### Module 3.10 — Event-Driven Architecture & CQRS for Indexed Data
- [ ] **You build:** An internal event bus (`AccountUpdated`, `TransferIndexed`, `CheckpointAdvanced`) distributed via `tokio::sync::broadcast` to multiple independent consumers: the DB writer, the cache invalidator, the WS fan-out, and an analytics sink (Module 3.11).
- [ ] **Concepts:** Event sourcing concepts · Enum-based typed events · Event handlers as trait implementations · CQRS — separating the write path (event log) from read models (denormalized query tables) · Event replay for rebuilding a read model from scratch
- [ ] **Architecture:** Why decoupling "a transfer was indexed" from "what happens next" lets you add a new consumer (say, a webhook notifier) without touching the pipeline core — the event-driven analog of Module 3.2's plugin decoders, but for the output side instead of the input side.
- [ ] **Deliverable:** At least 3 independent consumers subscribed to the same event stream, one of them addable/removable at runtime via config with zero pipeline changes.

---

### Module 3.11 — Columnar Analytics Storage for Scale
- [ ] **You build:** Route high-volume event data (every transfer, every swap) to a columnar/analytical store (ClickHouse or TimescaleDB) alongside Postgres, which stays the source of truth for current-state lookups.
- [ ] **Concepts:** Polyglot persistence — why the "current state" query pattern (Postgres, row-oriented, indexed by Pubkey) and the "aggregate over millions of historical events" pattern (columnar, analytical) want fundamentally different storage engines · Writing to both stores from the Module 3.10 event bus without duplicating pipeline logic
- [ ] **Architecture:** How real indexing analytics products (volume charts, top-N-over-time) stay fast at scale — never by scanning a row-oriented events table with millions of rows for every dashboard load.
- [ ] **Deliverable:** A "volume over the last 30 days" query answered from the columnar store in milliseconds against a dataset too large for a comfortable equivalent Postgres scan, benchmarked side by side.

---

### Module 3.12 — WebSocket Fan-Out at Scale
- [ ] **You build:** Harden Module 2.3's WS push endpoint to handle hundreds/thousands of concurrent subscribed clients efficiently, with per-client filtering (e.g. "only transfers for mint X") and backpressure handling for slow clients.
- [ ] **Concepts:** Efficient broadcast fan-out patterns beyond a naive `broadcast` channel per client · Per-subscription filtering without re-decoding per client · Backpressure and slow-consumer handling (drop vs buffer vs disconnect policy) · Connection lifecycle, heartbeat/ping-pong at scale
- [ ] **Architecture:** How exchanges/indexers handle 10k+ concurrent WebSocket subscribers on real-time feeds without the fan-out itself becoming the bottleneck.
- [ ] **Deliverable:** Load test with hundreds of simulated concurrent WS clients with varied filters, all receiving correctly-filtered events with bounded latency.

---

### Module 3.13 — Sharding & Horizontal Scaling the Indexer
- [ ] **You build:** Partition indexing work across multiple indexer instances — by program, or by slot-range for backfill — coordinated via a distributed lock (Postgres advisory locks or a Redis lease) so instances don't duplicate work.
- [ ] **Concepts:** Work partitioning strategies (by program vs by account-range vs by slot-range) · Distributed locking/leases for coordination · Idempotent merge of results from multiple shards back into shared storage
- [ ] **Architecture:** How an indexer scales past what one process can handle — parallel backfill across historical slot ranges, or dedicated instances per high-volume program, without two instances ever double-writing the same event.
- [ ] **Deliverable:** Two indexer instances run concurrently against different slot ranges (or different programs) and merge correctly into one shared database with zero duplicate/conflicting writes.

---

### Module 3.14 — Profiling & Performance Analysis
- [ ] **You build:** Full-system performance analysis: flamegraphs across the entire pipeline (not just decode), cache-miss and branch-prediction awareness on the hottest loops, `criterion` micro-benchmarks tying it together.
- [ ] **Concepts:** `cargo flamegraph`, `perf stat`/`perf record` · CPU cache hierarchy and why data layout affects cache misses · Branch prediction basics · Amdahl's law — why optimizing an already-fast stage barely moves total throughput
- [ ] **Architecture:** Performance engineering methodology: profile → identify → hypothesize → fix → measure, applied end to end across ingestion, decode, and storage rather than one module in isolation.
- [ ] **Deliverable:** A flamegraph-driven performance report identifying the true top-3 system-wide bottlenecks (not assumed ones), each addressed with a measured before/after.

---

### Module 3.15 — 🏁 Phase 3 Capstone: Complete Multi-Program Indexing Platform
- [ ] **You build:** Final integration of everything — plugin decoders (Module 3.2) for at least 2-3 real programs, Geyser gRPC ingestion (Module 3.4), reorg-safe correctness (Module 3.1), lock-free high-throughput pipeline (Module 3.5), event-driven fan-out to Postgres + columnar analytics + WS clients, sharded backfill. Load test the whole system. Full architecture documentation.
- [ ] **Concepts:** System integration · End-to-end and load testing · Performance regression testing · Architecture documentation · Deployment automation
- [ ] **Architecture:** Complete system architecture review — what would need to change for actual mainnet-scale production deployment, and what you've already built that genuinely wouldn't.
- [ ] **Deliverable:** A complete, multi-program, high-throughput, reorg-safe, horizontally-scalable indexing platform with full documentation and a performance report. **Portfolio piece #3 — the final showpiece.**

---

**🏁 Phase 3 Deliverables Summary:**
- Reorg/fork/finality-safe indexing correctness
- Plugin-based multi-program decoder architecture
- A real, working Geyser plugin (server-side)
- Production Yellowstone gRPC streaming ingestion
- Lock-free, high-throughput concurrent pipeline
- Zero-copy account/instruction parsing
- Arena/object-pool memory optimization for the decode hot path
- Justified, documented, Miri-clean `unsafe` Rust
- Procedural macro IDL-to-decoder code generation
- Event-driven architecture with CQRS
- Columnar analytics storage alongside Postgres
- WebSocket fan-out at scale
- Sharded, horizontally-scaled indexing
- Flamegraph-driven, system-wide performance tuning
- **Complete, production-grade, multi-program indexing platform**

---

## 📊 Concepts by Phase (Cross-Reference)

| Phase | Concepts Covered |
|---|---|
| **Phase 1** | Variables, types, structs, enums, match, ownership, borrowing, lifetimes intro, traits (decoders), error handling, Result/Option, iterators, closures, HashMap/BTreeMap, serde, smart pointers (Rc/RefCell/Weak), modules, testing, documentation, clippy/fmt, Solana account/tx/instruction model, Borsh, Anchor discriminators, blocking RPC calls |
| **Phase 2** | async/await, Future trait, Tokio runtime, WebSocket PubSub, Axum, sqlx, Arc, Mutex, RwLock, Semaphore, channels (mpsc/broadcast), Send/Sync, tracing, lag metrics, Docker, feature flags, Cargo workspaces, criterion benchmarking, tower middleware, Redis caching |
| **Phase 3** | Reorg/finality handling, trait objects & dynamic dispatch (plugin decoders), Geyser plugin development, Yellowstone gRPC, lock-free structures & atomics, memory ordering, zero-copy parsing, arena allocators, unsafe Rust + Miri, procedural macros (`syn`/`quote`), event-driven architecture & CQRS, columnar/polyglot storage, WebSocket fan-out at scale, distributed locking & sharding |
