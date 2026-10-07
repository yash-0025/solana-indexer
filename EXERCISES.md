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

*(None currently — all Module 1.2 exercises solved!)*

---

## Solved

### Exercise 1.2d (Day 1) — Slot Metadata & Tuple Structs: SlotInfo
**Status:** solved
**Goal:** Define `Slot` tuple struct and `SlotInfo` domain struct with constructor, consecutive parent check helper, `Display` formatting, and unit tests.

**Skeleton:**
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
        // TODO(1): Construct and return `Self` wrapping slots in `Slot(...)`
        todo!()
    }

    /// Checks if the parent slot is directly consecutive (i.e. slot == parent_slot + 1),
    /// indicating no leader slots were skipped between them.
    pub fn is_parent_consecutive(&self) -> bool {
        // TODO(2): Return `self.slot.0 == self.parent_slot.0 + 1` without trailing semicolon
        todo!()
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

**Constraints:** Maintain tuple struct wrapping `Slot(pub u64)`; ensure `Option<u64>` for `block_height`.
**Hints used:** 0/3
**My attempt:**
```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct SlotInfo {
    pub slot: Slot,
    pub parent_slot: Slot,
    pub block_height: Option<u64>,
}

impl SlotInfo {
    pub fn new(slot: u64, parent_slot: u64, block_height: Option<u64>) -> Self {
        Self {
            slot: Slot(slot),
            parent_slot: Slot(parent_slot),
            block_height,
        }
    }

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

---

### Exercise 1.2c (Day 1) — Instruction Modeling: DecodedInstruction & Algebraic Enums
**Status:** solved
**Goal:** Define `DecodedInstruction` and `InstructionPayload` algebraic enum with constructor, helper methods, `Display` formatting, and unit tests.

**Skeleton:**
```rust
use solana_sdk::pubkey::Pubkey;
use std::fmt;

/// Represents the payload of a decoded instruction.
/// Demonstrates enums as algebraic data types carrying variant-specific data.
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
        // TODO(1): Construct and return `Self`
        todo!()
    }

    /// Returns the number of accounts involved in this instruction.
    pub fn account_count(&self) -> usize {
        // TODO(2): Return `self.accounts.len()` without a trailing semicolon
        todo!()
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

**Constraints:** Maintain enum variant data payloads; ensure exhaustive `match` handling in `Display`.
**Hints used:** 0/3
**My attempt:**
```rust
use solana_sdk::pubkey::Pubkey;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionPayload {
    Transfer { amount: u64 },
    Raw(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecodedInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub payload: InstructionPayload,
}

impl DecodedInstruction {
    pub fn new(program_id: Pubkey, accounts: Vec<Pubkey>, payload: InstructionPayload) -> Self {
        Self {
            program_id,
            accounts,
            payload,
        }
    }

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

---

### Exercise 1.2b (Day 1) — Transaction Receipts: TransactionRecord
**Status:** solved
**Goal:** Define `TransactionRecord` domain struct with constructor, `Display` formatting, and unit tests.

**Skeleton:**
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
        // TODO(1): Construct and return `Self`
        todo!()
    }

    /// Checks if the transaction execution succeeded.
    pub fn is_success(&self) -> bool {
        // TODO(2): Return `self.success` without a trailing semicolon
        todo!()
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

**Constraints:** Maintain field types; implement `fmt::Display`.
**Hints used:** 0/3
**My attempt:**
```rust
use solana_sdk::signature::Signature;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionRecord {
    pub signature: Signature,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub success: bool,
}

impl TransactionRecord {
    pub fn new(signature: Signature, slot: u64, block_time: Option<i64>, success: bool) -> Self {
        Self {
            signature,
            slot,
            block_time,
            success,
        }
    }

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

---

### Exercise 1.2 (Day 1) — Modeling On-Chain State: AccountSnapshot
**Status:** solved
**Goal:** Define the core `AccountSnapshot` domain struct with constructor, helper methods, and unit tests.

**Skeleton:**
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
        // TODO(1): Construct and return `Self` with the given parameters
        todo!()
    }

    /// Helper to convert lamports to SOL (1 SOL = 1_000_000_000 lamports).
    pub fn sol_balance(&self) -> f64 {
        // TODO(2): Convert `self.lamports` to f64 divided by 1_000_000_000.0
        todo!()
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

**Constraints:** Do not change field types; ensure constructor returns `Self`.
**Hints used:** 0/3
**My attempt:**
```rust
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, PartialEq)]
pub struct AccountSnapshot {
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub slot: u64,
}

impl AccountSnapshot {
    pub fn new(pubkey: Pubkey, owner: Pubkey, lamports: u64, data: Vec<u8>, slot: u64) -> Self {
        Self { pubkey, owner, lamports, data, slot }
    }

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

---

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