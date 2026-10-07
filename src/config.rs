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


    pub fn load_from_str_and_env(toml_str: Option<&str>) -> Result<Self, String>{
        // mut is necessary as we are going to mutate field afterwards
        let mut config = Self::default();

        if let Some(content) = toml_str{
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

    #[test]
    fn test_config_file_from_toml_str() {
        let toml_data = r#"
        rpc_url= "https://api.mainnet-beta.solana.com"
        commitment = "finalized"
        poll_interval_ms = 500
        "#;

        let file = ConfigFile::from_toml_str(toml_data).expect("Failed to parse TOML");
        assert_eq!(file.rpc_url, Some("https://api.mainnet-beta.solana.com".to_string()));
        assert_eq!(file.commitment, Some("finalized".to_string()));
        assert_eq!(file.poll_interval_ms, Some(500));
        assert_eq!(file.data_dir, None);
        assert_eq!(file.target_program, None);
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
       unsafe { std::env::set_var("INDEXER_RPC_URL", "https://env-override-rpc.com");
        std::env::set_var("INDEXER_POLL_INTERVAL_MS", "100");}
        let toml_data = r#"
            rpc_url = "https://custom-rpc.com"
            poll_interval_ms = 250
        "#;
        let config = IndexerConfig::load_from_str_and_env(Some(toml_data))
            .expect("Failed to load layered config");
        assert_eq!(config.rpc_url, "https://env-override-rpc.com");
        assert_eq!(config.poll_interval_ms, 100);
       unsafe { std::env::remove_var("INDEXER_RPC_URL");
        std::env::remove_var("INDEXER_POLL_INTERVAL_MS");}
    }
}
