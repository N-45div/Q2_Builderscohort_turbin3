use anchor_lang::prelude::*;

#[account]
pub struct Researcher {
    pub authority: Pubkey, // Wallet address of the researcher
    pub name: String,      // Researcher name (max 100 chars)
    pub verified: bool,    // Verification status
}

#[account]
pub struct Research {
    pub author: Pubkey,    // Researcher who uploaded
    pub title: String,     // Research title (max 100 chars)
    pub ipfs_hash: String, // IPFS hash of metadata (max 256 chars)
    pub minted: bool,      // Whether minted as NFT
}

#[account]
pub struct Escrow {
    pub seller: Pubkey,    // Seller's wallet
    pub research: Pubkey,  // Research being sold
    pub price: u64,        // Price in lamports
    pub active: bool,      // Escrow status
}