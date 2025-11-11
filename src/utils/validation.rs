use anchor_lang::prelude::*;

/// Validate transfer amount is within acceptable range
pub fn validate_amount(amount: u64) -> Result<()> {
    require!(amount > 0, ProgramError::InvalidArgument);
    require!(amount < u64::MAX / 2, ProgramError::InvalidArgument);
    Ok(())
}

/// Validate commitment format
pub fn validate_commitment(commitment: &[u8; 32]) -> Result<()> {
    // Check commitment is non-zero
    let is_zero = commitment.iter().all(|&b| b == 0);
    require!(!is_zero, ProgramError::InvalidArgument);
    Ok(())
}

/// Validate nullifier format
pub fn validate_nullifier(nullifier: &[u8; 32]) -> Result<()> {
    // Check nullifier is non-zero
    let is_zero = nullifier.iter().all(|&b| b == 0);
    require!(!is_zero, ProgramError::InvalidArgument);
    Ok(())
}

