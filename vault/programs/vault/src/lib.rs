use anchor_lang::prelude::*;

declare_id!("28QkR1tTJzQ3MwQjva72KYN8P7AGELqKBibPYD1X84D4");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
