# 📖 LEARNING.md — Living Progress Journal

> Source of truth for what's actually been learned, built, and understood. `ROADMAP.md` is the plan; this is the reality. Status markers: `[ ]` not started · `[~]` in progress · `[x]` done & understood · `[!]` done but shaky.
>
> Module-level tracking here; see `ROADMAP.md` for each module's full "You build" / "Concepts" / "Architecture" detail to audit against before marking complete (Rule 14).

---

## ⚡ 7-Day Sprint Progress

| Day | Target Modules | Status | Major Milestone |
|:---|:---|:---:|:---|
| **Day 1** | Modules 1.1 – 1.7 | [~] | RPC & Decoding Foundations |
| **Day 2** | Modules 1.8 – 1.15 | [ ] | Phase 1 Capstone: CLI Indexer |
| **Day 3** | Modules 2.1 – 2.3 | [ ] | Async WebSocket Ingestion & Axum API |
| **Day 4** | Modules 2.4 – 2.8 | [ ] | PostgreSQL Storage & Multi-Stage Pipeline |
| **Day 5** | Modules 2.9 – 2.15 | [ ] | Phase 2 Capstone: Production Backend |
| **Day 6** | Modules 3.1 – 3.8 | [ ] | Advanced Core, Reorgs, Geyser & Yellowstone |
| **Day 7** | Modules 3.9 – 3.15 | [ ] | Phase 3 Capstone: High-Throughput Platform |

---

## Phase 1 — Synchronous Foundations Through a Real CLI Indexer

- [x] 1.1 — Project Setup & Cargo Fundamentals
- [ ] 1.2 — Domain Types: The Language of On-Chain Data
- [ ] 1.3 — Configuration System
- [ ] 1.4 — CLI Interface: The Indexer Terminal
- [ ] 1.5 — Error Handling: When RPC Calls Fail
- [ ] 1.6 — Solana RPC Client Fundamentals
- [ ] 1.7 — Account & Instruction Decoding
- [ ] 1.8 — In-Memory Indexing Engine
- [ ] 1.9 — Polling-Based "Live" Indexing Loop
- [ ] 1.10 — File Persistence: Saving Indexed State
- [ ] 1.11 — Backfill & Checkpoint Cursor
- [ ] 1.12 — Testing Suite
- [ ] 1.13 — Multi-Module Architecture Refactoring
- [ ] 1.14 — Documentation & Code Quality
- [ ] 1.15 — 🏁 Phase 1 Capstone: Portfolio-Ready CLI Indexer

## Phase 2 — Production Backend

- [ ] 2.1 — Async Foundations: Why Indexers Need Async
- [ ] 2.2 — Async Live Ingestion: WebSocket PubSub on Tokio
- [ ] 2.3 — REST & WebSocket API: Serving Indexed Data
- [ ] 2.4 — Database: PostgreSQL with Compile-Time Safety
- [ ] 2.5 — API Auth: Keys & Middleware
- [ ] 2.6 — Redis: Caching Hot Queries & Rate Limiting
- [ ] 2.7 — Concurrency: Multi-Stage Async Pipeline
- [ ] 2.8 — Tracing & Observability
- [ ] 2.9 — Docker: Containerization & Deployment
- [ ] 2.10 — Configuration Management
- [ ] 2.11 — Background Jobs & Scheduling
- [ ] 2.12 — Multi-Layer Caching Strategy
- [ ] 2.13 — Workspace Architecture: Multi-Crate Project
- [ ] 2.14 — Benchmarking & Profiling
- [ ] 2.15 — 🏁 Phase 2 Capstone: Production-Ready Indexer API

## Phase 3 — Advanced Indexing Infrastructure & High-Throughput Concepts

- [ ] 3.1 — Reorg, Fork & Finality Handling: The Correctness Core
- [ ] 3.2 — Plugin-Based Decoder Architecture: Multi-Program Indexing
- [ ] 3.3 — Writing a Geyser Plugin (Server-Side)
- [ ] 3.4 — Production Streaming: Yellowstone gRPC Ingestion
- [ ] 3.5 — Lock-Free, High-Throughput Concurrent Pipeline
- [ ] 3.6 — Zero-Copy Account & Instruction Parsing
- [ ] 3.7 — Memory Optimizations: Arenas & Object Pools
- [ ] 3.8 — Unsafe Rust: When Safety Isn't Enough
- [ ] 3.9 — Procedural Macros: IDL → Decoder Code Generation
- [ ] 3.10 — Event-Driven Architecture & CQRS for Indexed Data
- [ ] 3.11 — Columnar Analytics Storage for Scale
- [ ] 3.12 — WebSocket Fan-Out at Scale
- [ ] 3.13 — Sharding & Horizontal Scaling the Indexer
- [ ] 3.14 — Profiling & Performance Analysis
- [ ] 3.15 — 🏁 Phase 3 Capstone: Complete Multi-Program Indexing Platform

---

## Shaky / Needs Revisit
*(Flag anything marked `[!]` above here with a one-line note on what's shaky.)*

- *(none yet)*
