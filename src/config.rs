use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;


#[derive(Debug, Clone, PartialEq)]
pub struct IndexerConfig {
    pub rpc_url : String,
    pub target_program: Pubkey,
    pub commitment: String,
    pub poll_interval_ms : u64,
    pub data_dir : String,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            rpc_url : "https://api.devnet.solana.com".to_string(),
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
        "http://127.0.0.1:8899".to_string(), custom_program, "finalized".to_string(), 500, "/tmp/indexer-data".to_string(),
      );

      assert_eq!(custom_config.rpc_url, "http://127.0.0.1:8899");
      assert_eq!(custom_config.target_program, custom_program);
      assert_eq!(custom_config.commitment, "finalized");
      assert_eq!(custom_config.poll_interval_ms, 500);
      assert_eq!(custom_config.data_dir, "/tmp/indexer-data");
    }
}
