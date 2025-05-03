use anchor_lang::prelude::*;

declare_id!("3vTaTzgXpECzZmL4WVPLaGXHx4gnpoNs71Q1hKUNPiaE");

pub mod state;
pub mod error;
pub mod contexts;

pub use contexts::*;


#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
