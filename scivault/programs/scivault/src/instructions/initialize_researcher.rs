use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeResearcher<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 100 + 1 // Discriminator + Pubkey + String(100) + bool
    )]
    pub researcher: Account<'info, Researcher>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeResearcher>, name: String) -> Result<()> {
    let researcher = &mut ctx.accounts.researcher;
    researcher.authority = ctx.accounts.signer.key();
    researcher.name = name;
    researcher.verified = true; // Simplified for MVP
    Ok(())
}