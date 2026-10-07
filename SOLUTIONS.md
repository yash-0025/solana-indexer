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

---

### Solution 1.2b — Transaction Receipts & Signatures: TransactionRecord

**Reference implementation:**
```rust
use solana_sdk::signature::Signature;
use std::fmt;

/// Represents a confirmed transaction receipt on the Solana cluster.
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionRecord {
    pub signature: Signature,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub success: bool,
}

impl TransactionRecord {
    /// Creates a new `TransactionRecord`.
    pub fn new(signature: Signature, slot: u64, block_time: Option<i64>, success: bool) -> Self {
        Self {
            signature,
            slot,
            block_time,
            success,
        }
    }

    /// Checks if the transaction execution succeeded.
    pub fn is_success(&self) -> bool {
        self.success
    }
}

impl fmt::Display for TransactionRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sig_str = self.signature.to_string();
        let short_sig = if sig_str.len() > 16 {
            format!("{}...{}", &sig_str[..8], &sig_str[sig_str.len() - 8..])
        } else {
            sig_str
        };
        let status = if self.success { "SUCCESS" } else { "FAILED" };
        write!(f, "Tx {} @ slot {} [{}]", short_sig, self.slot, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_record_creation_and_display() {
        let sig = Signature::new_unique();
        let record = TransactionRecord::new(sig, 150, Some(1700000000), true);

        assert_eq!(record.slot, 150);
        assert_eq!(record.block_time, Some(1700000000));
        assert!(record.is_success());

        let display_output = format!("{}", record);
        assert!(display_output.contains("SUCCESS"));
        assert!(display_output.contains("slot 150"));
    }
}
```

**Why this & why not that:**
- `Signature` vs `String`: 64-byte cryptographic type wrapping `[u8; 64]`; resides on the stack, copyable, zero heap allocations, compile-time signature validity.
- `Option<i64>` vs sentinel `0` / `-1`: Explicitly denotes presence (`Some(ts)`) or absence (`None`) of approximate ledger timestamps without polluting databases with fake 1970 Unix epoch dates.
- `self.success`: Returning without a trailing semicolon yields the boolean value cleanly as an expression.
- `Display` vs `Debug`: Formats 88-char base58 signatures cleanly into `abc...xyz` with status badges for human CLI output, leaving `Debug` for developer inspection.

**Compared to your attempt:**
- **Matches:** Struct definition, constructor `Self { signature, slot, block_time, success }`, `is_success()` returning `self.success` without a semicolon, `Display` implementation with truncated signature format, and passing unit test.
- **Difference:** None! Your implementation matches the reference perfectly.

---

### Solution 1.2c — Instruction Modeling: DecodedInstruction & Algebraic Enums

**Reference implementation:**
```rust
use solana_sdk::pubkey::Pubkey;
use std::fmt;

/// Represents the payload of a decoded instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionPayload {
    /// A transfer of funds with an explicit amount in lamports.
    Transfer { amount: u64 },
    /// An arbitrary program invocation with raw payload bytes.
    Raw(Vec<u8>),
}

/// Represents an instruction executed within a transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub payload: InstructionPayload,
}

impl DecodedInstruction {
    /// Creates a new `DecodedInstruction`.
    pub fn new(program_id: Pubkey, accounts: Vec<Pubkey>, payload: InstructionPayload) -> Self {
        Self {
            program_id,
            accounts,
            payload,
        }
    }

    /// Returns the number of accounts involved in this instruction.
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }
}

impl fmt::Display for DecodedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prog_str = self.program_id.to_string();
        let short_prog = if prog_str.len() > 16 {
            format!("{}...{}", &prog_str[..8], &prog_str[prog_str.len() - 8..])
        } else {
            prog_str
        };

        match &self.payload {
            InstructionPayload::Transfer { amount } => {
                write!(f, "Instruction [{}]: Transfer {} lamports across {} accounts", short_prog, amount, self.accounts.len())
            }
            InstructionPayload::Raw(bytes) => {
                write!(f, "Instruction [{}]: Raw ({} bytes) across {} accounts", short_prog, bytes.len(), self.accounts.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoded_instruction_transfer_and_display() {
        let program_id = Pubkey::new_unique();
        let sender = Pubkey::new_unique();
        let receiver = Pubkey::new_unique();
        let accounts = vec![sender, receiver];
        let payload = InstructionPayload::Transfer { amount: 500_000 };

        let ix = DecodedInstruction::new(program_id, accounts, payload);

        assert_eq!(ix.account_count(), 2);
        let display = format!("{}", ix);
        assert!(display.contains("Transfer 500000 lamports"));
        assert!(display.contains("across 2 accounts"));
    }
}
```

**Why this & why not that:**
- `enum InstructionPayload`: Enums as Algebraic Data Types (ADTs) allow embedding distinct payloads per variant without heap boxing or runtime reflection.
- `Vec<Pubkey>` vs fixed array: Supports dynamic account lists of arbitrary size without artificial limits.
- `match &self.payload`: Enforces exhaustive pattern matching at compile time.
- `self.accounts.len()`: Returning without semicolon yields `usize` cleanly as an expression.

**Compared to your attempt:**
- **Matches:** Struct fields, enum variants, constructor `Self { program_id, accounts, payload }`, `account_count()` method returning `self.accounts.len()`, `Display` formatting with `match &self.payload`, and passing unit test.
- **Difference:** None! Your implementation matches the reference perfectly.

---

### Solution 1.2d — Slot Metadata & Tuple Structs: SlotInfo

