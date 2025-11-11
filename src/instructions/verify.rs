use anchor_lang::prelude::*;
use crate::privacy::proofs::verify_zk_proof;
use crate::errors::ShroudError;

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    pub verifier: Signer<'info>,
}

pub fn handler(
    _ctx: Context<VerifyProof>,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<()> {
    // Verify the zero-knowledge proof
    verify_zk_proof(&proof, &public_inputs)
        .map_err(|_| ShroudError::InvalidProof)?;
    
    msg!("Zero-knowledge proof verified successfully");
    
    Ok(())
}

