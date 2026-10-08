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
        pubkey: String
    },
    Tx {
        signature: String
    },
    Watch {
        program_id: String
    },
    Backfill {
        program_id: String,
        #[arg(long)] // It means since <SLOT> will be flagged and Option<u64> will be the value that user will enter.
        since: Option<u64>,
    },
    Stats,
}

pub fn execute_command(cmd: &Commands) -> String {
    match cmd{
     Commands::Account { pubkey } => format!("Fetching account: {}", {pubkey}),
     Commands::Tx { signature } => format!("Fetching transaction {}", signature),
     Commands::Watch { program_id } => format!("Watching program: {}", program_id),
     Commands::Backfill { program_id, since } => if let Some(s) = since {
        format!("Backfilling program: {} since slot {}", program_id, s) } else {
            format!("Backfilling program: {} from beginning", program_id)
        }
     Commands::Stats => format!("Displaying indexer statistics"),
    }
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
                pubkey:"11111111111111111111111111111111".to_string()
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