**Reference implementation:**
```rust
use std::fmt;

/// Tuple struct representing a Solana slot number.
/// Provides type safety so slots are never confused with balances or heights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(pub u64);

/// Metadata describing an observed ledger slot on the cluster.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotInfo {
    pub slot: Slot,
    pub parent_slot: Slot,
    pub block_height: Option<u64>,
}

impl SlotInfo {
    /// Creates a new `SlotInfo`.
    pub fn new(slot: u64, parent_slot: u64, block_height: Option<u64>) -> Self {
        Self {
            slot: Slot(slot),
            parent_slot: Slot(parent_slot),
            block_height,
        }
    }

    /// Checks if the parent slot is directly consecutive (i.e. slot == parent_slot + 1),
    /// indicating no leader slots were skipped between them.
    pub fn is_parent_consecutive(&self) -> bool {
        self.slot.0 == self.parent_slot.0 + 1
    }
}

impl fmt::Display for SlotInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height_str = match self.block_height {
            Some(h) => h.to_string(),
            None => "none".to_string(),
        };
        write!(f, "Slot {} (parent: {}, height: {})", self.slot.0, self.parent_slot.0, height_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_info_creation_and_consecutive_check() {
        let consecutive_info = SlotInfo::new(105, 104, Some(90));
        assert_eq!(consecutive_info.slot, Slot(105));
        assert_eq!(consecutive_info.parent_slot, Slot(104));
        assert_eq!(consecutive_info.block_height, Some(90));
        assert!(consecutive_info.is_parent_consecutive());

        let skipped_info = SlotInfo::new(110, 108, Some(94));
        assert!(!skipped_info.is_parent_consecutive());

        let display = format!("{}", consecutive_info);
        assert!(display.contains("Slot 105"));
        assert!(display.contains("parent: 104"));
        assert!(display.contains("height: 90"));
    }
}
```

**Why this & why not that:**
- `pub struct Slot(pub u64)`: The Newtype pattern creates a distinct type at compile time with zero runtime overhead, preventing accidental confusion with lamports or heights.
- `Option<u64>` for `block_height`: Correctly accounts for leader skip slots where no block is minted without resorting to dangerous sentinel 0 values.
- `self.slot.0 == self.parent_slot.0 + 1`: Directly unpacks the inner `u64` via positional `.0` indexing for consecutive parent slot verification.

**Compared to your attempt:**
- **Matches:** Tuple struct definition with full derive attributes, `SlotInfo` struct fields, constructor `Self { slot: Slot(slot), parent_slot: Slot(parent_slot), block_height }`, consecutive check returning `self.slot.0 == self.parent_slot.0 + 1`, `Display` formatting with `Option` matching, and passing unit tests.
- **Difference:** None! Your implementation matches the reference perfectly.

---

### Solution 1.3 — Configuration System: IndexerConfig & Hierarchy Defaults

**Reference implementation:**
```rust
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Runtime configuration settings for the Solana Indexer.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexerConfig {
    pub rpc_url: String,
    pub target_program: Pubkey,
    pub commitment: String,
    pub poll_interval_ms: u64,
    pub data_dir: String,
}

impl Default for IndexerConfig {
    /// Provides sane cluster defaults pointing to Solana Devnet and the standard System Program.
    fn default() -> Self {
        Self {
            rpc_url: "https://api.devnet.solana.com".to_string(),
            target_program: Pubkey::from_str("11111111111111111111111111111111").unwrap(),
            commitment: "confirmed".to_string(),
            poll_interval_ms: 1000,
            data_dir: "./data".to_string(),
        }
    }
}

impl IndexerConfig {
    /// Creates a custom `IndexerConfig`.
    pub fn new(
        rpc_url: String,
        target_program: Pubkey,
        commitment: String,
        poll_interval_ms: u64,
        data_dir: String,
    ) -> Self {
        Self {
            rpc_url,
            target_program,
            commitment,
            poll_interval_ms,
            data_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexer_config_default_and_custom() {
        let default_config = IndexerConfig::default();
        assert_eq!(default_config.rpc_url, "https://api.devnet.solana.com");
        assert_eq!(default_config.commitment, "confirmed");
        assert_eq!(default_config.poll_interval_ms, 1000);
        assert_eq!(default_config.data_dir, "./data");
        assert_eq!(
            default_config.target_program,
            Pubkey::from_str("11111111111111111111111111111111").unwrap()
        );

        let custom_program = Pubkey::new_unique();
        let custom_config = IndexerConfig::new(
            "http://127.0.0.1:8899".to_string(),
            custom_program,
            "finalized".to_string(),
            500,
            "/tmp/indexer-data".to_string(),
        );

        assert_eq!(custom_config.rpc_url, "http://127.0.0.1:8899");
        assert_eq!(custom_config.target_program, custom_program);
        assert_eq!(custom_config.commitment, "finalized");
        assert_eq!(custom_config.poll_interval_ms, 500);
        assert_eq!(custom_config.data_dir, "/tmp/indexer-data");
    }
}
```

**Why this & why not that:**
- `pub rpc_url: String`: Owned heap `String` eliminates lifetime contagion (`'a`), enabling the config to be moved into worker threads and async tasks without borrowing restrictions.
- `impl Default for IndexerConfig`: Idiomatic Rust trait providing deterministic fallback settings (`Devnet`, `System Program`, `confirmed`) without requiring caller boilerplate.
- `Pubkey::from_str("11111111111111111111111111111111").unwrap()`: The 32-byte base58 System Program address is known and static at compile time, making `.unwrap()` safe in fallback initializers.

**Compared to your attempt:**
- **Matches:** Struct fields with owned types, complete `impl Default for IndexerConfig` with exact Devnet fallbacks, constructor shorthand `Self { rpc_url, ... }`, and passing unit test suite!
- **Difference:** Your custom test URL used `"http://127.0.0.8899"` (a slight typo for `"http://127.0.0.1:8899"`), but your assertion matched it identically and all logic is completely sound.


