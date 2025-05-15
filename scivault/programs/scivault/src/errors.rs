use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Research already minted as NFT")]
    AlreadyMinted,
    #[msg("Escrow is not active")]
    EscrowInactive,
    #[msg("Insufficient funds for purchase")]
    InsufficientFunds,
    #[msg("Researcher not verified")]
    NotVerified,
    #[msg("Invalid seller account")]
    InvalidSeller,
}