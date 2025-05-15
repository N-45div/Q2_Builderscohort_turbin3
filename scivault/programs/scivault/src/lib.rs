use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;


declare_id!("Gv81pBL9CrgUtTNwWFtJB4ww5iHDmZXuCsYFpQXDcX5J");

#[program]
pub mod scivault {
    use super::*;

    pub fn initialize_researcher(ctx: Context<InitializeResearcher>, name: String) -> Result<()> {
        instructions::initialize_researcher::handler(ctx, name)
    }

    pub fn upload_research(ctx: Context<UploadResearch>, title: String, ipfs_hash: String) -> Result<()> {
        instructions::upload_research::handler(ctx, title, ipfs_hash)
    }

    pub fn mint_research_nft(ctx: Context<MintResearchNFT>, uri: String) -> Result<()> {
        instructions::mint_research_nft::handler(ctx, uri)
    }

    pub fn create_escrow(ctx: Context<CreateEscrow>, price: u64) -> Result<()> {
        instructions::create_escrow::handler(ctx, price)
    }

    pub fn buy_research(ctx: Context<BuyResearch>) -> Result<()> {
        instructions::buy_research::handler(ctx)
    }
}
