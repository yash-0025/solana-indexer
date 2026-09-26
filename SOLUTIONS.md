# 🔐 SOLUTIONS.md — Gated Reference Solutions

> **Read this only after you've attempted the matching exercise in `EXERCISES.md` and explicitly asked to see the solution.**
> Reading ahead defeats the point — the exercise is where the actual learning happens, not the solution.
>
> Gate conditions before any entry is revealed:
> 1. You've pasted/described a real attempt (even a broken or partial one).
> 2. You've explicitly asked to see the solution.

**Entry numbering matches `EXERCISES.md` exactly** — Exercise 1.1 → Solution 1.1, etc.

---

## Entry Format

```
### Solution <day>.<n> — <short title>

**Reference implementation:**
​```rust
fn example() -> Result<(), IndexerError> {
    // ...
    Ok(())
}
​```

**Line-by-line:**
- explanation of each line/decision

**Compared to your attempt:**
- **Matches**: what you got right
- **Difference**: what differed and why
```

### Solution 1.1 — Initializing RpcClient & Cluster Connectivity Handshake

**Reference implementation:**
```rust
use solana_client::rpc_client::RpcClient;

fn main() {
    println!("==================================================");
    println!("          SOLANA INDEXER — PHASE 1 CLI            ");
    println!("==================================================");

    let rpc_url = "https://api.devnet.solana.com";
    println!("[*] Connecting to RPC endpoint: {}", rpc_url);

    let rpc_client = RpcClient::new(rpc_url.to_string());

    match rpc_client.get_version() {
        Ok(v) => {
            println!("[+] Connected! Node version: {}", v.solana_core);
        }
        Err(e) => {
            println!("[-] Connection failed: {}", e);
            std::process::exit(1);
        }
    }
}
```

**Why this & why not that:**
- `rpc_url.to_string()`: `RpcClient::new` requires an owned `String` so it owns its endpoint configuration without lifetime dependencies (`'a`).
- `match rpc_client.get_version()`: Network I/O yields `Result<T, E>`. `match` makes failure handling non-negotiable; `.unwrap()` would crash on transient network glitches.
- `std::process::exit(1)`: Signals explicit process failure to OS / Docker supervisors instead of silently hanging or exiting with 0.

**Compared to your attempt:**
- **Matches:** Correctly imported `solana_client::rpc_client::RpcClient`, instantiated `RpcClient::new(rpc_url.to_string())`, and matched on `rpc_client.get_version()` handling both `Ok(v)` and `Err(e)`.
- **Difference:** The reference formats `v.solana_core` into `println!` and executes `std::process::exit(1)` directly in the `Err` branch, which your updated code matches.

---

### Solution 1.2 — Modeling On-Chain State: AccountSnapshot

**Reference implementation:**
```rust
use solana_sdk::pubkey::Pubkey;

/// Represents an observed state snapshot of an on-chain Solana account at a specific slot.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountSnapshot {
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub slot: u64,
}

impl AccountSnapshot {
    /// Creates a new `AccountSnapshot`.
    pub fn new(pubkey: Pubkey, owner: Pubkey, lamports: u64, data: Vec<u8>, slot: u64) -> Self {
        Self {
            pubkey,
            owner,
            lamports,
            data,
            slot,
        }
    }

    /// Helper to convert lamports to SOL (1 SOL = 1_000_000_000 lamports).
    pub fn sol_balance(&self) -> f64 {
        self.lamports as f64 / 1_000_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_snapshot_creation_and_sol_balance() {
        let pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let snapshot = AccountSnapshot::new(pubkey, owner, 2_500_000_000, vec![1, 2, 3], 100);

        assert_eq!(snapshot.lamports, 2_500_000_000);
        assert_eq!(snapshot.sol_balance(), 2.5);
        assert_eq!(snapshot.data.len(), 3);
    }
}
```

**Why this & why not that:**
- `Pubkey` vs `String`: 32-byte cryptographic copyable array vs heap-allocated 44+ byte base58 string.
- `Vec<u8>` vs `&[u8]`: Owned buffer allows moving snapshot across pipeline channels without lifetime annotations.
- `Self { ... }`: Idiomatic type alias within `impl` blocks.
- `self.lamports as f64 / 1_000_000_000.0`: Omission of trailing semicolon turns expression into the implicit return value; cast to `f64` prevents integer division truncation.

**Compared to your attempt:**
- **Matches:** Correct struct fields, derives (`Debug, Clone, PartialEq`), constructor shorthand field syntax `Self { pubkey, owner, lamports, data, slot }`, correct float division, and passing unit test.
- **Difference:** Your updated implementation matches the reference perfectly.
