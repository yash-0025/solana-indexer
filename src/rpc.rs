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


    pub fn new(rpc_url: &str) -> Self {

        RpcClient::new_with_commitment(rpc_url.to_string(), 
    CommitmentConfig::confirmed())
    
    }
}



