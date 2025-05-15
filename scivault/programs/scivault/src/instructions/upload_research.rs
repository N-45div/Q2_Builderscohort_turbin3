use anchor_lang::prelude::*;
use crate::{state::*, errors::ErrorCode};

#[derive(Accounts)]
pub struct UploadResearch<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 100 + 256 + 1 // Discriminator + Pubkey + String(100) + String(256) + bool
    )]
    pub research: Account<'info, Research>,
    #[account(
        constraint = researcher.verified @ ErrorCode::NotVerified,
        constraint = researcher.authority == signer.key() @ ErrorCode::NotVerified
    )]
    pub researcher: Account<'info, Researcher>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UploadResearch>, title: String, ipfs_hash: String) -> Result<()> {
    let research = &mut ctx.accounts.research;
    research.author = ctx.accounts.researcher.key();
    research.title = title;
    research.ipfs_hash = ipfs_hash; // Mocked IPFS hash for MVP
    research.minted = false;
    Ok(())
}