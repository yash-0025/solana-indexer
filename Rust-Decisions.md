# 🦀 Rust-Decisions.md — "Why This & Why Not That?"

> A running ledger of every Rust design and syntax decision made across the Solana indexer codebase. Each entry details why a specific Rust construct, type, or pattern was chosen, and why common alternatives were rejected.

---

### Module 1.1 — Initializing RpcClient & Cluster Connectivity Handshake

#### 1. `rpc_url.to_string()` — Why not pass `rpc_url` directly?
- **Why `to_string()` (`String`)**: `RpcClient::new()` expects an owned `String`. That means it wants to take full ownership of the URL bytes so it can hold onto them internally for the entire life of the client.
- **Why not `&str`**: `"https://api.devnet.solana.com"` is a string slice (`&str`)—just a borrowed view. If the client accepted `&str`, it would need lifetime annotations (like `RpcClient<'a>`), which would tie the client to where the string was created and make it messy to store in structs later.

#### 2. `match client.get_version()` — Why not `.unwrap()` or `if let`?
- **Why `match`**: In Rust, network calls return `Result<T, E>`. `match` forces you to handle both `Ok(...)` and `Err(...)`.
- **Why not `.unwrap()`**: Calling `.unwrap()` panics (crashes) the program if the RPC is unreachable. In an indexer, crashing the entire service on a transient network hiccup is bad practice.
- **Why not `if let Ok(...)`**: `if let` only handles the happy path and silently ignores the error. For an indexer's startup handshake, we need to know and log the exact error if the connection fails.

#### 3. `std::process::exit(1)` on error
- **Why this**: Returning an exit code of `1` communicates to the OS or terminal that the binary terminated with a failure, allowing container orchestrators (like Docker or systemd) to detect that the indexer failed to boot.

---

### Module 1.2 — Modeling On-Chain State: AccountSnapshot

#### 1. `Pubkey` vs `String`
- **Why `Pubkey`**: Solana addresses are 32-byte binary public keys. `solana_sdk::pubkey::Pubkey` is a zero-cost wrapper around `[u8; 32]`. It implements `Copy`, requires zero heap allocation, and guarantees validity at compile time.
- **Why not `String`**: Base58 strings (e.g. `"4Nd1m..."`) take 44+ bytes on the heap, require heap allocations to clone, and can accidentally contain invalid characters.

#### 2. `Vec<u8>` vs `&[u8]` for `data`
- **Why `Vec<u8>`**: The snapshot owns its raw data buffer. This allows the snapshot to be sent across channels to storage or database threads without lifetime annotations (`'a`).
- **Why not `&[u8]`**: A borrowed slice would lock the snapshot to the temporary lifetime of the RPC response buffer.

#### 3. `Self` in `pub fn new(...) -> Self`
- In Rust, `Self` inside an `impl AccountSnapshot` block is an alias for the type `AccountSnapshot`. Using `Self` is idiomatic Rust because if you ever rename the struct, the constructor signature doesn't break.

#### 4. `self.lamports as f64 / 1_000_000_000.0`
- Lamports are stored as integer `u64`. Because integer division in Rust truncates decimal points (`5 / 2 = 2`), we cast to float `f64` before dividing by `1_000_000_000.0` so we get fractional SOL (e.g., `2.5` SOL).

#### 5. Semicolons: Expression vs Statement in Function Returns
- **Why omit the semicolon (`self.lamports as f64 / 1_000_000_000.0`)**: In Rust, the final expression in a block without a semicolon is automatically returned. This is idiomatic Rust.
- **Why not `return ...;`**: `return` works, but idiomatic Rust reserves the `return` keyword only for early exits (like returning early from inside an `if` check). Putting a semicolon turns an expression into a statement, yielding `()` instead of the value.

#### 6. Method Call Parentheses `()` vs Field Access
- In Rust, calling an instance method always requires parentheses `()`, even if it takes no arguments beyond `&self` (e.g. `snapshot.sol_balance()`). Omitting parentheses causes the compiler to look for a struct data field of that name.

---

### Module 1.2b — Transaction Receipts & Signatures: TransactionRecord

#### 1. `Signature` vs `String`
- **Why `Signature`**: `solana_sdk::signature::Signature` is a 64-byte cryptographic type wrapping `[u8; 64]`. It implements `Copy`, requires zero heap allocation, and enforces cryptographic signature length at compile time.
- **Why not `String`**: Base58 transaction signatures take 88+ bytes on the heap, require heap allocations, and permit invalid non-base58 strings.

#### 2. `Option<i64>` for `block_time`
- **Why `Option<i64>`**: Solana ledger block times are estimated Unix timestamps produced by validator votes; for certain slots (or under slot skips), a timestamp may not exist. Rust's `Option` makes presence or absence explicit (`Some(ts)` vs `None`).
- **Why not a sentinel integer like `0` or `-1`**: Sentinel values can accidentally be parsed as valid dates (e.g. `0` = Jan 1, 1970), leading to subtle downstream database corruption.

#### 3. `impl fmt::Display` vs `#[derive(Debug)]`
- **Why `Display`**: Formats the transaction into user-friendly CLI output (e.g., truncating the 88-char signature into `abc...xyz` with status badge) when printed with `{}`.
- **Why `Debug`**: Intended for developers and internal tracing/logging with `{:?}`, dumping full raw field data.

---


