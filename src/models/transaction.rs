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
            format!("{}...{}", &sig_str[..8], &sig_str[sig_str.len() - 8..] )
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