use anchor_lang::prelude::*;
use crate::{state::*, errors::ErrorCode};

#[derive(Accounts)]
pub struct BuyResearch<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub research: Account<'info, Research>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Seller's account to receive funds, validated by escrow.seller
    #[account(mut, constraint = escrow.seller == seller.key() @ ErrorCode::InvalidSeller)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyResearch>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(escrow.active, ErrorCode::EscrowInactive);

    // Verify buyer has enough lamports
    let buyer_lamports = ctx.accounts.buyer.lamports();
    require!(buyer_lamports >= escrow.price, ErrorCode::InsufficientFunds);

    // Transfer lamports from buyer to seller using System Program
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        escrow.price,
    )?;

    // Close escrow
    escrow.active = false;
    Ok(())
}