# ⛓️ Solana Indexer — Mastery Build

A production-grade, multi-program Solana indexer built from scratch in Rust, structured as 3 Phases × 15 Modules (45 total): a synchronous CLI foundation, an async production backend (Postgres, Redis, Axum, Docker), and advanced infrastructure (reorg handling, a real Geyser plugin, lock-free pipelines, zero-copy parsing, procedural macro codegen, sharding).

Built as a companion project to the 30-Day Rust Mastery Roadmap (trading-platform project), reusing the same project-based, one-concept-at-a-time, milestone-driven teaching workflow.

## Workspace files

| File | Purpose |
|---|---|
| `ROADMAP.md` | The 3-phase, 45-module curriculum — source of the plan |
| `RULES.md` | Governance rules the AI tutor follows every session |
| `KICKOFF_PROMPT.md` | Paste into a fresh chat to start/resume |
| `LEARNING.md` | Living progress journal — source of truth for what's actually done |
| `EXERCISES.md` | Skeleton exercises with `todo!()` blanks |
| `SOLUTIONS.md` | Gated reference solutions (only after a real attempt) |
| `EXAMPLES.md` | ELI5 + technical explanation for every concept taught |
| `DECISIONS.md` | Architecture decision records |
| `LOGS.md` | Every file edit, logged with before/after |
| `NOTES.md` | Freeform notes and gotchas |
| `PROMPTS.md` | Reusable prompt snippets |
| `QUESTIONS.md` | Parking lot for questions that would derail the current concept |

## Setup

```bash
# Toolchain
rustc --version   # confirm stable, matches rust-toolchain.toml

# Solana CLI + local validator
solana --version
solana-test-validator

# Postgres (Day 4 onward)
docker run -d --name indexer-pg -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:16
```

Project crates get scaffolded as the roadmap reaches them (`cargo new`), starting Day 1 with `solana-explorer`.

## Status

See `LEARNING.md` for current phase/module and per-topic status.