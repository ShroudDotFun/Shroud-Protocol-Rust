use anchor_lang::prelude::*;

/// Privacy metadata account
/// Stores zero-knowledge proof verification data
#[account]
#[derive(Default)]
pub struct PrivacyState {
    /// Authority of the privacy state
    pub authority: Pubkey,
    
    /// Merkle root of the commitment tree
    pub merkle_root: [u8; 32],
    
    /// Nullifier set root (prevents double-spends)
    pub nullifier_root: [u8; 32],
    
    /// Total number of private transfers processed
    pub transfer_count: u64,
    
    /// Last update timestamp
    pub last_update: i64,
    
    /// Reserved for future use
    pub _reserved: [u8; 128],
}

impl PrivacyState {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // merkle_root
        32 + // nullifier_root
        8 + // transfer_count
        8 + // last_update
        128; // reserved
}

