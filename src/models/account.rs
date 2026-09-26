use solana_sdk::pubkey::Pubkey;


// Represent an observed state snapshot of an on-chain solana account at a specific slot.
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
        Self {pubkey, owner, lamports, data, slot}
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