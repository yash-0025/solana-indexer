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

### Exercise 1.6 (Day 1) — Resilient Solana RPC Client & Rate-Limit Backoff
**Status:** open
**Goal:** Implement `SolanaRpcClient` wrapping `solana-client::RpcClient` with `confirmed` commitment, rate-limit detection, exponential backoff retry loop, and account/balance query helpers converting into `AccountSnapshot` and `IndexerError`.

**Skeleton:**
```rust
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;

use crate::error::IndexerError;
use crate::models::account::AccountSnapshot;

/// Resilient RPC client wrapper with built-in retry and backoff logic.
pub struct SolanaRpcClient {
    pub client: RpcClient,
    pub commitment: CommitmentConfig,
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
}

impl SolanaRpcClient {
    /// Creates a new resilient RPC client with default confirmed commitment and retry settings.
    pub fn new(rpc_url: &str) -> Self {
        // TODO(1): Instantiate RpcClient with confirmed commitment, and return Self with:
        //          - client: RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed())
        //          - commitment: CommitmentConfig::confirmed()
        //          - max_retries: 3
        //          - initial_backoff_ms: 500
        todo!()
    }

    /// Creates a client with custom commitment and retry parameters.
    pub fn new_with_config(
        rpc_url: &str,
        commitment: CommitmentConfig,
        max_retries: u32,
        initial_backoff_ms: u64,
    ) -> Self {
        // TODO(2): Instantiate and return Self with provided parameters
        todo!()
    }

    /// Determines whether a given `ClientError` represents an HTTP 429 or rate limit response.
    pub fn is_rate_limited(err: &ClientError) -> bool {
        // TODO(3): Check if err.to_string().to_lowercase() contains:
        //          "429", "too many requests", or "rate limit"
        todo!()
    }

    /// Executes an RPC operation with exponential backoff retry upon encountering rate limits.
    pub fn execute_with_retry<T, F>(&self, mut op: F) -> Result<T, IndexerError>
    where
        F: FnMut() -> Result<T, ClientError>,
    {
        // TODO(4): Loop attempt from 0 to self.max_retries (inclusive):
        //          - Call op()
        //          - If Ok(val) => return Ok(val)
        //          - If Err(e):
        //              - If Self::is_rate_limited(&e) and attempt < self.max_retries:
        //                  let backoff = self.initial_backoff_ms * 2u64.pow(attempt);
        //                  std::thread::sleep(Duration::from_millis(backoff));
        //                  continue;
        //              - If Self::is_rate_limited(&e) and attempt == self.max_retries:
        //                  return Err(IndexerError::RateLimited);
        //              - Otherwise:
        //                  return Err(IndexerError::RpcError(e.to_string()));
        todo!()
    }

    /// Fetches an account snapshot at the configured commitment, converting it into `AccountSnapshot`.
    pub fn get_account(&self, pubkey: &Pubkey) -> Result<AccountSnapshot, IndexerError> {
        // TODO(5): Call self.client.get_account_with_commitment(pubkey, self.commitment)
        //          inside self.execute_with_retry:
        //          - If response.value is None => return Err(IndexerError::AccountNotFound(pubkey.to_string()))
        //          - If response.value is Some(acc) => return Ok(AccountSnapshot::new(
        //              *pubkey, acc.owner, acc.lamports, acc.data, response.context.slot
        //            ))
        todo!()
    }

    /// Fetches the lamport balance of an account with automatic retry.
    pub fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, IndexerError> {
        // TODO(6): Fetch balance via self.client.get_balance(pubkey) inside self.execute_with_retry
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_initialization() {
        let client = SolanaRpcClient::new("https://api.devnet.solana.com");
        assert_eq!(client.commitment, CommitmentConfig::confirmed());
        assert_eq!(client.max_retries, 3);
        assert_eq!(client.initial_backoff_ms, 500);
    }

    #[test]
    fn test_rate_limit_detection() {
        let dummy_io_err = std::io::Error::new(std::io::ErrorKind::Other, "HTTP 429 Too Many Requests");
        let client_err = ClientError::from(dummy_io_err);
        assert!(SolanaRpcClient::is_rate_limited(&client_err));

        let normal_io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "entity not found");
        let normal_client_err = ClientError::from(normal_io_err);
        assert!(!SolanaRpcClient::is_rate_limited(&normal_client_err));
    }

    #[test]
    fn test_execute_with_retry_succeeds_first_try() {
        let client = SolanaRpcClient::new_with_config(
            "https://api.devnet.solana.com",
            CommitmentConfig::confirmed(),
            3,
            10,
        );
        let res = client.execute_with_retry(|| Ok(42));
        assert_eq!(res.unwrap(), 42);
    }

    #[test]
    fn test_execute_with_retry_exhaustion() {
        let client = SolanaRpcClient::new_with_config(
            "https://api.devnet.solana.com",
            CommitmentConfig::confirmed(),
            2,
            5,
        );
        let mut attempts = 0;
        let res: Result<(), IndexerError> = client.execute_with_retry(|| {
            attempts += 1;
            let io_err = std::io::Error::new(std::io::ErrorKind::Other, "429 Too Many Requests");
            Err(ClientError::from(io_err))
        });
        assert_eq!(res.unwrap_err(), IndexerError::RateLimited);
        assert_eq!(attempts, 3);
    }
}
```
**Constraints:** Do not change struct or method signatures. Keep retry backoff formula exponential.
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*

