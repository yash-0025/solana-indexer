# 🏛️ DECISIONS.md — Architecture Decision Records (ADRs)

> Every significant architectural decision made in this project is recorded here with: the decision, why it was chosen, alternatives considered, trade-offs, and future improvements.

---

## ADR-001: Ingestion Transport — WebSocket PubSub First, Geyser gRPC Later

**Date:** 2026-09-22
**Status:** Proposed (to be implemented Module 2.2, revisited Module 3.4)
**Module:** Ingestion stage

### Context
The indexer needs a live feed of on-chain activity for the target program. Solana offers several transports: polling RPC (used in Phase 1, Module 1.9, deliberately, to feel its limits), WebSocket JSON-RPC PubSub (`accountSubscribe`/`programSubscribe`/`logsSubscribe`), and Geyser plugin streaming (typically consumed via a gRPC feed such as Yellowstone, offered by RPC providers).

### Decision
Build Module 2.2's live ingestor on **WebSocket PubSub**. Treat **Geyser gRPC** as the documented, planned upgrade path (Module 3.4 in `ROADMAP.md`), not something to build in Phase 2.

### Why This Approach
1. **Learning-first:** WebSocket PubSub requires no extra infra beyond `solana-test-validator` and teaches the exact same "subscribe → decode → persist" shape the whole pipeline needs, without a third-party gRPC endpoint dependency.
2. **Clean swap point:** Because the pipeline architecture (Module 2.7) separates "ingestion" from "decode/persist" behind a channel, swapping the transport later (Module 3.4) means only touching the ingestion stage — the decoders and storage layer never change.
3. **Known limitation, documented on purpose:** WebSocket PubSub is known to drop/lag under load and lacks historical replay — this ADR exists specifically so that limitation is a written, understood trade-off rather than a surprise found in production.

### Alternatives Considered
| Alternative | Pros | Cons |
|---|---|---|
| Geyser gRPC (Yellowstone) from Day 1 | Production-grade, no drop/lag issues | Requires a provider account/infra before any Solana concept is learned; distracts from core account/tx model |
| Pure RPC polling (`getProgramAccounts` on an interval) | Simplest possible code | High latency, hammers RPC rate limits, no true "live" feel |
| WebSocket PubSub (chosen) | Zero extra infra, teaches subscribe/decode/persist shape directly | Known drop/lag risk under load, no historical replay |

### Trade-offs
- **Pro:** Fast to build, zero external dependency, directly reusable pattern.
- **Con:** Not what you'd ship to mainnet at scale without hardening or swapping transport.
- **Acceptable because:** The architecture is built so the swap is cheap later, and the limitation is explicit, not hidden.

### Future Improvements
- Module 3.4: Replace WebSocket ingestor with a Yellowstone gRPC client behind the same channel interface.
- Module 3.3: Write an actual Geyser plugin (server-side) rather than only consuming one.

---

*(New ADRs — e.g. Day 4's schema design, Day 7's capstone program choice — get added here as they're decided.)*