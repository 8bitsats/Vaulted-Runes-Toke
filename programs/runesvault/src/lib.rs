use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

declare_id!("YourTokenProgramID");

#[program]
pub mod alpha_vault_token {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>) -> ProgramResult {
        let mint = &mut ctx.accounts.mint;
        mint.initialize_mint(
            ctx.accounts.authority.key,
            Some(6), // Decimal places
            None,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(init, payer = authority, mint::decimals = 6, mint::authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
