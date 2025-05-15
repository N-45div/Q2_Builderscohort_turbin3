use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{MasterEditionAccount, Metadata, MetadataAccount}, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"marketplace".as_ref(), marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [marketplace.key(),as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ]
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(
        seeds = [
           b"metadata",
           metadata_program.key().as_ref(),
           maker_mint.key().as_ref(),
           b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: InterfaceAccount<'info, MasterEditionAccount>,
    pub metadata_program: InterfaceAccount<'info, MetadataProgram>,
    pub token_program: InterfaceAccount<'info, TokenProgram>,
    pub system_program: InterfaceAccount<'info, SystemProgram>,
    pub associated_token_program: InterfaceAccount<'info, AssociatedToken>,
}

impl<'info> List<'info> {
    
}