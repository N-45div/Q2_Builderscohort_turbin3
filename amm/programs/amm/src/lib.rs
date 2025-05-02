use anchor_lang::prelude::*;

declare_id!("3vTaTzgXpECzZmL4WVPLaGXHx4gnpoNs71Q1hKUNPiaE");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
