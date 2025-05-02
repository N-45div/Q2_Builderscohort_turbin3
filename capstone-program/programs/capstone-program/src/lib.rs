use anchor_lang::prelude::*;

declare_id!("FX1c52jr2jqPsrMSr1mXaYzJYzgAJuMdkxqQKBbPu6km");

#[program]
pub mod capstone_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
