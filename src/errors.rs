use anchor_lang::prelude::*;

#[error_code]
pub enum ShroudError {
    #[msg("Invalid zero-knowledge proof")]
    InvalidProof,
    
    #[msg("Invalid proof length")]
    InvalidProofLength,
    
    #[msg("Invalid public inputs")]
    InvalidPublicInputs,
    
    #[msg("Nullifier has already been spent")]
    NullifierAlreadySpent,
    
    #[msg("Invalid commitment")]
    InvalidCommitment,
    
    #[msg("Transfer amount exceeds balance")]
    InsufficientBalance,
    
    #[msg("Invalid ephemeral account")]
    InvalidEphemeralAccount,
    
    #[msg("Merkle proof verification failed")]
    InvalidMerkleProof,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
}

