use anchor_lang::prelude::*;

declare_id!("SHRouD11111111111111111111111111111111111111");

pub mod instructions;
pub mod state;
pub mod privacy;
pub mod errors;
pub mod utils;

use instructions::*;

#[program]
pub mod shroud_protocol {
    use super::*;

    /// Initialize the Shroud protocol state
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Execute a privacy-preserving token transfer
    pub fn private_transfer(
        ctx: Context<PrivateTransfer>,
        proof: Vec<u8>,
        commitment: [u8; 32],
        nullifier: [u8; 32],
        amount: u64,
    ) -> Result<()> {
        instructions::transfer::handler(ctx, proof, commitment, nullifier, amount)
    }

    /// Verify zero-knowledge proof
    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
    ) -> Result<()> {
        instructions::verify::handler(ctx, proof, public_inputs)
    }

    /// Close ephemeral accounts and reclaim rent
    pub fn close_ephemeral(ctx: Context<CloseEphemeral>) -> Result<()> {
        instructions::close::handler(ctx)
    }
}

