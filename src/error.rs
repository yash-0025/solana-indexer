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