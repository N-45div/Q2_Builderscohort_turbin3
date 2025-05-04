use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::{ConstantProduct, LiquidityPair, SwapResult};

use crate::state::Config;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            mint_x.key().to_bytes().as_ref(),
            mint_y.key().to_bytes().as_ref(),
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_ata_y: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct SwapArgs {
    is_x: bool,
    amount: u64,
    min: u64,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self , args: SwapArgs) -> Result<()> {
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None,
        ).unwrap();

        let p = match args.is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, args.amount, args.min).unwrap();

        let res2 = SwapResult {
            deposit: res.deposit.clone(),
            withdraw: res.withdraw.clone(),
            fee: res.fee.clone(),
        };

        self.transfer_to_vault(args.clone(),res)?;

        self.withdraw_from_vault(args, res2)?;

        Ok(())

    }

    fn transfer_to_vault(&mut self, args: SwapArgs, res: SwapResult) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts , mint_decimals) = match args.is_x {
            true => ( TransferChecked {
                from : self.user_ata_x.to_account_info(),
                mint : self.mint_x.to_account_info(),
                to : self.vault_x.to_account_info(),
                authority : self.user.to_account_info(),
            }, self.mint_x.decimals),
            false => ( TransferChecked {
                from : self.user_ata_x.to_account_info(),
                mint : self.mint_x.to_account_info(),
                to : self.vault_x.to_account_info(),
                authority : self.user.to_account_info(),
            }, self.mint_x.decimals),
            };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, res.deposit, mint_decimals)?;
        Ok(())
    }

    fn withdraw_from_vault(&mut self, args: SwapArgs, res: SwapResult) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts , mint_decimals) = match args.is_x {
            true => ( TransferChecked {
                from : self.vault_y.to_account_info(),
                mint : self.mint_y.to_account_info(),
                to : self.user_ata_y.to_account_info(),
                authority : self.user.to_account_info(),
            }, self.mint_y.decimals),
            false => ( TransferChecked {
                from : self.vault_x.to_account_info(),
                mint : self.mint_x.to_account_info(),
                to : self.user_ata_x.to_account_info(),
                authority : self.user.to_account_info(),
            }, self.mint_x.decimals),
            };

        let config_bump = self.config.seed.to_le_bytes();

        let seeds = [
            b"config",
            self.mint_x.to_account_info().key.as_ref(),
            self.mint_y.to_account_info().key.as_ref(),
            config_bump.as_ref(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, res.withdraw, mint_decimals)?;
        Ok(())
    }
}