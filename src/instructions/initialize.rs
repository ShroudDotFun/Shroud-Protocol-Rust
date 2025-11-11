use anchor_lang::prelude::*;
use crate::state::PrivacyState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = PrivacyState::LEN,
        seeds = [b"privacy"],
        bump
    )]
    pub privacy_state: Account<'info, PrivacyState>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let privacy_state = &mut ctx.accounts.privacy_state;
    
    privacy_state.authority = ctx.accounts.authority.key();
    privacy_state.merkle_root = [0u8; 32];
    privacy_state.nullifier_root = [0u8; 32];
    privacy_state.transfer_count = 0;
    privacy_state.last_update = Clock::get()?.unix_timestamp;
    
    msg!("Shroud Protocol initialized");
    msg!("Authority: {}", privacy_state.authority);
    
    Ok(())
}

