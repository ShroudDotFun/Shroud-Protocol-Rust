use anchor_lang::prelude::*;

/// Transfer state account
/// Stores metadata for privacy-preserving transfers
#[account]
#[derive(Default)]
pub struct TransferState {
    /// Bump seed for PDA derivation
    pub bump: u8,
    
    /// Commitment to the transfer amount (Pedersen commitment)
    pub amount_commitment: [u8; 32],
    
    /// Nullifier to prevent double-spending
    pub nullifier: [u8; 32],
    
    /// Timestamp of transfer initiation
    pub timestamp: i64,
    
    /// Transfer status
    pub status: TransferStatus,
    
    /// Reserved space for future upgrades
    pub _reserved: [u8; 64],
}

impl TransferState {
    pub const LEN: usize = 8 + // discriminator
        1 + // bump
        32 + // amount_commitment
        32 + // nullifier
        8 + // timestamp
        1 + // status
        64; // reserved
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransferStatus {
    #[default]
    Pending,
    Processing,
    Completed,
    Failed,
}

