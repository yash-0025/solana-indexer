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
    pub accounts : Vec<Pubkey>,
    pub payload: InstructionPayload,
}


impl DecodedInstruction{
    pub fn new(program_id: Pubkey, accounts: Vec<Pubkey>, payload:InstructionPayload) -> Self {
        Self{
            program_id,
            accounts, 
            payload,
        }
    }

    pub fn account_count(&self) -> usize{
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
    fn test_decoded_instruction_transfer_and_display()
 {
    let program_id = Pubkey::new_unique();
         let program_id = Pubkey::new_unique();
         let sender = Pubkey::new_unique();
         let receiver = Pubkey::new_unique();
         let accounts = vec![sender, receiver];
         let payload = InstructionPayload::Transfer { amount: 500_000};


         let ix = DecodedInstruction::new(program_id, accounts, payload);

         assert_eq!(ix.account_count  (), 2);
         let display = format!("{}", ix);
         assert!(display.contains("Transfer 500000 lamports"));
         assert!(display.contains("across 2 accounts"));

 }}