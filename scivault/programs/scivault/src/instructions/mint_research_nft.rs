use anchor_lang::prelude::*;
use crate::{state::*, errors::ErrorCode};

#[derive(Accounts)]
pub struct MintResearchNFT<'info> {
    #[account(
        mut,
        constraint = research.author == researcher.key() @ ErrorCode::NotVerified
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

pub fn handler(ctx: Context<MintResearchNFT>, _uri: String) -> Result<()> {
    let research = &mut ctx.accounts.research;
    require!(!research.minted, ErrorCode::AlreadyMinted);
    // Mock Metaplex NFT minting (replace with actual Metaplex CPI post-MVP)
    research.minted = true;
    Ok(())
}