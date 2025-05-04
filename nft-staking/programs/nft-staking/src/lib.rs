use anchor_lang::prelude::*;

declare_id!("4oA96QJGJKFRak87PVeNUtFpmUR4c2n41tqBU4sqvfwK");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
