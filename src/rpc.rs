use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;



use crate::error::IndexerError;
use crate::models::account::AccountSnapshot;



pub struct SolanaRpcClient{
    pub client: RpcClient,
    pub commitment: CommitmentConfig,
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
}

impl SolanaRpcClient {


    // pub fn new(rpc_url: &str) -> Self {

    //     RpcClient::new_with_commitment(rpc_url.to_string(),CommitmentConfig::confirmed())
    //     commitment: CommitmentConfig::confirmed(),
    //     max_retries: 3,
    //     initial_backoff_ms: 500,
    // }

    pub fn new(rpc_url: &str) -> Self {
        Self::new_with_config(
            rpc_url,
            CommitmentConfig::confirmed(),
            3,
            500,
        )
    }

    pub fn new_with_config(rpc_url: &str, commitment: CommitmentConfig, max_retries: u32, initial_backoff_ms: u64, ) -> Self {
        Self {
            client: RpcClient::new_with_commitment(rpc_url.to_string(), commitment),
            commitment,
            max_retries,
            initial_backoff_ms,}
    }

    pub fn is_rate_limited(err: &ClientError) -> bool {
       let msg = err.to_string().to_lowercase();
       msg.contains("429") || msg.contains("too many requests") || msg.contains("rate limit")
    }

    pub fn execute_with_retry<T, F>(&self, mut op: F) -> Result<T, IndexerError> where F: FnMut() -> Result<T, ClientError>, {
        for attempt in 0..=self.max_retries {
            match op() {
                Ok(val) => return Ok(val),
                Err(e) => {
                    if Self::is_rate_limited(&e) {
                        if attempt < self.max_retries {
                            let backoff = self.initial_backoff_ms * 2u64.pow(attempt);

                            std::thread::sleep(Duration::from_millis(backoff));
                                continue;
                        } else {
                            return Err(IndexerError::RateLimited);
                        }
                    } else {
                        return Err(IndexerError::RpcError(e.to_string()));
                    }
                }
            }
        }
        Err(IndexerError::RateLimited)
    }

    pub fn get_account(&self, pubkey: &Pubkey) -> Result<AccountSnapshot, IndexerError> {
        
    }
}



