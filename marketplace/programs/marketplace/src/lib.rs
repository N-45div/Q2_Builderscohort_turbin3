use anchor_lang::prelude::*;

declare_id!("FWVP24pdVC4x7Y1PabYKMQYxU8arfK34XSZXazCnbNTh");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
