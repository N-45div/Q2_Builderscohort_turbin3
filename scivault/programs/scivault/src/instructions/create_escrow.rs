use anchor_lang::prelude::*;
use crate::{state::*, errors::ErrorCode};

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1 // Discriminator + Pubkey + Pubkey + u64 + bool
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        constraint = research.author == researcher.key() @ ErrorCode::NotVerified
    )]
    pub research: Account<'info, Research>,
    #[account(
        constraint = researcher.verified @ ErrorCode::NotVerified,
        constraint = researcher.authority == seller.key() @ ErrorCode::NotVerified
    )]
    pub researcher: Account<'info, Researcher>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateEscrow>, price: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.seller = ctx.accounts.seller.key();
    escrow.research = ctx.accounts.research.key();
    escrow.price = price;
    escrow.active = true;
    Ok(())
}