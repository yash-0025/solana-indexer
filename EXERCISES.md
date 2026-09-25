# ✍️ EXERCISES.md — Hands-On Exercises (Skeleton + Hints)

> Every hands-on part of a day lands here as an exercise BEFORE any full solution exists.
> You write the code. The AI gives you a skeleton with the taught concept blanked out, plus tiered hints on request.
> Full solutions live in `SOLUTIONS.md` and are gated — nobody opens it until you've made a real attempt AND asked to see it.

**Status legend:** `open` (not attempted yet) · `attempted` (had a go, not yet checked) · `solved` (compared against SOLUTIONS.md and understood)

**Rules for this file**
1. Skeleton code only has the CURRENT concept blanked out (`todo!()` / `// TODO(n)`). Everything else — imports, unrelated function bodies, struct defs — is pre-filled so you're never blocked by unrelated syntax.
2. Hints are tiered (conceptual → structural → near-solution) and only given one at a time, on request. "Hints used" gets bumped each time.
3. Nobody (including the AI) opens `SOLUTIONS.md` for an exercise until you've made an actual attempt AND asked to see it.

---

## Entry Format

```
### Exercise <day>.<n> — <short title>
**Status:** open
**Goal:** one sentence — what this proves you can do.

**Skeleton:**
​```rust
// pre-filled context
fn example() -> Result<(), IndexerError> {
    // TODO(1): ...
    todo!()
}
​```

**Constraints:** what NOT to change (signatures, imports, deps).
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*
```

---

## Open / In-Progress

*(Empty — open exercises will appear here.)*

---

## Solved

### Exercise 1.1 (Day 1) — Initializing RpcClient & Cluster Connectivity Handshake
**Status:** solved
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
**My attempt:**
```rust
use solana_client::rpc_client::RpcClient;

fn main() {
    println!("===========================================");
    println!("          SOLANA INDEXER                   ");
    println!("===========================================");

    let rpc_url = "https://api.devnet.solana.com";
    println!("[*] connecting to RPC endpoint: {}", rpc_url);

    let rpc_client = RpcClient::new(rpc_url.to_string());

    match rpc_client.get_version() {
        Ok(v) => {
            println!("[+] Connected! Node version: {} ", v.solana_core);
        }
        Err(e) => {
            println!("[-] Connection Failed: {} ", e);
            std::process::exit(1);
        }
    }
}
```