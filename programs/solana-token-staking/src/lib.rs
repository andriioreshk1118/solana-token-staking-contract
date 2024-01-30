use anchor_lang::prelude::*;

declare_id!("BESn7mK1YzUMGjdW8V5msezxN51ZuoxUnjMkGTUSPTHW");

#[program]
pub mod solana_token_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