---

## Solved

### Exercise 1.5 (Day 1) — Resilient Error Handling & IndexerError with thiserror
**Status:** solved
**Goal:** Implement the custom `IndexerError` enum covering all 7 required domain variants using `thiserror` attributes and an automatic `From<std::io::Error>` conversion.

**Skeleton:**
```rust
use thiserror::Error;

/// Custom error domain for the Solana Indexer pipeline.
#[derive(Error, Debug, PartialEq)]
pub enum IndexerError {
    // TODO(1): Define RpcError with message: #[error("RPC client error: {0}")]
    //          carrying an inner `String`
    // TODO(2): Define DecodeError with message: #[error("Failed to decode account data: {0}")]
    //          carrying an inner `String`
    // TODO(3): Define AccountNotFound with message: #[error("Account not found: {0}")]
    //          carrying an inner `String`
    // TODO(4): Define InvalidPubkey with message: #[error("Invalid public key string: {0}")]
    //          carrying an inner `String`
    // TODO(5): Define RateLimited with message: #[error("RPC rate limit reached. Retry after backoff")]
    //          as a unit variant (no fields)
    // TODO(6): Define ConfigError with message: #[error("Configuration error: {0}")]
    //          carrying an inner `String`
    // TODO(7): Define StorageError with message: #[error("Storage I/O error: {0}")]
    //          carrying an inner `String`
}

// TODO(8): Implement From<std::io::Error> for IndexerError
impl From<std::io::Error> for IndexerError {
    fn from(err: std::io::Error) -> Self {
        // Map into IndexerError::StorageError carrying the error display string
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexer_error_display_messages() {
        assert_eq!(
            IndexerError::RpcError("connection timeout".to_string()).to_string(),
            "RPC client error: connection timeout"
        );
        assert_eq!(
            IndexerError::DecodeError("invalid borsh length".to_string()).to_string(),
            "Failed to decode account data: invalid borsh length"
        );
        assert_eq!(
            IndexerError::AccountNotFound("4Nd1m...".to_string()).to_string(),
            "Account not found: 4Nd1m..."
        );
        assert_eq!(
            IndexerError::InvalidPubkey("bad_key".to_string()).to_string(),
            "Invalid public key string: bad_key"
        );
        assert_eq!(
            IndexerError::RateLimited.to_string(),
            "RPC rate limit reached. Retry after backoff"
        );
        assert_eq!(
            IndexerError::ConfigError("missing rpc_url".to_string()).to_string(),
            "Configuration error: missing rpc_url"
        );
        assert_eq!(
            IndexerError::StorageError("disk full".to_string()).to_string(),
            "Storage I/O error: disk full"
        );
    }

    #[test]
    fn test_from_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let indexer_err: IndexerError = io_err.into();
        assert_eq!(
            indexer_err,
            IndexerError::StorageError("access denied".to_string())
        );
    }

    #[test]
    fn test_question_mark_propagation() {
        fn mock_fallible_operation(fail: bool) -> Result<String, IndexerError> {
            if fail {
                Err(IndexerError::RateLimited)
            } else {
                Ok("success".to_string())
            }
        }

        fn caller(fail: bool) -> Result<String, IndexerError> {
            let res = mock_fallible_operation(fail)?;
            Ok(res)
        }

        assert_eq!(caller(false).unwrap(), "success");
        assert_eq!(caller(true).unwrap_err(), IndexerError::RateLimited);
    }
}
```
**Constraints:** Do not change enum variant names or test assertions. Use `thiserror::Error` derive.
**Hints used:** 0/3
**My attempt:**
```rust
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IndexerError {
    #[error("RPC client error: {0}")] 
    RpcError(String),
    #[error("Failed to decode account data: {0}")]
    DecodeError(String),
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    #[error("Invalid public key string: {0}")]
    InvalidPubkey(String),
    #[error("RPC rate limit reached. Retry after backoff")]
    RateLimited,
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Storage I/O error: {0}")]
    StorageError(String),
}

impl From<std::io::Error> for IndexerError {
    fn from(err: std::io::Error) -> Self {
        IndexerError::StorageError(err.to_string())
    }
}
```

