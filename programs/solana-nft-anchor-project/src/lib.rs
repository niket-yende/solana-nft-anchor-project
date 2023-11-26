use anchor_lang::prelude::*;

declare_id!("2STkCaRwFSPpCjmoV212Rn4xR5QBbMeWRyCvPVBKWq9Y");

#[program]
pub mod solana_nft_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
