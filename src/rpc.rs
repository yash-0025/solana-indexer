use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use std::time::Duration;




use crate::error::IndexerError;
use crate::models::account::AccountSnapshot;
use crate::models::transaction::TransactionRecord;




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
        let response = self.execute_with_retry(|| {
            self.client.get_account_with_commitment(pubkey, self.commitment)
        })?;

        match response.value {
            None => Err(IndexerError::AccountNotFound(pubkey.to_string())),
            Some(acc) => Ok(AccountSnapshot::new(
                *pubkey,
                acc.owner,
                acc.lamports,
                acc.data,
                response.context.slot,
            )),
        }
    }
    
    
    pub fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, IndexerError> {
            self.execute_with_retry(|| self.client.get_balance(pubkey))
        }

    
    pub fn get_signatures_for_address(&self, pubkey: &Pubkey ) -> Result<Vec<TransactionRecord> , IndexerError> {
        let raw_statuses = self.execute_with_retry(|| self.client.get_signatures_for_address(pubkey))?;

        raw_statuses.into_iter().map(|status| {
            let sig = Signature::from_str(&status.signature).map_err(|e| IndexerError::InvalidPublickey(e.to_string()))?;


            let success = status.err.is_none();

            Ok(TransactionRecord::new(
                sig,
                status.slot,
                status.block_time,
                success,
            ))
        })

        .collect()

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