---

### Exercise 1.4 (Day 1) — CLI Interface & Subcommands with Clap Derive
**Status:** solved
**Goal:** Implement the top-level CLI parser struct and Commands enum using Clap derive with subcommands (`account`, `tx`, `watch`, `backfill`, `stats`) and an execution dispatcher.

**Skeleton:**
```rust
use clap::{Parser, Subcommand};

/// Solana Real-Time & Historical Blockchain Indexer CLI.
#[derive(Parser, Debug)]
#[command(name = "rust-indexer", about = "Solana Real-Time & Historical Blockchain Indexer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported administrative and operational commands for the indexer.
#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    // TODO(1): Define Account subcommand with `pubkey: String` positional argument
    // TODO(2): Define Tx subcommand with `signature: String` positional argument
    // TODO(3): Define Watch subcommand with `program_id: String` positional argument
    // TODO(4): Define Backfill subcommand with `program_id: String` positional argument and optional `#[arg(long)] since: Option<u64>` flag
    // TODO(5): Define Stats subcommand taking no arguments
}

/// Dispatches the parsed command to placeholder handlers.
pub fn execute_command(cmd: &Commands) -> String {
    // TODO(6): Match exhaustively on `cmd` and return descriptive placeholder strings:
    // - Commands::Account { pubkey } => format!("Fetching account: {}", pubkey)
    // - Commands::Tx { signature } => format!("Fetching transaction: {}", signature)
    // - Commands::Watch { program_id } => format!("Watching program: {}", program_id)
    // - Commands::Backfill { program_id, since } => if let Some(s) = since { format!("Backfilling program: {} since slot {}", program_id, s) } else { format!("Backfilling program: {} from beginning", program_id) }
    // - Commands::Stats => "Displaying indexer statistics".to_string()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_account_parsing() {
        let args = vec!["rust-indexer", "account", "11111111111111111111111111111111"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse account command");
        assert_eq!(
            cli.command,
            Commands::Account {
                pubkey: "11111111111111111111111111111111".to_string()
            }
        );
        let output = execute_command(&cli.command);
        assert_eq!(output, "Fetching account: 11111111111111111111111111111111");
    }

    #[test]
    fn test_cli_tx_parsing() {
        let args = vec!["rust-indexer", "tx", "5Verifysig12345"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse tx command");
        assert_eq!(
            cli.command,
            Commands::Tx {
                signature: "5Verifysig12345".to_string()
            }
        );
        let output = execute_command(&cli.command);
        assert_eq!(output, "Fetching transaction: 5Verifysig12345");
    }

    #[test]
    fn test_cli_watch_parsing() {
        let args = vec!["rust-indexer", "watch", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse watch command");
        assert_eq!(
            cli.command,
            Commands::Watch {
                program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string()
            }
        );
        let output = execute_command(&cli.command);
        assert_eq!(output, "Watching program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    }

    #[test]
    fn test_cli_backfill_parsing() {
        let args_with_slot = vec![
            "rust-indexer",
            "backfill",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "--since",
            "1500",
        ];
        let cli = Cli::try_parse_from(args_with_slot).expect("Failed to parse backfill with slot");
        assert_eq!(
            cli.command,
            Commands::Backfill {
                program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                since: Some(1500),
            }
        );
        let output = execute_command(&cli.command);
        assert_eq!(
            output,
            "Backfilling program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA since slot 1500"
        );

        let args_no_slot = vec![
            "rust-indexer",
            "backfill",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        ];
        let cli_no_slot = Cli::try_parse_from(args_no_slot).expect("Failed to parse backfill without slot");
        assert_eq!(
            cli_no_slot.command,
            Commands::Backfill {
                program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
                since: None,
            }
        );
        let output_no_slot = execute_command(&cli_no_slot.command);
        assert_eq!(
            output_no_slot,
            "Backfilling program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA from beginning"
        );
    }

    #[test]
    fn test_cli_stats_parsing() {
        let args = vec!["rust-indexer", "stats"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse stats command");
        assert_eq!(cli.command, Commands::Stats);
        let output = execute_command(&cli.command);
        assert_eq!(output, "Displaying indexer statistics");
    }
}
```
**Constraints:** Do not change enum variant names or test assertions. Use `clap` derive macros.
**Hints used:** 0/3
**My attempt:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rust-indexer", about = "Solana Real-Time & Historical Blockchain Indexer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    Account {
        pubkey: String,
    },
    Tx {
        signature: String,
    },
    Watch {
        program_id: String,
    },
    Backfill {
        program_id: String,
        #[arg(long)]
        since: Option<u64>,
    },
    Stats,
}

pub fn execute_command(cmd: &Commands) -> String {
    match cmd {
        Commands::Account { pubkey } => format!("Fetching account: {}", pubkey),
        Commands::Tx { signature } => format!("Fetching transaction {}", signature),
        Commands::Watch { program_id } => format!("Watching program: {}", program_id),
        Commands::Backfill { program_id, since } => {
            if let Some(s) = since {
                format!("Backfilling program: {} since slot {}", program_id, s)
            } else {
                format!("Backfilling program: {} from beginning", program_id)
            }
        }
        Commands::Stats => format!("Displaying indexer statistics"),
    }
}
```

---

### Exercise 1.3b (Day 1) — 3-Tier Precedence Configuration Loading & TOML Parsing
**Status:** solved
**Goal:** Implement `ConfigFile` deserialization and layered configuration loading resolving defaults, TOML overrides, and environment variable overrides with unit tests.

**Skeleton:**
```rust
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Intermediate optional schema for deserializing `config.toml`.
/// Fields are `Option<T>` so partial configuration files merge cleanly onto defaults.
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct ConfigFile {
    pub rpc_url: Option<String>,
    pub target_program: Option<String>,
    pub commitment: Option<String>,
    pub poll_interval_ms: Option<u64>,
    pub data_dir: Option<String>,
}

impl ConfigFile {
    /// Deserializes a raw TOML string slice into a `ConfigFile`.
    pub fn from_toml_str(content: &str) -> Result<Self, toml::de::Error> {
        // TODO(1): Use `toml::from_str` to deserialize `content` into `ConfigFile`
        todo!()
    }
}

impl IndexerConfig {
    /// Loads configuration through 3-tier precedence:
    /// Tier 1: `IndexerConfig::default()`
    /// Tier 2: `toml_str` overrides (if provided)
    /// Tier 3: Environment variables (`INDEXER_RPC_URL`, `INDEXER_COMMITMENT`, `INDEXER_POLL_INTERVAL_MS`, `INDEXER_DATA_DIR`)
    pub fn load_from_str_and_env(toml_str: Option<&str>) -> Result<Self, String> {
        // TODO(2): Initialize `config` with `Self::default()`

        // TODO(3): If `toml_str` is Some, parse via `ConfigFile::from_toml_str`.
        //          For each field present in `ConfigFile`:
        //          - `rpc_url`: update `config.rpc_url`
        //          - `target_program`: parse string into Pubkey via `Pubkey::from_str` and update `config.target_program`
        //          - `commitment`: update `config.commitment`
        //          - `poll_interval_ms`: update `config.poll_interval_ms`
        //          - `data_dir`: update `config.data_dir`

        // TODO(4): Check environment variables and override corresponding fields:
        //          - If `std::env::var("INDEXER_RPC_URL")` is Ok, update `config.rpc_url`
        //          - If `std::env::var("INDEXER_COMMITMENT")` is Ok, update `config.commitment`
        //          - If `std::env::var("INDEXER_POLL_INTERVAL_MS")` is Ok, parse `.parse::<u64>()` and update `config.poll_interval_ms`
        //          - If `std::env::var("INDEXER_DATA_DIR")` is Ok, update `config.data_dir`

        // TODO(5): Return `Ok(config)`
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_file_from_toml_str() {
        let toml_data = r#"
            rpc_url = "https://api.mainnet-beta.solana.com"
            commitment = "finalized"
            poll_interval_ms = 500
        "#;
        let file = ConfigFile::from_toml_str(toml_data).expect("Failed to parse TOML");
        assert_eq!(file.rpc_url, Some("https://api.mainnet-beta.solana.com".to_string()));
        assert_eq!(file.commitment, Some("finalized".to_string()));
        assert_eq!(file.poll_interval_ms, Some(500));
        assert_eq!(file.data_dir, None);
    }

    #[test]
    fn test_layered_config_toml_overrides_defaults() {
        let toml_data = r#"
            rpc_url = "https://custom-rpc.com"
            poll_interval_ms = 250
        "#;
        let config = IndexerConfig::load_from_str_and_env(Some(toml_data))
            .expect("Failed to load layered config");
        assert_eq!(config.rpc_url, "https://custom-rpc.com");
        assert_eq!(config.poll_interval_ms, 250);
        assert_eq!(config.commitment, "confirmed");
        assert_eq!(config.data_dir, "./data");
    }

    #[test]
    fn test_layered_config_env_overrides_file_and_defaults() {
        unsafe {
            std::env::set_var("INDEXER_RPC_URL", "https://env-override-rpc.com");
            std::env::set_var("INDEXER_POLL_INTERVAL_MS", "100");
        }
        let toml_data = r#"
            rpc_url = "https://custom-rpc.com"
            poll_interval_ms = 250
        "#;
        let config = IndexerConfig::load_from_str_and_env(Some(toml_data))
            .expect("Failed to load layered config");

        assert_eq!(config.rpc_url, "https://env-override-rpc.com");
        assert_eq!(config.poll_interval_ms, 100);

        unsafe {
            std::env::remove_var("INDEXER_RPC_URL");
            std::env::remove_var("INDEXER_POLL_INTERVAL_MS");
        }
    }
}
```

**Constraints:** Retain method signatures; ensure `Option<T>` fields in `ConfigFile`; do not alter test assertions.
**Hints used:** 0/3
**My attempt:**
```rust
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct ConfigFile {
    pub rpc_url: Option<String>,
    pub target_program: Option<String>,
    pub commitment: Option<String>,
    pub poll_interval_ms: Option<u64>,
    pub data_dir: Option<String>,
}

impl ConfigFile {
    pub fn from_toml_str(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

impl IndexerConfig {
    pub fn load_from_str_and_env(toml_str: Option<&str>) -> Result<Self, String> {
        let mut config = Self::default();

        if let Some(content) = toml_str {
            let file_config = ConfigFile::from_toml_str(content).map_err(|e| e.to_string())?;

            if let Some(url) = file_config.rpc_url {
                config.rpc_url = url;
            }

            if let Some(c) = file_config.commitment {
                config.commitment = c;
            }

            if let Some(interval) = file_config.poll_interval_ms {
                config.poll_interval_ms = interval;
            }

            if let Some(dir) = file_config.data_dir {
                config.data_dir = dir;
            }

            if let Some(program_str) = file_config.target_program {
                config.target_program = Pubkey::from_str(&program_str).map_err(|e| e.to_string())?;
            }
        }

        if let Ok(env_url) = std::env::var("INDEXER_RPC_URL") {
            config.rpc_url = env_url;
        }

        if let Ok(env_commit) = std::env::var("INDEXER_COMMITMENT") {
            config.commitment = env_commit;
        }

        if let Ok(env_poll) = std::env::var("INDEXER_POLL_INTERVAL_MS") {
            if let Ok(val) = env_poll.parse::<u64>() {
                config.poll_interval_ms = val;
            }
        }

        if let Ok(env_dir) = std::env::var("INDEXER_DATA_DIR") {
            config.data_dir = env_dir;
        }

        if let Ok(env_prog) = std::env::var("INDEXER_TARGET_PROGRAM") {
            if let Ok(prog) = Pubkey::from_str(&env_prog) {
                config.target_program = prog;
            }
        }
        Ok(config)
    }
}
```

---

### Exercise 1.3 (Day 1) — Configuration System: IndexerConfig & Hierarchy Defaults
**Status:** solved
**Goal:** Define `IndexerConfig` struct representing indexer configuration with owned `String` fields, `Default` trait implementation for Devnet defaults, and constructor with unit tests.

**Skeleton:**
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
        // TODO(1): Construct Self with:
        // - rpc_url: "https://api.devnet.solana.com".to_string()
        // - target_program: Pubkey::from_str("11111111111111111111111111111111").unwrap()
        // - commitment: "confirmed".to_string()
        // - poll_interval_ms: 1000
        // - data_dir: "./data".to_string()
        todo!()
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
        // TODO(2): Return `Self` populated with the given arguments
        todo!()
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

**Constraints:** Maintain field types; ensure `Default` trait returns valid Devnet fallbacks; do not change test assertions.
**Hints used:** 0/3
**My attempt:**
```rust
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexerConfig {
    pub rpc_url: String,
    pub target_program: Pubkey,
    pub commitment: String,
    pub poll_interval_ms: u64,
    pub data_dir: String,
}

impl Default for IndexerConfig {
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
    pub fn new(rpc_url: String, target_program: Pubkey, commitment: String, poll_interval_ms: u64, data_dir: String) -> Self {
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
        assert_eq!(default_config.target_program, Pubkey::from_str("11111111111111111111111111111111").unwrap());
        assert_eq!(default_config.commitment, "confirmed");
        assert_eq!(default_config.poll_interval_ms, 1000);
        assert_eq!(default_config.data_dir, "./data");

        let custom_program = Pubkey::new_unique();
        let custom_config = IndexerConfig::new(
            "http://127.0.0.8899".to_string(), custom_program, "finalized".to_string(), 500, "/tmp/indexer-data".to_string(),
        );

        assert_eq!(custom_config.rpc_url, "http://127.0.0.8899");
        assert_eq!(custom_config.target_program, custom_program);
        assert_eq!(custom_config.commitment, "finalized");
        assert_eq!(custom_config.poll_interval_ms, 500);
        assert_eq!(custom_config.data_dir, "/tmp/indexer-data");
    }
}
```

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