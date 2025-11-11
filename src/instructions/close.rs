use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, CloseAccount};

#[derive(Accounts)]
pub struct CloseEphemeral<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        close = authority
    )]
    pub ephemeral_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CloseEphemeral>) -> Result<()> {
    // Close ephemeral token account and return rent
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.ephemeral_token_account.to_account_info(),
        destination: ctx.accounts.authority.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::close_account(cpi_ctx)?;
    
    msg!("Ephemeral account closed, rent reclaimed");
    
    Ok(())
}

